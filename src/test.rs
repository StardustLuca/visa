use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{AccessMode, Instrument, ResourceManager};

pub struct FakeInstrument3000 {
    pub(crate) inner: Arc<Mutex<Instrument>>,
}

impl FakeInstrument3000 {
    pub fn new(inner: Arc<Mutex<Instrument>>) -> Self {
        Self { inner }
    }

    pub fn instrument_specific_command(&self) {
        let inner = self.inner.lock().unwrap();
    }
}

#[test]
fn test() {
    let mut resource_manager = ResourceManager::new().unwrap();

    let instrument = resource_manager
        .open("ASRL12::INSTR", AccessMode::NO_LOCK, Duration::from_secs(0))
        .unwrap();

    let fake_instrument_3000 = FakeInstrument3000::new(instrument);

    fake_instrument_3000.instrument_specific_command();
}
