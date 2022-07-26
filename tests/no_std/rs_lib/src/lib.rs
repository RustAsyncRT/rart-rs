#![feature(core_ffi_c)]
#![no_std]
#![no_builtins]

extern crate panic_halt;
extern crate rart_rs;
extern crate rart_macros;
extern crate alloc;

use rart_rs::*;

mod zbus;

use crate::zbus::zbus_ping_pong;

#[rart_macros::entry]
#[rart_macros::tasks(zbus_ping_pong)]
async fn main_task() -> TaskResult {
    Ok(())
}
