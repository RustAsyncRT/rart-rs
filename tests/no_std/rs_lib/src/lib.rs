#![feature(core_ffi_c)]
#![no_std]
#![no_builtins]

extern crate panic_halt;
extern crate rart_rs;
extern crate rart_macros;
extern crate alloc;

use rart_rs::*;

mod zbus;
mod trigger;
mod mutex;

use crate::zbus::zbus_ping_pong;
use crate::trigger::{trigger_task1, trigger_task2};
use crate::mutex::{mutex_task1, mutex_task2};

#[rart_macros::entry]
#[rart_macros::tasks(zbus_ping_pong, trigger_task1, trigger_task2, mutex_task1, mutex_task2)]
async fn main_task() -> TaskResult {
    Ok(())
}
