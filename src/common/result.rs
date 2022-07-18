#[cfg(not(feature = "std"))]
use const_format::formatcp;
#[cfg(not(feature = "std"))]
use crate::mc_panic;
#[cfg(not(feature = "std"))]
use crate::no_std::panic;
#[cfg(feature = "std")]
use std::sync::PoisonError;
#[cfg(feature = "std")]
use std::sync::mpsc::SendError;
#[cfg(feature = "std")]
use std::sync::mpsc::RecvError;

pub enum MCError {
    Generic,
    MutexPoisonError,
    SendError,
    RecvError,
    LazyNotInit,
    Trigger,
    Semaphore,
    Timer,
    WrongGPIOPort,
    WrongGPIOPin,
}

pub trait Expect<T> {
    fn mc_expect(self, msg: &str) -> T;
}

impl<T, E> Expect<T> for Result<T, E> {
    fn mc_expect(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            #[cfg(feature = "std")]
            Err(_e) => mc_panic!("{}", msg),
            #[cfg(not(feature = "std"))]
            Err(_e) => mc_panic!("%s", msg)
        }
    }
}

impl<T> Expect<T> for Option<T> {
    fn mc_expect(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            #[cfg(feature = "std")]
            None => mc_panic!("{}", msg),
            #[cfg(not(feature = "std"))]
            None => mc_panic!("%s", msg)
        }
    }
}

#[cfg(feature = "std")]
impl<T> From<PoisonError<T>> for MCError {
    fn from(_: PoisonError<T>) -> Self {
        MCError::MutexPoisonError
    }
}

#[cfg(feature = "std")]
impl<T> From<SendError<T>> for MCError {
    fn from(_: SendError<T>) -> Self {
        MCError::SendError
    }
}

#[cfg(feature = "std")]
impl From<RecvError> for MCError {
    fn from(_: RecvError) -> Self {
        MCError::RecvError
    }
}

#[cfg(not(feature = "std"))]
impl From<i32> for MCError {
    fn from(_: i32) -> Self {
        MCError::Generic
    }
}
