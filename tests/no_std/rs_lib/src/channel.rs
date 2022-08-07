use rart_rs::*;

const TASK_NUM: usize = 2;

struct Pkt {
    produce_timestamp: u32,
}

channel!(chan, Pkt, 5, TASK_NUM);


pub async fn producer_task() -> TaskResult {
    delay_secs(1).await;
    log!("[chan] producer %d", timestamp());
    delay_secs(1).await;
    log!("[chan] producer %d", timestamp());
    chan.send(Pkt { produce_timestamp: unsafe { timestamp() } }).await
}

pub async fn consumer_task() -> TaskResult {
    log!("[chan] receiver %d", timestamp());
    let pkt = chan.recv().await?;
    log!("[chan] receiver %d", timestamp());
    log!("[chan] pkt produce timestamp: %d", pkt.produce_timestamp);

    Ok(())
}
