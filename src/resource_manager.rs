use super::{
    bindings::*,
    error::{Error, Result, parse_vi_status},
    instrument::Instrument,
    session::Session,
};
use bitflags::bitflags;
use regex::Regex;
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    Global,
    Local,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccessMode: ViAccessMode {
        const NO_LOCK = VI_NO_LOCK;
        const EXCLUSIVE_LOCK = VI_EXCLUSIVE_LOCK;
        const SHARED_LOCK = VI_SHARED_LOCK;
        const LOAD_CONFIG = VI_LOAD_CONFIG;
    }
}

#[derive(Debug)]
pub struct ResourceManager {
    inner: Session,
    pub(crate) instruments: HashMap<String, Arc<Mutex<Instrument>>>,
}

impl ResourceManager {
    pub fn new() -> Result<Self> {
        let mut session: ViSession = 0;
        unsafe {
            let status = viOpenDefaultRM(&mut session as _);
            parse_vi_status(status)?;
            Ok(Self {
                inner: Session::from_vi_session(session),
                instruments: HashMap::new(),
            })
        }
    }

    pub fn from_vi_session(session: ViSession) -> Self {
        Self {
            inner: Session::from_vi_session(session),
            instruments: HashMap::new(),
        }
    }

    pub fn as_vi_session(&self) -> ViSession {
        self.inner.as_vi_session()
    }

    pub fn open(
        &mut self,
        resource: &str,
        access_mode: AccessMode,
        timeout: Duration,
    ) -> Result<Arc<Mutex<Instrument>>> {
        match self.instruments.get(resource) {
            Some(instrument) => {
                let identification = {
                    let mut instrument = instrument.lock().unwrap();
                    instrument.query_identification()
                };

                match identification {
                    Ok(_) => Ok(instrument.clone()),
                    Err(_) => {
                        self.instruments.remove(resource);
                        self.open(resource, access_mode, timeout)
                    }
                }
            }
            None => {
                let c_resource = CString::from_str(resource).map_err(|_| Error::InvalidString)?;
                let mut session: ViSession = 0;

                unsafe {
                    let status = viOpen(
                        self.as_vi_session(),
                        c_resource.as_ptr(),
                        access_mode.bits(),
                        timeout.as_millis() as _,
                        &mut session as _,
                    );
                    parse_vi_status(status)?;
                }

                let instrument = Arc::new(Mutex::new(Instrument::new(Session::from_vi_session(
                    session,
                ))?));

                self.instruments
                    .insert(resource.to_owned(), instrument.clone());

                Ok(instrument)
            }
        }
    }

    pub fn close(&mut self, resource: &str) -> Result<()> {
        let instrument = self.instruments.remove(resource);

        match instrument {
            Some(instrument) => {
                let instrument = instrument.lock().unwrap();
                unsafe {
                    let status = viClose(instrument.as_vi_session());
                    parse_vi_status(status)?;
                    Ok(())
                }
            }
            None => Err(Error::InstrumentNotFound),
        }
    }

    pub fn get_resources_with_expression(&self, expression: &str) -> Result<Vec<String>> {
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

    pub fn get_resources_with_scope(&self, scope: Scope) -> Result<Vec<String>> {
        match scope {
            Scope::Global => self.get_resources_with_expression("?*INSTR"),
            Scope::Local => {
                self.get_resources_with_expression("(USB|GPIB|VXI|ASRL|TCPIP?::169.254)?*INSTR")
            }
        }
    }

    pub fn open_with_identification(
        &mut self,
        manufacturer: &str,
        model: &str,
        serial_number: &str,
        access_mode: AccessMode,
        scope: Scope,
        timeout: Duration,
    ) -> Result<Arc<Mutex<Instrument>>> {
        let resources = self.get_resources_with_scope(scope)?;

        for resource in resources {
            let instrument = self.open(&resource, access_mode, timeout);

            let instrument = match instrument {
                Ok(instrument) => instrument,
                Err(_) => continue,
            };

            let identification = match instrument.lock().unwrap().query_identification() {
                Ok(identification) => identification,
                Err(_) => continue,
            };

            let manufacturer = Regex::new(manufacturer).map_err(|_| Error::InvalidString)?;
            let model = Regex::new(model).map_err(|_| Error::InvalidString)?;
            let serial_number = Regex::new(serial_number).map_err(|_| Error::InvalidString)?;

            if manufacturer.is_match(&identification.manufacturer)
                && model.is_match(&identification.model)
                && serial_number.is_match(&identification.serial_number)
            {
                return Ok(instrument);
            }
        }

        Err(Error::InstrumentNotFound)
    }
}
