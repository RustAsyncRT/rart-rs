use rart_macros::mutex;
use rart_rs::*;
use crate::task_counter;

pub async fn mutex_task1() -> TaskResult {
    for i in 0..2 {
        {
            let mut counter = mutex!(task_counter).lock().await;
            log!("[mtx] t1 %d old val: %d (tm %d)", i, *counter, timestamp());
            *counter += 1;
            log!("[mtx] t1 %d new val: %d (tm %d)", i, *counter, timestamp());
        }
        delay_secs(2).await;
    }

    Ok(())
}

pub async fn mutex_task2() -> TaskResult {
    for i in 0..2 {
        {
            let mut counter = mutex!(task_counter).lock().await;
            log!("[mtx] t2 %d old val: %d (tm %d)", i, *counter, timestamp());
            *counter += 1;
            log!("[mtx] t2 %d new val: %d (tm %d)", i, *counter, timestamp());
        }
        delay_secs(2).await;
    }

    Ok(())
}
