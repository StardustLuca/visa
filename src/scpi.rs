use super::{Error, Instrument, Result};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct Identification {
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    pub firmware_version: String,
}

impl FromStr for Identification {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.splitn(4, ',').map(str::trim);

        Ok(Self {
            manufacturer: parts
                .next()
                .ok_or("Missing manufacturer".to_string())?
                .to_owned(),
            model: parts.next().ok_or("Missing model".to_string())?.to_owned(),
            serial_number: parts
                .next()
                .ok_or("Missing serial number".to_string())?
                .to_owned(),
            firmware_version: parts
                .next()
                .ok_or("Missing firmware version".to_string())?
                .to_owned(),
        })
    }
}

// Implement all common SCPI commands
impl Instrument {
    pub fn query_identification(&mut self) -> Result<Identification> {
        let response = self.query(b"*IDN?")?;
        Ok(response
            .parse()
            .map_err(|error| Error::InvalidIdentification(error))?)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.write("*RST")?;
        Ok(())
    }

    pub fn query_self_test(&mut self) -> Result<()> {
        let response = self.query(b"*TST?")?;
        Ok(())
    }

    pub fn operation_complete(&mut self) -> Result<()> {
        self.write("*OPC")?;
        Ok(())
    }

    pub fn query_operation_completed(&mut self) -> Result<()> {
        let response = self.query(b"*OPC?")?;
        Ok(())
    }

    pub fn wait_on_complete(&mut self) -> Result<()> {
        self.write("*WAI")?;
        Ok(())
    }

    pub fn clear_status(&mut self) -> Result<()> {
        self.write("*CLS")?;
        Ok(())
    }

    pub fn event_status_enable(&mut self) -> Result<()> {
        self.write("*ESE")?;
        Ok(())
    }

    pub fn query_event_status_enable(&mut self) -> Result<()> {
        let response = self.query(b"*ESE?")?;
        Ok(())
    }

    pub fn query_event_status_register(&mut self) -> Result<()> {
        let response = self.query(b"*ESR?")?;
        Ok(())
    }

    pub fn service_request_enable(&mut self) -> Result<()> {
        self.write("*SRE")?;
        Ok(())
    }

    pub fn query_service_request_enable(&mut self) -> Result<()> {
        let response = self.query(b"*SRE?")?;
        Ok(())
    }

    pub fn query_read_status_byte(&mut self) -> Result<()> {
        let response = self.query(b"*STB?")?;
        Ok(())
    }
}
