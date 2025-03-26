use super::{bindings::*, parse_vi_status};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Session {
    pub(crate) inner: ViSession,
}

// impl Drop for Session {
//     fn drop(&mut self) {
//         unsafe {
//             let status = viClose(self.as_vi_session());
//             parse_vi_status(status).unwrap();
//         }
//     }
// }

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
        self.inner
    }
}

impl IntoViSession for Session {
    fn into_vi_session(self) -> ViSession {
        let session = self.inner;
        drop(self);
        session
    }
}

impl FromViSession for Session {
    unsafe fn from_vi_session(session: ViSession) -> Self {
        Self { inner: session }
    }
}
