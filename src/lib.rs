#![no_std]

//! Core library for the devminds Embassy-based lamp firmware.
//!
//! This crate contains two layers:
//! - Hardware-facing modules enabled for embedded targets (`target_os = "none"`)
//! - Pure rendering and animation logic that is host-testable

/// Embedded board pin mapping and timing constants.
#[cfg(target_os = "none")]
pub mod board;
/// Button actions, bindings, and asynchronous button input handling.
#[cfg(target_os = "none")]
pub mod buttons;
/// Lamp state machine and pattern selection orchestration.
pub mod led_strip;
/// LED pattern definitions and frame renderers.
pub mod led_strip_patterns;
/// PIO-based WS2812 LED strip driver.
#[cfg(target_os = "none")]
pub mod rpi_pio;
/// Status LED background blink task.
#[cfg(target_os = "none")]
pub mod status_led;

#[cfg(test)]
extern crate std;
