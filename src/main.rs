//! This example shows powerful PIO module in the RP235x chip to communicate with WS2812 LED modules.
//! See (https://www.sparkfun.com/categories/tags/ws2812)
//! Pin 4 = GPIO 2 = Button 1
//! Pin 5 = GPIO 3 = Button 2
//! Pin 6 = GPIO 4 = Button 3
//! Pin 7 = GPIO 5 = LED
//! Pin 21 = GPIO 16 (WS2812 data out)
//! Pin 25 = GPIO 19 (WS2812 data in)

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use embassy_time::{Duration, Ticker};
use smart_leds::RGB8;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

// devminds brand colors
pub const DM_BLUE: RGB8 = RGB8 {
    r: 10,
    g: 46,
    b: 120,
};
pub const DM_GREEN: RGB8 = RGB8 {
    r: 10,
    g: 100,
    b: 10,
};
pub const DM_ORANGE: RGB8 = RGB8 {
    r: 150,
    g: 38,
    b: 0,
};
pub const DM_RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };

// devminds logo LED pixel mappings
pub const DM_LOGO_BLUE_PIXELS: [usize; 9] = [1, 2, 3, 6, 7, 13, 14, 17, 22];
pub const DM_LOGO_GREEN_PIXELS: [usize; 7] = [0, 4, 8, 9, 10, 11, 12];
pub const DM_LOGO_ORANGE_PIXELS: [usize; 7] = [5, 15, 16, 18, 19, 20, 21];

// devminds text LED pixel mappings
pub const DM_TEXT_PIXELS: [usize; 8] = [23, 24, 25, 26, 27, 28, 29, 30];

#[derive(Debug, Format, Clone, Copy)]
pub enum RunMode {
    Colored,
    Rainbow,
    White,
}

// Global run mode state (using embassy Signal for thread-safe communication)
static RUN_MODE: Signal<CriticalSectionRawMutex, RunMode> = Signal::new();

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}

#[embassy_executor::task]
async fn blink_task(pin: Peri<'static, AnyPin>, delay: Duration) {
    let mut led = Output::new(pin, Level::Low);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after(delay).await;
        led.set_low();
        Timer::after(delay).await;
    }
}

#[embassy_executor::task(pool_size = 3)]
async fn button_task(pin: Peri<'static, AnyPin>, mode: RunMode) {
    let mut button = Input::new(pin, Pull::Up);
    loop {
        button.wait_for_falling_edge().await;

        // Set given mode
        RUN_MODE.signal(mode);
        info!("Switching to mode: {:?}", mode);

        // Sleep a bit to debounce the button (simple software debounce)
        Timer::after(Duration::from_millis(200)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    // Spawned tasks run in the background, concurrently.
    spawner
        .spawn(blink_task(p.PIN_5.into(), Duration::from_millis(1000)))
        .unwrap();

    spawner
        .spawn(button_task(p.PIN_2.into(), RunMode::Colored))
        .unwrap();
    spawner
        .spawn(button_task(p.PIN_3.into(), RunMode::Rainbow))
        .unwrap();
    spawner
        .spawn(button_task(p.PIN_4.into(), RunMode::White))
        .unwrap();

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    // This is the number of leds in the string
    const NUM_LEDS: usize = 31;
    let mut data = [RGB8::default(); NUM_LEDS];

    // Setup Raspberry Pi Pico's PIO to drive the WS2812 LED string
    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_19, &program);

    // Initialize with default mode
    let mut current_mode = RunMode::Colored;

    // Loop forever making RGB values and pushing them out to the WS2812
    let mut ticker = Ticker::every(Duration::from_millis(10));
    loop {
        // Check for mode updates (non-blocking)
        if RUN_MODE.signaled() {
            current_mode = RUN_MODE.wait().await;
        }

        match current_mode {
            RunMode::Colored => {
                for &i in &DM_LOGO_BLUE_PIXELS {
                    data[i] = DM_BLUE;
                }
                for &i in &DM_LOGO_GREEN_PIXELS {
                    data[i] = DM_GREEN;
                }
                for &i in &DM_LOGO_ORANGE_PIXELS {
                    data[i] = DM_ORANGE;
                }
                for &i in &DM_TEXT_PIXELS {
                    data[i] = DM_RED;
                }
                ws2812.write(&data).await;
                ticker.next().await;
            }
            RunMode::Rainbow => {
                for j in 0..(256 * 5) {
                    // If mode changed, exit rainbow loop early
                    if RUN_MODE.signaled() {
                        current_mode = RUN_MODE.wait().await;
                        break;
                    }

                    //debug!("New Colors:");
                    for (i, pixel) in data.iter_mut().enumerate() {
                        *pixel =
                            wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                        //debug!("R: {} G: {} B: {}", pixel.r, pixel.g, pixel.b);
                    }
                    ws2812.write(&data).await;
                    ticker.next().await;
                }
            }
            RunMode::White => {
                for pixel in data.iter_mut() {
                    *pixel = RGB8::new(255, 255, 255);
                }
                ws2812.write(&data).await;
                ticker.next().await;
            }
        }
    }
}
