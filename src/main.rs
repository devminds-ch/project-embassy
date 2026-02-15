#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Input, Level, Output, Pull};
use {defmt_rtt as _, panic_probe as _};

// Pin 4 = GPIO 2 = Button 1
// Pin 5 = GPIO 3 = Button 2
// Pin 6 = GPIO 4 = Button 3
// Pin 7 = GPIO 5 = LED
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_5, Level::Low);
    let mut button1 = Input::new(p.PIN_2, Pull::Up);

    loop {
        info!("Waiting for button 1 press...");
        button1.wait_for_low().await;

        info!("Button 1 pressed - LED on");
        led.set_high();

        button1.wait_for_high().await;

        info!("Button 1 released - LED off");
        led.set_low();
    }
}
