use super::bindings::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnedSession {
    session: ViSession,
}

impl Drop for OwnedSession {
    fn drop(&mut self) {
        unsafe {
            viClose(self.session);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BorrowedSession<'a> {
    session: ViSession,
    _phantom: PhantomData<&'a ViSession>,
}

impl BorrowedSession<'_> {
    pub unsafe fn borrow_vi_session(session: ViSession) -> Self {
        Self {
            session,
            _phantom: PhantomData,
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
    fn into_vi_session(&self) -> ViSession;
}

pub trait AsBorrowedSession {
    fn as_borrowed_session(&self) -> BorrowedSession<'_>;
}

impl AsViSession for BorrowedSession<'_> {
    fn as_vi_session(&self) -> ViSession {
        self.session
    }
}

impl AsViSession for OwnedSession {
    fn as_vi_session(&self) -> ViSession {
        self.session
    }
}

impl IntoViSession for OwnedSession {
    fn into_vi_session(&self) -> ViSession {
        let session = self.session;
        std::mem::forget(self);
        session
    }
}

impl FromViSession for OwnedSession {
    unsafe fn from_vi_session(session: ViSession) -> Self {
        Self { session }
    }
}

impl<T: AsBorrowedSession> AsBorrowedSession for &T {
    #[inline]
    fn as_borrowed_session(&self) -> BorrowedSession<'_> {
        T::as_borrowed_session(self)
    }
}

impl<T: AsBorrowedSession> AsBorrowedSession for &mut T {
    #[inline]
    fn as_borrowed_session(&self) -> BorrowedSession<'_> {
        T::as_borrowed_session(self)
    }
}

impl AsBorrowedSession for BorrowedSession<'_> {
    #[inline]
    fn as_borrowed_session(&self) -> BorrowedSession<'_> {
        *self
    }
}

impl AsBorrowedSession for OwnedSession {
    #[inline]
    fn as_borrowed_session(&self) -> BorrowedSession<'_> {
        unsafe { BorrowedSession::borrow_vi_session(self.session) }
    }
}
