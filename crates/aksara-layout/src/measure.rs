//! Text measurement: trait plus fixed stub (shaping substitute).

/// Measures text width and line height in points.
pub trait TextMeasurer {
    /// Width of `text` at `size_pt` in points.
    fn width_pt(&self, text: &str, size_pt: f32, bold: bool, italic: bool) -> f64;
    /// Height of one line at `size_pt` with `spacing` multiplier.
    fn line_height_pt(&self, size_pt: f32, spacing: f32) -> f64;
}

/// Deterministic stub: width = chars * size * ratio.
///
/// `bold`/`italic` do not change the result; this is documented
/// test scaffolding, not shaping. Real shaping is a later phase.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedMeasurer {
    /// Mean advance per char as a fraction of font size.
    pub avg_advance_ratio: f64,
    /// Line height as a fraction of (size * spacing).
    pub line_ratio: f64,
}

impl Default for FixedMeasurer {
    fn default() -> Self {
        Self {
            avg_advance_ratio: 0.6,
            line_ratio: 1.2,
        }
    }
}

impl TextMeasurer for FixedMeasurer {
    fn width_pt(&self, text: &str, size_pt: f32, _bold: bool, _italic: bool) -> f64 {
        text.chars().count() as f64 * f64::from(size_pt) * self.avg_advance_ratio
    }

    fn line_height_pt(&self, size_pt: f32, spacing: f32) -> f64 {
        f64::from(size_pt) * f64::from(spacing) * self.line_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_has_zero_width() {
        let m = FixedMeasurer::default();
        assert_eq!(m.width_pt("", 12.0, false, false), 0.0);
    }

    #[test]
    fn ten_chars_at_12pt_is_72pt() {
        let m = FixedMeasurer::default();
        assert!((m.width_pt("0123456789", 12.0, false, false) - 72.0).abs() < 1e-9);
    }

    #[test]
    fn bold_italic_do_not_change_fixed_width() {
        let m = FixedMeasurer::default();
        let plain = m.width_pt("hello", 12.0, false, false);
        assert_eq!(m.width_pt("hello", 12.0, true, false), plain);
        assert_eq!(m.width_pt("hello", 12.0, false, true), plain);
    }

    #[test]
    fn line_height_12pt_single_is_14_4() {
        let m = FixedMeasurer::default();
        assert!((m.line_height_pt(12.0, 1.0) - 14.4).abs() < 1e-9);
    }
}
