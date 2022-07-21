use crate::mc_panic;
#[cfg(not(feature = "std"))]
use const_format::formatcp;
#[cfg(not(feature = "std"))]
use crate::no_std::panic;
#[cfg(feature = "std")]
use std::sync::PoisonError;
#[cfg(feature = "std")]
use std::sync::mpsc::SendError;
#[cfg(feature = "std")]
use std::sync::mpsc::RecvError;

pub enum RARTError {
    Generic,
    MutexPoisonError,
    SendError,
    RecvError,
    LazyNotInit,
    Trigger,
    Semaphore,
    Timer,
    CError(i32),
    WrongGPIOPort,
    WrongGPIOPin,
}

pub trait Expect<T> {
    fn rart_expect(self, msg: &str) -> T;
}

impl<T, E> Expect<T> for Result<T, E> {
    fn rart_expect(self, msg: &str) -> T {
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
    fn rart_expect(self, msg: &str) -> T {
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
impl<T> From<PoisonError<T>> for RARTError {
    fn from(_: PoisonError<T>) -> Self {
        RARTError::MutexPoisonError
    }
}

#[cfg(feature = "std")]
impl<T> From<SendError<T>> for RARTError {
    fn from(_: SendError<T>) -> Self {
        RARTError::SendError
    }
}

#[cfg(feature = "std")]
impl From<RecvError> for RARTError {
    fn from(_: RecvError) -> Self {
        RARTError::RecvError
    }
}

#[cfg(not(feature = "std"))]
impl From<i32> for RARTError {
    fn from(_: i32) -> Self {
        RARTError::Generic
    }
}
