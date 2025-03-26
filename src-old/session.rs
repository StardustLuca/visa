use super::{bindings::*, parse_vi_status};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Session {
    session: ViSession,
}

impl Drop for Session {
    fn drop(&mut self) {
        #[cfg(not(feature = "mock"))]
        {
            unsafe {
                let status = viClose(self.session);
                match parse_vi_status(status) {
                    Ok(status) => {
                        tracing::debug!("viClose result: {:?}", status)
                    }
                    Err(error) => {
                        tracing::debug!("viClose result: {}", error)
                    }
                }
            }
        }

        #[cfg(feature = "mock")]
        {
            let count = super::SESSION_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            tracing::debug!("SESSION_COUNT: {}", count);
        }
    }
}

pub trait AsViSession {
    fn as_vi_session(&self) -> ViSession;
}

pub trait FromViSession {
    unsafe fn from_vi_session(session: ViSession) -> Self;
}

pub trait IntoViSession {
    fn into_vi_session(self) -> ViSession;
}

impl AsViSession for Session {
    fn as_vi_session(&self) -> ViSession {
        self.session
    }
}

impl IntoViSession for Session {
    fn into_vi_session(self) -> ViSession {
        let session = self.session;
        drop(self);
        session
    }
}

impl FromViSession for Session {
    unsafe fn from_vi_session(session: ViSession) -> Self {
        Self { session }
    }
}
