#![feature(core_ffi_c)]
#![no_std]
#![no_builtins]

extern crate panic_halt;
extern crate rart_rs;
extern crate rart_macros;
extern crate alloc;

use rart_rs::*;

#[derive(Clone)]
pub enum ZbusChannelIndex {
    Ping,
    Pong,
}

impl Into<u32> for ZbusChannelIndex {
    fn into(self) -> u32 {
        match self {
            ZbusChannelIndex::Ping => 0,
            ZbusChannelIndex::Pong => 1,
        }
    }
}

#[no_mangle]
static ping_listener: zbus_observer = zbus_observer::new();

#[repr(C)]
#[derive(Clone)]
struct BallPos {
    x: u32,
    y: u32,
}

#[rart_macros::entry]
async fn ping_task() -> TaskResult {
    let ping_chan = ZbusChannel::new(ZbusChannelIndex::Ping.into());
    let pong_chan = ZbusChannel::new(ZbusChannelIndex::Pong.into());

    for i in 0..3 {
        log!("[rs|%d] publishing at pong channel...", i);
        pong_chan.publish(BallPos { x: 1, y: 2 }).await;
        log!("[rs|%d] published. Waiting data in ping channel...", i);
        let recv_msg: BallPos = ping_chan.read().await;
        log!("[rs|%d] receive ball pos: <%d, %d>", i, recv_msg.x, recv_msg.y);
    }

    Ok(())
}
