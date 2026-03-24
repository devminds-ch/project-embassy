use smart_leds::RGB8;

use crate::led_strip_patterns::{DEFAULT_PATTERN_INDEX, DEVMINDS_LAMP_LED_NUM, PATTERNS, Pattern};

/// Owns the current LED frame and active animation pattern.
pub struct LedStripPatternGenerator {
    colors: [RGB8; DEVMINDS_LAMP_LED_NUM],
    current_pattern: usize,
    pattern_frame: u32,
}

impl Default for LedStripPatternGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl LedStripPatternGenerator {
    /// Creates a controller initialized to the default pattern.
    pub fn new() -> Self {
        let mut lamp = Self {
            colors: [RGB8::default(); DEVMINDS_LAMP_LED_NUM],
            current_pattern: DEFAULT_PATTERN_INDEX,
            pattern_frame: 0,
        };
        lamp.activate_pattern(DEFAULT_PATTERN_INDEX);
        lamp
    }

    /// Selects the next pattern in the list and returns it.
    pub fn next_pattern(&mut self) -> &'static Pattern {
        let next_pattern = (self.current_pattern + 1) % PATTERNS.len();
        self.activate_pattern(next_pattern);
        self.current_pattern()
    }

    /// Selects the previous pattern in the list and returns it.
    pub fn previous_pattern(&mut self) -> &'static Pattern {
        let previous_pattern = if self.current_pattern == 0 {
            PATTERNS.len() - 1
        } else {
            self.current_pattern - 1
        };
        self.activate_pattern(previous_pattern);
        self.current_pattern()
    }

    /// Activates the default pattern and returns it.
    pub fn default_pattern(&mut self) -> &'static Pattern {
        self.activate_pattern(DEFAULT_PATTERN_INDEX);
        self.current_pattern()
    }

    /// Returns the currently active pattern descriptor.
    pub fn current_pattern(&self) -> &'static Pattern {
        &PATTERNS[self.current_pattern]
    }

    /// Returns the current LED frame buffer.
    pub fn colors(&self) -> &[RGB8; DEVMINDS_LAMP_LED_NUM] {
        &self.colors
    }

    /// Renders one frame and reports whether the output changed.
    ///
    /// This allows callers to skip WS2812 transfers when a static pattern
    /// would produce the same frame twice.
    pub fn render_next_frame(&mut self) -> bool {
        let previous_colors = self.colors;
        self.current_pattern()
            .render(&mut self.colors, self.pattern_frame);
        self.pattern_frame = self.pattern_frame.wrapping_add(1);
        self.colors != previous_colors
    }

    fn activate_pattern(&mut self, index: usize) {
        self.current_pattern = index % PATTERNS.len();
        self.pattern_frame = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::led_strip_patterns::{DEFAULT_PATTERN_INDEX, PATTERNS};

    #[test]
    fn default_static_pattern_changes_once_until_pattern_changes() {
        let mut lamp = LedStripPatternGenerator::new();

        assert!(lamp.render_next_frame());
        assert!(!lamp.render_next_frame());

        assert_eq!(
            lamp.current_pattern().name,
            PATTERNS[DEFAULT_PATTERN_INDEX].name
        );
    }

    #[test]
    fn white_static_pattern_changes_once_until_pattern_changes() {
        let mut lamp = LedStripPatternGenerator::new();
        lamp.next_pattern();
        lamp.next_pattern();

        assert!(lamp.render_next_frame());
        assert!(!lamp.render_next_frame());
    }

    #[test]
    fn rainbow_mode_updates_every_frame() {
        let mut lamp = LedStripPatternGenerator::new();
        lamp.next_pattern();

        assert!(lamp.render_next_frame());
        let first_frame = *lamp.colors();

        assert!(lamp.render_next_frame());
        assert_ne!(lamp.colors(), &first_frame);
    }

    #[test]
    fn switching_back_to_rainbow_restarts_animation() {
        let mut reference = LedStripPatternGenerator::new();
        reference.next_pattern();
        assert!(reference.render_next_frame());
        let initial_rainbow_frame = *reference.colors();

        let mut lamp = LedStripPatternGenerator::new();
        lamp.next_pattern();
        assert!(lamp.render_next_frame());
        assert!(lamp.render_next_frame());

        lamp.default_pattern();
        assert!(lamp.render_next_frame());

        lamp.next_pattern();
        assert!(lamp.render_next_frame());
        assert_eq!(lamp.colors(), &initial_rainbow_frame);
    }

    #[test]
    fn white_pattern_second_frame_has_same_result_and_reports_no_change() {
        let mut lamp = LedStripPatternGenerator::new();
        lamp.previous_pattern();

        assert!(lamp.render_next_frame());
        let first_frame = *lamp.colors();

        assert!(!lamp.render_next_frame());
        assert_eq!(lamp.colors(), &first_frame);
    }

    #[test]
    fn next_pattern_wraps() {
        let mut lamp = LedStripPatternGenerator::new();

        for expected_name in PATTERNS.iter().skip(1).map(|pattern| pattern.name) {
            assert_eq!(lamp.next_pattern().name, expected_name);
        }

        assert_eq!(
            lamp.next_pattern().name,
            PATTERNS[DEFAULT_PATTERN_INDEX].name
        );
    }

    #[test]
    fn previous_pattern_wraps() {
        let mut lamp = LedStripPatternGenerator::new();

        assert_eq!(
            lamp.previous_pattern().name,
            PATTERNS[PATTERNS.len() - 1].name
        );
        assert_eq!(
            lamp.next_pattern().name,
            PATTERNS[DEFAULT_PATTERN_INDEX].name
        );
    }

    #[test]
    fn reset_returns_to_default_pattern() {
        let mut lamp = LedStripPatternGenerator::new();

        lamp.next_pattern();
        lamp.next_pattern();

        assert_eq!(
            lamp.default_pattern().name,
            PATTERNS[DEFAULT_PATTERN_INDEX].name
        );
    }
}
