use core::time::Duration;
use rart_rs::*;

const TASK_NUM: usize = 2;

struct Pkt {
    produce_timestamp: u32,
}

channel!(chan, Pkt, 5, TASK_NUM);


pub async fn producer_task() -> TaskResult {
    delay(Duration::from_millis(1)).await;
    log!("[chan] producer %d", timestamp_millis());
    delay(Duration::from_millis(1)).await;
    log!("[chan] producer %d", timestamp_millis());
    chan.send(Pkt { produce_timestamp: unsafe { timestamp_millis() } }).await
}

pub async fn consumer_task() -> TaskResult {
    log!("[chan] receiver %d", timestamp_millis());
    let pkt = chan.recv().await?;
    log!("[chan] receiver %d", timestamp_millis());
    log!("[chan] pkt produce timestamp: %d", pkt.produce_timestamp);

    Ok(())
}
