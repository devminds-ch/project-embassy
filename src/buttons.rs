use defmt::warn;
use embassy_executor::{SpawnError, Spawner};
use embassy_rp::Peri;
use embassy_rp::gpio::{AnyPin, Input, Pull};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Duration;
use embassy_time::Timer;
use thiserror::Error;

/// Number of physical user buttons on the board.
pub const BUTTON_COUNT: usize = 3;
/// Debounce delay after a falling edge before confirming a press.
pub const BUTTON_DEBOUNCE_TIME: Duration = Duration::from_millis(200);

/// High-level actions triggered by button presses.
#[derive(Debug, defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum ButtonAction {
    /// Switch to the previous lamp pattern.
    PreviousPattern,
    /// Reset the lamp to the default pattern.
    ResetToDefaultPattern,
    /// Switch to the next lamp pattern.
    NextPattern,
}

/// Errors that can occur during button handling.
#[derive(Error, Debug, defmt::Format)]
pub enum ButtonError {
    #[error("Button tasks have already been spawned")]
    TasksAlreadySpawned,
    #[error("Failed to spawn button task: {0}")]
    TaskSpawnFailed(#[from] SpawnError),
}

/// Connects a hardware pin to a semantic button action.
pub struct ButtonBinding {
    /// Physical GPIO pin used as button input.
    pub pin: Peri<'static, AnyPin>,
    /// Action emitted when the button press is validated.
    pub action: ButtonAction,
}

const BUTTON_ACTION_QUEUE_SIZE: usize = 8;

static BUTTON_ACTIONS: Channel<CriticalSectionRawMutex, ButtonAction, BUTTON_ACTION_QUEUE_SIZE> =
    Channel::new();

/// Spawns asynchronous button tasks and exposes debounced actions.
pub struct ButtonHandler {
    bindings: Option<[ButtonBinding; BUTTON_COUNT]>,
}

impl ButtonHandler {
    /// Creates a button controller with all configured board bindings.
    pub const fn new(bindings: [ButtonBinding; BUTTON_COUNT]) -> Self {
        Self {
            bindings: Some(bindings),
        }
    }

    /// Spawns one button polling task per configured binding.
    pub fn spawn_tasks(&mut self, spawner: &Spawner) -> Result<(), ButtonError> {
        let bindings = self
            .bindings
            .take()
            .ok_or(ButtonError::TasksAlreadySpawned)?;

        for binding in bindings {
            let task_token = button_task(binding.pin, binding.action)?;
            spawner.spawn(task_token);
        }

        Ok(())
    }

    /// Waits for the next button action from the shared queue.
    pub async fn wait_for_action(&self) -> ButtonAction {
        BUTTON_ACTIONS.receive().await
    }
}

#[embassy_executor::task(pool_size = BUTTON_COUNT)]
async fn button_task(pin: Peri<'static, AnyPin>, action: ButtonAction) {
    let mut button = Input::new(pin, Pull::Up);

    loop {
        button.wait_for_falling_edge().await;
        Timer::after(BUTTON_DEBOUNCE_TIME).await;

        if button.is_low() {
            if BUTTON_ACTIONS.try_send(action).is_err() {
                warn!("Button action queue full, dropping input");
            }
            button.wait_for_rising_edge().await;
        }
    }
}
