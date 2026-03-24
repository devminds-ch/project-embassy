//! The firmware initializes board peripherals, spawns background tasks for
//! status LED blinking and button input, and then drives WS2812 frames based
//! on the active pattern selected by user input.
//! Pin 4 = GPIO 2 = Button 1
//! Pin 5 = GPIO 3 = Button 2
//! Pin 6 = GPIO 4 = Button 3
//! Pin 7 = GPIO 5 = LED
//! Pin 25 = GPIO 19 (WS2812 data)
#![no_std]
#![no_main]

use defmt::*;
use devminds_lamp::board::{LED_UPDATE_INTERVAL, LampBoard, STATUS_BLINK_INTERVAL};
use devminds_lamp::buttons::{ButtonAction, ButtonHandler};
use devminds_lamp::led_strip::LedStripPatternGenerator;
use devminds_lamp::rpi_pio::PioWs2812Controller;
use devminds_lamp::status_led::StatusLedController;
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_time::Ticker;
use {defmt_rtt as _, panic_probe as _};

/// Firmware entrypoint running on the Embassy async executor.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting devminds lamp...");

    let LampBoard {
        status_led,
        buttons,
        ws2812_data,
        ws2812_pio,
        ws2812_dma,
    } = LampBoard::from_peripherals(embassy_rp::init(Default::default()));

    let mut status_led_controller = StatusLedController::new(status_led, STATUS_BLINK_INTERVAL);
    status_led_controller.spawn_task(&spawner);

    let mut button_handler = ButtonHandler::new(buttons);
    button_handler.spawn_tasks(&spawner);

    // Setup Raspberry Pi Pico's PIO to drive the WS2812 LED string
    let mut pio_ws2812_controller = PioWs2812Controller::new(ws2812_pio, ws2812_dma, ws2812_data);

    let mut led_strip_pattern_generator = LedStripPatternGenerator::new();

    info!("Startup complete, entering main loop");
    // Loop forever making RGB values and pushing them out to the WS2812.
    let mut ticker = Ticker::every(LED_UPDATE_INTERVAL);
    loop {
        match select(ticker.next(), button_handler.wait_for_action()).await {
            Either::First(_) => {
                if led_strip_pattern_generator.render_next_frame() {
                    pio_ws2812_controller
                        .write(led_strip_pattern_generator.colors())
                        .await;
                }
            }
            Either::Second(action) => {
                let pattern_name = match action {
                    ButtonAction::PreviousPattern => {
                        led_strip_pattern_generator.previous_pattern().name
                    }
                    ButtonAction::ResetToDefaultPattern => {
                        led_strip_pattern_generator.default_pattern().name
                    }
                    ButtonAction::NextPattern => led_strip_pattern_generator.next_pattern().name,
                };

                info!("Switching to pattern: {=str}", pattern_name);

                if led_strip_pattern_generator.render_next_frame() {
                    pio_ws2812_controller
                        .write(led_strip_pattern_generator.colors())
                        .await;
                }
            }
        }
    }
}
