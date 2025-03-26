use super::{
    AsViSession, Error, FromViSession, Instrument, Result, Session, bindings::*, parse_vi_status,
    scpi::Identification,
};
use bitflags::bitflags;
use regex::Regex;
use std::{
    ffi::{CStr, CString},
    str::FromStr,
    time::Duration,
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccessMode: ViAccessMode {
        const NO_LOCK = VI_NO_LOCK;
        const EXCLUSIVE_LOCK = VI_EXCLUSIVE_LOCK;
        const SHARED_LOCK = VI_SHARED_LOCK;
        const LOAD_CONFIG = VI_LOAD_CONFIG;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ResourceManager(Session);

impl ResourceManager {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "mock")]
        {
            super::SESSION_COUNT.store(0, std::sync::atomic::Ordering::Relaxed);
        }

        let mut session: ViSession = 0;
        unsafe {
            let status = viOpenDefaultRM(&mut session as _);
            parse_vi_status(status)?;
            Ok(Self(Session::from_vi_session(session)))
        }
    }
}

impl AsViSession for ResourceManager {
    fn as_vi_session(&self) -> ViSession {
        self.0.as_vi_session()
    }
}

impl AsResourceManager for ResourceManager {}

impl FromViSession for ResourceManager {
    unsafe fn from_vi_session(session: ViSession) -> Self {
        unsafe { Self(FromViSession::from_vi_session(session)) }
    }
}

pub trait AsResourceManager: AsViSession {
    fn get_resources_with_expression(&self, expression: &str) -> Result<Vec<String>> {
        let mut list: ViFindList = 0;
        let mut count: ViUInt32 = 0;
        let mut instrument_description = [0; VI_FIND_BUFLEN as _];
        let expression = CString::from_str(expression).map_err(|_| Error::InvalidString)?;
        unsafe {
            let status = viFindRsrc(
                self.as_vi_session(),
                expression.as_ptr(),
                &mut list,
                &mut count,
                instrument_description.as_mut_ptr() as _,
            );
            parse_vi_status(status)?;
        }

        let mut resources = vec![];
        if count > 0 {
            let resource = CStr::from_bytes_until_nul(&instrument_description)
                .map_err(|_| Error::InvalidString)?
                .to_string_lossy()
                .into_owned();

            resources.push(resource);

            for _ in 1..count {
                unsafe {
                    let status = viFindNext(list, instrument_description.as_mut_ptr() as _);
                    parse_vi_status(status)?;
                }

                let resource = CStr::from_bytes_until_nul(&instrument_description)
                    .map_err(|_| Error::InvalidString)?
                    .to_string_lossy()
                    .into_owned();

                resources.push(resource);
            }
        }
        Ok(resources)
    }

    fn get_resources_with_scope(&self, scope: Scope) -> Result<Vec<String>> {
        match scope {
            Scope::Global => self.get_resources_with_expression("?*INSTR"),
            Scope::Local => {
                self.get_resources_with_expression("(USB|GPIB|VXI|ASRL|TCPIP?::169.254)?*INSTR")
            }
        }
    }

    fn get_resources_and_identification_with_scope(
        &self,
        scope: Scope,
    ) -> Result<Vec<(String, Identification)>> {
        let resources = self.get_resources_with_scope(scope)?;

        let collection = resources
            .into_iter()
            .filter_map(|resource| {
                self.open(&resource, AccessMode::NO_LOCK, Duration::from_secs(0))
                    .ok()
                    .and_then(|mut instrument| {
                        instrument
                            .query_identification()
                            .map(|identification| (resource, identification))
                            .ok()
                    })
            })
            .collect();

        Ok(collection)
    }

    fn open(
        &self,
        resource: &str,
        access_mode: AccessMode,
        timeout: Duration,
    ) -> Result<Instrument> {
        let resource = CString::from_str(resource).map_err(|_| Error::InvalidString)?;
        let mut instrument: ViSession = 0;
        #[cfg(not(feature = "mock"))]
        {
            unsafe {
                let status = viOpen(
                    self.as_vi_session(),
                    resource.as_ptr(),
                    access_mode.bits(),
                    timeout.as_millis() as _,
                    &mut instrument as _,
                );
                parse_vi_status(status)?;
            }
        }

        #[cfg(feature = "mock")]
        {
            let count = super::SESSION_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            tracing::debug!("SESSION_COUNT: {}", count);
        }

        Ok(unsafe { Instrument::from_vi_session(instrument) })
    }

    fn open_with_resource(
        &self,
        resource: &str,
        access_mode: AccessMode,
        scope: Scope,
        timeout: Duration,
    ) -> Result<Instrument> {
        let matching_resource = resource;
        let resources = self.get_resources_with_scope(scope)?;

        for resource in resources {
            if matching_resource == resource {
                let instrument = self.open(&resource, access_mode, timeout)?;

                return Ok(instrument);
            }
        }

        Err(Error::InstrumentNotFound)
    }

    fn open_with_identification(
        &self,
        manufacturer: &str,
        model: &str,
        serial_number: &str,
        access_mode: AccessMode,
        scope: Scope,
        timeout: Duration,
    ) -> Result<Instrument> {
        let resources = self.get_resources_with_scope(scope)?;

        for resource in resources {
            let instrument = self.open(&resource, access_mode, timeout);

            let mut instrument = match instrument {
                Ok(instrument) => instrument,
                Err(_) => continue,
            };

            let identification = match instrument.query_identification() {
                Ok(identification) => identification,
                Err(_) => continue,
            };

            let manufacturer = Regex::new(manufacturer).map_err(|_| Error::InvalidString)?;
            let model = Regex::new(model).map_err(|_| Error::InvalidString)?;
            let serial_number = Regex::new(serial_number).map_err(|_| Error::InvalidString)?;

            #[cfg(not(feature = "mock"))]
            {
                if manufacturer.is_match(&identification.manufacturer)
                    && model.is_match(&identification.model)
                    && serial_number.is_match(&identification.serial_number)
                {
                    return Ok(instrument);
                }
            }

            #[cfg(feature = "mock")]
            {
                return Ok(instrument);
            }
        }

        Err(Error::InstrumentNotFound)
    }

    fn close_all(&self) {
        std::mem::drop(unsafe { ResourceManager::from_vi_session(self.as_vi_session()) });
    }
}
