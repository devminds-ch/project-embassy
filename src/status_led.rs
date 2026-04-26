use embassy_executor::{SpawnError, Spawner};
use embassy_rp::Peri;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::{Duration, Timer};
use thiserror::Error;

/// Errors that can occur during status LED handling.
#[derive(Error, Debug, defmt::Format)]
pub enum StatusLedError {
    #[error("Status LED task has already been spawned")]
    TaskAlreadySpawned,
    #[error("Failed to spawn status LED task: {0}")]
    TaskSpawnFailed(#[from] SpawnError),
}

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
    pub fn spawn_task(&mut self, spawner: &Spawner) -> Result<(), StatusLedError> {
        let pin = self.pin.take().ok_or(StatusLedError::TaskAlreadySpawned)?;
        let task_token = blink_task(pin, self.delay)?;
        spawner.spawn(task_token);
        Ok(())
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
