use std::time::Duration;
use rart_rs::*;
use std::time::Instant;

#[derive(Debug)]
pub enum BtnState {
    Press,
    _DoublePress,
}

#[derive(Debug)]
enum LedState {
    On,
    _Off,
}

const TASK_NUM: usize = 2;

channel_pub!(btn_msgq, BtnState, 10, TASK_NUM);
channel!(led_msgq, LedState, 5, TASK_NUM);

async fn task1() -> TaskResult {
    let now = Instant::now();
    let times = [now.elapsed().as_millis() + 10, now.elapsed().as_millis() + 20];

    for _ in 0..1 {
        delay(Duration::from_millis(10)).await;
        assert_eq!(now.elapsed().as_millis(), times[0]);
        delay(Duration::from_millis(10)).await;
        assert_eq!(now.elapsed().as_millis(), times[1]);
        btn_msgq.send(BtnState::Press).await?;
    }

    Ok(())
}

async fn task2() -> TaskResult {
    let now = Instant::now();
    let times = [
        now.elapsed().as_millis() + 20,
        now.elapsed().as_millis() + 23,
        now.elapsed().as_millis() + 26
    ];

    for _ in 0..1 {
        let _led_state = led_msgq.recv().await;
        assert_eq!(now.elapsed().as_millis(), times[0]);
        delay(Duration::from_millis(3)).await;
        assert_eq!(now.elapsed().as_millis(), times[1]);
        delay(Duration::from_millis(3)).await;
        assert_eq!(now.elapsed().as_millis(), times[2]);
    }

    Ok(())
}

#[rart_macros::entry]
#[rart_macros::tasks(task1, task2)]
async fn main_task() -> TaskResult {
    let now = Instant::now();
    let times = [now.elapsed().as_millis() + 20];

    for _ in 0..1 {
        let _btn_state = btn_msgq.recv().await;
        assert_eq!(now.elapsed().as_millis(), times[0]);

        led_msgq.send(LedState::On).await?;
    }

    Ok(())
}

#[test]
fn basic() {
    main_task();
}
