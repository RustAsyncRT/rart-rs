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
pub struct BallPos {
    x: u32,
    y: u32,
}

pub async fn zbus_ping_pong() -> TaskResult {
    let ping_chan = ZbusChannel::new(ZbusChannelIndex::Ping.into());
    let pong_chan = ZbusChannel::new(ZbusChannelIndex::Pong.into());

    for i in 0..3 {
        log!("[zbus] %d. publishing at pong channel...", i);
        pong_chan.publish(BallPos { x: 1, y: 2 }).await;
        log!("[zbus] %d. published. Waiting data in ping channel...", i);
        let recv_msg: BallPos = ping_chan.read().await;
        log!("[zbus] %d. receive ball pos: <%d, %d>", i, recv_msg.x, recv_msg.y);
    }

    Ok(())
}
