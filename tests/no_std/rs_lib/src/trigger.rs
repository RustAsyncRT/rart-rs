use rart_rs::*;
use rart_macros::trigger;
use crate::enable_t2;

pub async fn trigger_task2() -> TaskResult {
    log!("[tg] waiting the task1 to trigger");
    trigger!(enable_t2).wait().await;
    log!("[tg] Task2 is enabled");

    Ok(())
}

pub async fn trigger_task1() -> TaskResult {
    log!("[tg] waiting 3 seconds before enable task2");
    for i in 0..3 {
        delay_secs(1).await;
        log!("[tg] second %d...", i+1);
    }

    trigger!(enable_t2).trigger()
}
