use rart_rs::*;

const TASK_NUM: usize = 2;

trigger!(enable_t2, TASK_NUM);

pub async fn trigger_task1() -> TaskResult {
    log!("[tg] waiting 3 seconds before enable task2");
    for i in 0..3 {
        delay_secs(1).await;
        log!("[tg] second %d...", i+1);
    }

    enable_t2.trigger()
}

pub async fn trigger_task2() -> TaskResult {
    log!("[tg] waiting the task1 to trigger");
    enable_t2.wait().await;
    log!("[tg] Task2 is enabled");

    Ok(())
}
