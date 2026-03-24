use embassy_rp::gpio::AnyPin;
use embassy_rp::peripherals::{DMA_CH0, PIN_19, PIO0};
use embassy_rp::{Peri, Peripherals};
use embassy_time::Duration;

use crate::buttons::{BUTTON_COUNT, ButtonAction, ButtonBinding};

/// Blink interval for the onboard status LED.
pub const STATUS_BLINK_INTERVAL: Duration = Duration::from_millis(1_000);
/// Frame update cadence for the WS2812 LED strip.
pub const LED_UPDATE_INTERVAL: Duration = Duration::from_millis(10);

/// Runtime board resources and static pin assignments.
///
/// This type bundles all peripherals needed by the firmware entrypoint.
pub struct LampBoard {
    /// Onboard LED pin used for the heartbeat blink task.
    pub status_led: Peri<'static, AnyPin>,
    /// Debounced user buttons and their semantic actions.
    pub buttons: [ButtonBinding; BUTTON_COUNT],
    /// Data pin connected to the WS2812 strip.
    pub ws2812_data: Peri<'static, PIN_19>,
    /// PIO block driving the WS2812 protocol.
    pub ws2812_pio: Peri<'static, PIO0>,
    /// DMA channel feeding pixel data into the PIO state machine.
    pub ws2812_dma: Peri<'static, DMA_CH0>,
}

impl LampBoard {
    /// Creates a fully mapped board instance from initialized Embassy peripherals.
    pub fn from_peripherals(peripherals: Peripherals) -> Self {
        Self {
            status_led: peripherals.PIN_5.into(),
            buttons: [
                ButtonBinding {
                    pin: peripherals.PIN_2.into(),
                    action: ButtonAction::PreviousPattern,
                },
                ButtonBinding {
                    pin: peripherals.PIN_3.into(),
                    action: ButtonAction::ResetToDefaultPattern,
                },
                ButtonBinding {
                    pin: peripherals.PIN_4.into(),
                    action: ButtonAction::NextPattern,
                },
            ],
            ws2812_data: peripherals.PIN_19.into(),
            ws2812_pio: peripherals.PIO0,
            ws2812_dma: peripherals.DMA_CH0,
        }
    }
}
