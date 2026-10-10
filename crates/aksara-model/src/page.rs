//! Page setup: paper size, orientation, and margins in points.

use crate::units::{A4_HEIGHT_PT, A4_WIDTH_PT};

/// Page orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Orientation {
    /// Height is longer than width.
    #[default]
    Portrait,
    /// Width is longer than height.
    Landscape,
}

/// Paper size plus margins, all in logical points.
///
/// Margins sit outside the content area and never hold document text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageSetup {
    /// Page width in points.
    pub width_pt: f64,
    /// Page height in points.
    pub height_pt: f64,
    /// Top margin in points.
    pub margin_top_pt: f64,
    /// Bottom margin in points.
    pub margin_bottom_pt: f64,
    /// Left margin in points.
    pub margin_left_pt: f64,
    /// Right margin in points.
    pub margin_right_pt: f64,
}

impl PageSetup {
    /// A4 portrait with uniform one-inch (72 pt) margins.
    pub fn a4_portrait() -> Self {
        Self {
            width_pt: A4_WIDTH_PT,
            height_pt: A4_HEIGHT_PT,
            margin_top_pt: 72.0,
            margin_bottom_pt: 72.0,
            margin_left_pt: 72.0,
            margin_right_pt: 72.0,
        }
    }

    /// A4 landscape: same paper, width and height swapped.
    pub fn a4_landscape() -> Self {
        let base = Self::a4_portrait();
        Self {
            width_pt: base.height_pt,
            height_pt: base.width_pt,
            ..base
        }
    }

    /// Usable content width after left and right margins.
    pub fn content_width(&self) -> f64 {
        self.width_pt - self.margin_left_pt - self.margin_right_pt
    }

    /// Usable content height after top and bottom margins.
    pub fn content_height(&self) -> f64 {
        self.height_pt - self.margin_top_pt - self.margin_bottom_pt
    }

    /// True when the size is positive and margins fit inside the page.
    pub fn is_valid(&self) -> bool {
        self.width_pt > 0.0
            && self.height_pt > 0.0
            && self.margin_top_pt >= 0.0
            && self.margin_bottom_pt >= 0.0
            && self.margin_left_pt >= 0.0
            && self.margin_right_pt >= 0.0
            && self.margin_left_pt + self.margin_right_pt < self.width_pt
            && self.margin_top_pt + self.margin_bottom_pt < self.height_pt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4_portrait_has_expected_size() {
        let setup = PageSetup::a4_portrait();
        assert!((setup.width_pt - 595.28).abs() <= 0.05);
        assert!((setup.height_pt - 841.89).abs() <= 0.05);
        assert!((setup.content_width() - (595.28 - 144.0)).abs() <= 0.05);
        assert!(setup.is_valid());
    }

    #[test]
    fn a4_landscape_swaps_dimensions() {
        let setup = PageSetup::a4_landscape();
        assert!((setup.width_pt - 841.89).abs() <= 0.05);
        assert!((setup.height_pt - 595.28).abs() <= 0.05);
        assert!(setup.is_valid());
    }

    #[test]
    fn rejects_excessive_margins() {
        let setup = PageSetup {
            margin_left_pt: 500.0,
            margin_right_pt: 500.0,
            ..PageSetup::a4_portrait()
        };
        assert!(!setup.is_valid());
    }

    #[test]
    fn rejects_zero_size() {
        let setup = PageSetup {
            width_pt: 0.0,
            height_pt: 0.0,
            ..PageSetup::a4_portrait()
        };
        assert!(!setup.is_valid());
    }
}
