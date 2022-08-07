use std::time::Duration;
use chrono::Local;
use rart_rs::*;

#[rart::entry]
async fn blink_task() -> TaskResult
{
    log!("Blink task");
    let now = Local::now().timestamp();

    let mut gpio2 = Gpio::new(1, 2).await?;
    assert_eq!(read_gpio(&gpio2), false);
    gpio2.toggle();
    assert_eq!(read_gpio(&gpio2), true);
    assert_eq!(Local::now().timestamp(), now);
    delay(Duration::from_secs(1)).await;
    assert_eq!(Local::now().timestamp(), now + 1);
    gpio2.toggle();
    assert_eq!(read_gpio(&gpio2), false);
    delay(Duration::from_secs(1)).await;
    assert_eq!(Local::now().timestamp(), now + 2);

    Ok(())
}

#[test]
fn blink() {
    blink_task()
}
