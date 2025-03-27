use super::{
    Identification,
    bindings::*,
    error::{Error, Result, VisaError, parse_vi_status, parse_vi_status_to_io},
    resource_manager::AccessMode,
    session::Session,
};
use bitflags::bitflags;
use std::{
    ffi::{CStr, CString},
    io::{BufRead, BufReader, Write},
    ops::Deref,
    str::FromStr,
    time::Duration,
};

bitflags! {
    pub struct FlushMode: ViUInt16 {
        const READ_BUF = VI_READ_BUF as _;
        const READ_BUF_DISCARD = VI_READ_BUF_DISCARD as _;
        const WRITE_BUF = VI_WRITE_BUF as _;
        const WRITE_BUF_DISCARD  = VI_WRITE_BUF_DISCARD as _;
        const IO_IN_BUF = VI_IO_IN_BUF as _;
        const IO_IN_BUF_DISCARD = VI_IO_IN_BUF_DISCARD as _;
        const IO_OUT_BUF = VI_IO_OUT_BUF as _;
        const IO_OUT_BUF_DISCARD = VI_IO_OUT_BUF_DISCARD as _;
    }
}

#[derive(Debug)]
pub struct Instrument {
    inner: Session,
    pub identification: Identification,
}

impl Deref for Instrument {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::io::Write for Instrument {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        <&Instrument>::write(&mut &*self, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        <&Instrument>::flush(&mut &*self)
    }
}

impl std::io::Read for Instrument {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        <&Instrument>::read(&mut &*self, buf)
    }
}

impl std::io::Write for &Instrument {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut ret_cnt: ViUInt32 = 0;
        unsafe {
            let status = viWrite(
                self.as_vi_session(),
                buf.as_ptr(),
                buf.len() as _,
                &mut ret_cnt as _,
            );
            parse_vi_status_to_io(status)?;
        }

        Ok(ret_cnt as _)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unsafe {
            let status = viFlush(self.as_vi_session(), FlushMode::IO_OUT_BUF.bits());
            parse_vi_status_to_io(status)?;
        }
        Ok(())
    }
}

impl std::io::Read for &Instrument {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut ret_cnt: ViUInt32 = 0;
        unsafe {
            let status = viRead(
                self.as_vi_session(),
                buf.as_mut_ptr(),
                buf.len() as _,
                &mut ret_cnt as _,
            );
            parse_vi_status_to_io(status)?;
        }
        Ok(ret_cnt as _)
    }
}

impl Instrument {
    pub fn new(session: Session) -> Result<Self> {
        let mut instrument = Self {
            inner: session,
            identification: Identification {
                manufacturer: "".into(),
                model: "".into(),
                serial_number: "".into(),
                firmware_version: "".into(),
            },
        };
        let identification = instrument.query_identification()?;
        instrument.identification = identification;
        Ok(instrument)
    }

    pub fn as_vi_session(&self) -> ViSession {
        self.inner.as_vi_session()
    }

    pub fn write(&mut self, buf: impl AsRef<[u8]>) -> Result<()> {
        self.write_all(buf.as_ref())?;
        Ok(())
    }

    pub fn read(&self) -> Result<String> {
        let mut reader = BufReader::new(self);
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        Ok(buf)
    }

    pub fn query(&mut self, buf: impl AsRef<[u8]>) -> Result<String> {
        self.write(buf)?;
        let response = self.read()?;
        Ok(response)
    }

    pub fn status_description(&self, error: VisaError) -> Result<()> {
        let mut buf = String::new();
        unsafe {
            let status = viStatusDesc(self.as_vi_session(), error as _, buf.as_mut_ptr() as _);
            parse_vi_status(status)?;
        }
        Ok(())
    }

    pub fn lock(
        &self,
        mode: AccessMode,
        timeout: Duration,
        key: Option<&str>,
    ) -> Result<Option<String>> {
        match (mode & AccessMode::SHARED_LOCK).is_empty() {
            true => unsafe {
                let status = viLock(
                    self.as_vi_session(),
                    mode.bits(),
                    timeout.as_millis() as _,
                    VI_NULL as _,
                    VI_NULL as _,
                );
                parse_vi_status(status)?;
                Ok(None)
            },
            false => {
                let key = match key {
                    Some(key) => {
                        let key = CString::from_str(key).map_err(|_| Error::InvalidString)?;
                        Some(key)
                    }
                    None => None,
                };

                let mut access_key = [0; VI_FIND_BUFLEN as _];
                unsafe {
                    let status = viLock(
                        self.as_vi_session(),
                        mode.bits(),
                        timeout.as_millis() as _,
                        key.map(|key| key.as_ptr()).unwrap_or(VI_NULL as _),
                        access_key.as_mut_ptr() as _,
                    );
                    parse_vi_status(status)?;
                }

                let access_key = CStr::from_bytes_with_nul(&access_key)
                    .map_err(|_| Error::InvalidString)?
                    .to_str()
                    .map_err(|_| Error::InvalidString)?;
                Ok(Some(access_key.to_owned()))
            }
        }
    }

    pub fn lock_exclusive(&self, timeout: Duration) -> Result<()> {
        unsafe {
            let status = viLock(
                self.as_vi_session(),
                AccessMode::EXCLUSIVE_LOCK.bits(),
                timeout.as_millis() as _,
                VI_NULL as _,
                VI_NULL as _,
            );
            parse_vi_status(status)?;
        }
        Ok(())
    }

    pub fn lock_shared(&self, timeout: Duration) -> Result<String> {
        let mut access_key = [0; VI_FIND_BUFLEN as _];
        unsafe {
            let status = viLock(
                self.as_vi_session(),
                AccessMode::EXCLUSIVE_LOCK.bits(),
                timeout.as_millis() as _,
                VI_NULL as _,
                access_key.as_mut_ptr() as _,
            );
            parse_vi_status(status)?;
        }
        let access_key = CStr::from_bytes_with_nul(&access_key)
            .map_err(|_| Error::InvalidString)?
            .to_str()
            .map_err(|_| Error::InvalidString)?;
        Ok(access_key.to_owned())
    }

    pub fn lock_shared_with_key(&self, timeout: Duration, key: &str) -> Result<String> {
        let key = CString::from_str(key).map_err(|_| Error::InvalidString)?;

        let mut access_key = [0; VI_FIND_BUFLEN as _];
        unsafe {
            let status = viLock(
                self.as_vi_session(),
                AccessMode::EXCLUSIVE_LOCK.bits(),
                timeout.as_millis() as _,
                key.as_ptr(),
                access_key.as_mut_ptr() as _,
            );
            parse_vi_status(status)?;
        }
        let access_key = CStr::from_bytes_with_nul(&access_key)
            .map_err(|_| Error::InvalidString)?
            .to_str()
            .map_err(|_| Error::InvalidString)?;
        Ok(access_key.to_owned())
    }

    pub fn unlock(&self) -> Result<()> {
        unsafe {
            let status = viUnlock(self.as_vi_session());
            parse_vi_status(status)?;
        }
        Ok(())
    }
}
