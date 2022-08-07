use rart_rs::*;

const TASK_NUM: usize = 2;

semaphore!(resource, 1, 1, TASK_NUM);

pub async fn sem_task1() -> TaskResult {
    log!("[sem] task1 trying to take the resource at %d ...", timestamp());
    resource.take().await;
    log!("[sem] task1 took the resource and exit at %d", timestamp());

    Ok(())
}

pub async fn sem_task2() -> TaskResult {
    log!("[sem] task2 trying to take the resource at %d ...", timestamp());
    resource.take().await;
    log!("[sem] task2 took the resource at %d", timestamp());
    delay_secs(1).await;
    log!("[sem] task2 is giving the resource at %d ...", timestamp());
    resource.give()?;
    log!("[sem] task2 give the resource at %d", timestamp());


    Ok(())
}

pub async fn sem_task3() -> TaskResult {
    log!("[sem] task3 will wait 2 seconds to give the resource. Now is %d", timestamp());
    delay_secs(2).await;
    log!("[sem] task3 giving the resource at %d ...", timestamp());
    resource.give()?;

    Ok(())
}
