use rart_rs::*;
use chrono::Local;
use rart::channel;

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

rart::mutex_def!(sample_mutex, LedState);
rart::semaphore_def!(sample_semaphore, 2);
rart::trigger_def!(sample_trigger);

rart::channel_pub_def!(btn_msgq, BtnState, 10);
rart::channel_def!(led_msgq, LedState, 5);

async fn task1() -> TaskResult {
    let now = Local::now().timestamp();
    let times = [now + 10, now + 20];

    for _ in [0; 1] {
        delay_secs(10).await;
        assert_eq!(Local::now().timestamp(), times[0]);
        delay_secs(10).await;
        assert_eq!(Local::now().timestamp(), times[1]);
        channel!(btn_msgq).send(BtnState::Press).await?;
    }

    Ok(())
}

async fn task2() -> TaskResult {
    let now = Local::now().timestamp();
    let times = [now + 20, now + 23, now + 26];

    for _ in [0; 1] {
        let _led_state = channel!(led_msgq).recv().await;
        assert_eq!(Local::now().timestamp(), times[0]);
        delay_secs(3).await;
        assert_eq!(Local::now().timestamp(), times[1]);
        delay_secs(3).await;
        assert_eq!(Local::now().timestamp(), times[2]);
    }

    Ok(())
}

#[rart::entry]
#[rart::tasks(task1, task2)]
#[rart::channels(btn_msgq, led_msgq)]
// #[rart::triggers(sample_trigger)]
// #[rart::init{
//     sample_mutex = Mutex::new(LedState::On);
//     sample_semaphore = Semaphore::new(2);
// }]
async fn main_task() -> TaskResult {
    let now = Local::now().timestamp();
    let times = [now + 20];

    for _ in [0; 1] {
        let _btn_state = channel!(btn_msgq).recv().await;
        assert_eq!(Local::now().timestamp(), times[0]);

        channel!(led_msgq).send(LedState::On).await?;
    }

    Ok(())
}

#[test]
fn basic() {
    main_task();
}
