use super::{bindings::*, parse_vi_status};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Session {
    inner: ViSession,
}

impl Session {
    pub fn from_vi_session(session: ViSession) -> Self {
        Self { inner: session }
    }

    pub fn as_vi_session(&self) -> ViSession {
        self.inner
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            let status = viClose(self.as_vi_session());
            let _ = parse_vi_status(status);
        }
    }
}
