use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::{Duration, Timer};

/// Spawns and owns the status LED heartbeat task.
pub struct StatusLedController {
    pin: Option<Peri<'static, AnyPin>>,
    delay: Duration,
}

impl StatusLedController {
    /// Creates a status LED controller with board pin and blink interval.
    pub const fn new(pin: Peri<'static, AnyPin>, delay: Duration) -> Self {
        Self {
            pin: Some(pin),
            delay,
        }
    }

    /// Spawns a task that toggles the LED at the configured interval.
    pub fn spawn_task(&mut self, spawner: &Spawner) {
        let pin = self.pin.take().expect("status LED task already spawned");
        unwrap!(spawner.spawn(blink_task(pin, self.delay)));
    }
}

#[embassy_executor::task]
async fn blink_task(pin: Peri<'static, AnyPin>, delay: Duration) {
    let mut led = Output::new(pin, Level::Low);

    loop {
        led.set_high();
        Timer::after(delay).await;
        led.set_low();
        Timer::after(delay).await;
    }
}
