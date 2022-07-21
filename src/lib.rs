#![cfg_attr(not(feature = "std"), no_std)]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
#![feature(once_cell)]

extern crate core;

mod futures;
mod common;
#[cfg(feature = "peripherals")]
mod peripherals;
#[cfg(feature = "std")]
mod std;
#[cfg(not(feature = "std"))]
mod no_std;

pub use common::task::{Task, TaskResult};
pub use common::executor::Executor;
pub use common::lazy::Lazy;
pub use common::timer::timer_init;

pub use futures::channel::Channel;
pub use futures::time::delay_secs;
pub use futures::mutex::Mutex;
pub use futures::trigger::Trigger;
pub use futures::semaphore::{Semaphore, SemaphoreUnbounded};

#[cfg(feature = "zbus")]
pub use futures::zbus::ZbusChannel;
#[cfg(all(not(feature = "std"), feature = "zbus"))]
pub use no_std::zbus_backend::zbus_observer;

pub use common::result::{Expect, RARTError};
#[cfg(not(feature = "std"))]
pub use no_std::{log_fn, timestamp};
#[cfg(not(feature = "std"))]
pub use const_format::formatcp;

#[cfg(all(feature = "std", feature = "peripherals"))]
pub use peripherals::{Peripheral, gpio::Gpio};
#[cfg(all(feature = "std", feature = "peripherals"))]
pub use crate::std::peripheral::read_gpio;
