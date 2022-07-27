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

struct Position {
    x: i32,
    y: i32,
}

rart_macros::trigger_pub_def!(enable_t2);
rart_macros::trigger_def!(sample_trigger);

rart_macros::mutex_pub_def!(task_counter, u32);
rart_macros::mutex_def!(sample_mutex, Position);

#[rart_macros::entry]
#[rart_macros::tasks(zbus_ping_pong, trigger_task1, trigger_task2, mutex_task1, mutex_task2)]
#[rart_macros::triggers(enable_t2)]
// #[rart_macros::mutexes(task_counter(0))]
// #[rart_macros::init {
//     sample_trigger = Trigger::new();
//     sample_mutex = Mutex::new(Position {.x: 0, .y: 0});
// }]
async fn main_task() -> TaskResult {
    task_counter.init(Mutex::new(0));

    Ok(())
}
