use rart_rs::*;

const TASK_NUM: usize = 2;

mutex!(task_counter, u32, 0, TASK_NUM);

pub async fn mutex_task1() -> TaskResult {
    for i in 0..2 {
        {
            let mut counter = task_counter.lock().await;
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
            let mut counter = task_counter.lock().await;
            log!("[mtx] t2 %d old val: %d (tm %d)", i, *counter, timestamp());
            *counter += 1;
            log!("[mtx] t2 %d new val: %d (tm %d)", i, *counter, timestamp());
        }
        delay_secs(2).await;
    }

    Ok(())
}
