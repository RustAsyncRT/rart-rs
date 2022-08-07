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
mod channel;
mod semaphore;

use crate::zbus::zbus_ping_pong;
use crate::trigger::{trigger_task1, trigger_task2};
use crate::mutex::{mutex_task1, mutex_task2};
use crate::channel::{producer_task, consumer_task};
use crate::semaphore::{sem_task1, sem_task2, sem_task3};

#[rart_macros::entry]
#[rart_macros::tasks(zbus_ping_pong, trigger_task1, trigger_task2, mutex_task1, mutex_task2,
                    producer_task, consumer_task, sem_task1, sem_task2, sem_task3)]
async fn main_task() -> TaskResult {
    Ok(())
}
