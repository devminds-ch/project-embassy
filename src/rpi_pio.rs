//! Raspberry Pi Pico PIO driver for the WS2812 LED strip.
//!
//! Wraps the embassy-rp PIO state machine setup and exposes a simple
//! `PioWs2812Controller` that can write a frame of pixel data.

use embassy_rp::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{DMA_CH0, PIN_19, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{Grb, PioWs2812, PioWs2812Program};
use smart_leds::RGB8;

use crate::led_strip_patterns::DEVMINDS_LAMP_LED_NUM;

bind_interrupts!(struct PioIrqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

bind_interrupts!(struct DmaIrqs {
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
});

/// Drives the WS2812 LED strip via the RP2350's PIO peripheral.
pub struct PioWs2812Controller<'d> {
    ws2812: PioWs2812<'d, PIO0, 0, DEVMINDS_LAMP_LED_NUM, Grb>,
}

impl<'d> PioWs2812Controller<'d> {
    /// Initialises the PIO state machine and WS2812 program.
    ///
    /// Consumes the PIO block, DMA channel, and data pin peripherals.
    pub fn new(pio: Peri<'d, PIO0>, dma: Peri<'d, DMA_CH0>, data: Peri<'d, PIN_19>) -> Self {
        let Pio {
            common: mut pio_common,
            sm0: pio_state_machine,
            ..
        } = Pio::new(pio, PioIrqs);

        let pio_program = PioWs2812Program::new(&mut pio_common);
        let ws2812 = PioWs2812::new(
            &mut pio_common,
            pio_state_machine,
            dma,
            DmaIrqs,
            data,
            &pio_program,
        );

        Self { ws2812 }
    }

    /// Writes a full frame of pixel data to the LED strip.
    pub async fn write(&mut self, colors: &[RGB8; DEVMINDS_LAMP_LED_NUM]) {
        self.ws2812.write(colors).await;
    }
}
