//! Pattern catalogue and frame rendering functions for the lamp.
use smart_leds::RGB8;

/// Number of LEDs in the devminds lamp layout.
pub(crate) const DEVMINDS_LAMP_LED_NUM: usize = 31;
/// Index of the pattern selected on startup.
pub const DEFAULT_PATTERN_INDEX: usize = 0;

const DEVMINDS_BLUE: RGB8 = RGB8 {
    r: 10,
    g: 46,
    b: 120,
};
const DEVMINDS_GREEN: RGB8 = RGB8 {
    r: 10,
    g: 100,
    b: 10,
};
const DEVMINDS_ORANGE: RGB8 = RGB8 {
    r: 150,
    g: 38,
    b: 0,
};
const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const WHITE: RGB8 = RGB8 {
    r: 255,
    g: 255,
    b: 255,
};

const DEVMINDS_LAMP_BLUE_PIXELS: [usize; 9] = [1, 2, 3, 6, 7, 13, 14, 17, 22];
const DEVMINDS_LAMP_GREEN_PIXELS: [usize; 7] = [0, 4, 8, 9, 10, 11, 12];
const DEVMINDS_LAMP_ORANGE_PIXELS: [usize; 7] = [5, 15, 16, 18, 19, 20, 21];
const DEVMINDS_TEXT_PIXELS: [usize; 8] = [23, 24, 25, 26, 27, 28, 29, 30];

// The following functions are used for compile-time validation of the pixel map above.
// They ensure that every pixel index is covered by exactly one of the color groups, and that no index is out of bounds.
const fn mark_indices_seen(indices: &[usize], seen: &mut u64) {
    let mut i = 0;
    while i < indices.len() {
        let index = indices[i];
        assert!(index < DEVMINDS_LAMP_LED_NUM);

        let bit = 1u64 << index;
        assert!((*seen & bit) == 0);
        *seen |= bit;

        i += 1;
    }
}
const fn validate_pixel_map() {
    let mut seen = 0u64;
    mark_indices_seen(&DEVMINDS_LAMP_BLUE_PIXELS, &mut seen);
    mark_indices_seen(&DEVMINDS_LAMP_GREEN_PIXELS, &mut seen);
    mark_indices_seen(&DEVMINDS_LAMP_ORANGE_PIXELS, &mut seen);
    mark_indices_seen(&DEVMINDS_TEXT_PIXELS, &mut seen);

    assert!(seen.count_ones() as usize == DEVMINDS_LAMP_LED_NUM);
}
const _: () = validate_pixel_map();

pub struct LedStripPattern {
    /// Display name for logs and UI interactions.
    pub name: &'static str,
    render: fn(&mut [RGB8; DEVMINDS_LAMP_LED_NUM], u32),
}

impl LedStripPattern {
    /// Renders a frame of this pattern into the provided color buffer.
    pub fn render(&self, colors: &mut [RGB8; DEVMINDS_LAMP_LED_NUM], frame: u32) {
        (self.render)(colors, frame);
    }
}

/// All available lamp patterns in selection order.
pub static LED_STRIP_PATTERNS: &[LedStripPattern] = &[
    LedStripPattern {
        name: "Brand Colors",
        render: render_colored,
    },
    LedStripPattern {
        name: "Rainbow",
        render: render_rainbow,
    },
    LedStripPattern {
        name: "White",
        render: render_white,
    },
];

fn render_colored(colors: &mut [RGB8; DEVMINDS_LAMP_LED_NUM], _frame: u32) {
    colors.fill(RGB8::default());

    for &index in &DEVMINDS_LAMP_BLUE_PIXELS {
        colors[index] = DEVMINDS_BLUE;
    }
    for &index in &DEVMINDS_LAMP_GREEN_PIXELS {
        colors[index] = DEVMINDS_GREEN;
    }
    for &index in &DEVMINDS_LAMP_ORANGE_PIXELS {
        colors[index] = DEVMINDS_ORANGE;
    }
    for &index in &DEVMINDS_TEXT_PIXELS {
        colors[index] = RED;
    }
}

fn render_white(colors: &mut [RGB8; DEVMINDS_LAMP_LED_NUM], _frame: u32) {
    colors.fill(WHITE);
}

fn render_rainbow(colors: &mut [RGB8; DEVMINDS_LAMP_LED_NUM], frame: u32) {
    let rainbow_offset = frame as u8;
    for (index, pixel) in colors.iter_mut().enumerate() {
        let position = (((index * 256) / DEVMINDS_LAMP_LED_NUM) + rainbow_offset as usize) as u8;
        *pixel = rainbow_wheel(position);
    }
}

fn rainbow_wheel(mut wheel_pos: u8) -> RGB8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern_index_by_name(name: &str) -> usize {
        LED_STRIP_PATTERNS
            .iter()
            .position(|pattern| pattern.name == name)
            .expect("Pattern must exist")
    }

    fn render_pattern(index: usize) -> [RGB8; DEVMINDS_LAMP_LED_NUM] {
        let mut colors = [RGB8::default(); DEVMINDS_LAMP_LED_NUM];
        let pattern = &LED_STRIP_PATTERNS[index];

        pattern.render(&mut colors, 0);

        colors
    }

    #[test]
    fn default_pattern_renders_brand_colors() {
        let colors = render_pattern(DEFAULT_PATTERN_INDEX);

        for &index in &DEVMINDS_LAMP_BLUE_PIXELS {
            assert_eq!(colors[index], DEVMINDS_BLUE);
        }
        for &index in &DEVMINDS_LAMP_GREEN_PIXELS {
            assert_eq!(colors[index], DEVMINDS_GREEN);
        }
        for &index in &DEVMINDS_LAMP_ORANGE_PIXELS {
            assert_eq!(colors[index], DEVMINDS_ORANGE);
        }
        for &index in &DEVMINDS_TEXT_PIXELS {
            assert_eq!(colors[index], RED);
        }
    }

    #[test]
    fn white_pattern_fills_every_pixel() {
        let colors = render_pattern(pattern_index_by_name("White"));

        assert!(colors.iter().all(|pixel| *pixel == WHITE));
    }

    #[test]
    fn rainbow_pattern_is_deterministic_and_wraps_every_256_frames() {
        let pattern = &LED_STRIP_PATTERNS[pattern_index_by_name("Rainbow")];
        let mut frame_0 = [RGB8::default(); DEVMINDS_LAMP_LED_NUM];
        let mut frame_1 = [RGB8::default(); DEVMINDS_LAMP_LED_NUM];
        let mut frame_256 = [RGB8::default(); DEVMINDS_LAMP_LED_NUM];

        pattern.render(&mut frame_0, 0);
        pattern.render(&mut frame_1, 1);
        pattern.render(&mut frame_256, 256);

        assert_ne!(frame_0, frame_1);
        assert_eq!(frame_0, frame_256);
    }
}
