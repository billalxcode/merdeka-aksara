//! Document units: points, millimeters, inches, and pixels.
//!
//! The document stores sizes in logical points (pt). Pixels are only
//! derived at render time from a zoom factor, never stored:
//!
//! ```text
//! px = pt * 96 / 72 * zoom
//! ```
//!
//! A4 paper: 210 x 297 mm, which converts to about 595.28 x 841.89 pt.
//! Those constants are rounded, so tests compare with a 0.05 tolerance.

/// Points per inch, by definition.
pub const PT_PER_INCH: f64 = 72.0;

/// Millimeters per inch, by definition.
pub const MM_PER_INCH: f64 = 25.4;

/// A4 width in millimeters (ISO 216).
pub const A4_WIDTH_MM: f64 = 210.0;

/// A4 height in millimeters (ISO 216).
pub const A4_HEIGHT_MM: f64 = 297.0;

/// A4 width in points (rounded from 210 mm).
pub const A4_WIDTH_PT: f64 = 595.28;

/// A4 height in points (rounded from 297 mm).
pub const A4_HEIGHT_PT: f64 = 841.89;

/// Convert millimeters to points.
pub fn mm_to_pt(mm: f64) -> f64 {
    mm * PT_PER_INCH / MM_PER_INCH
}

/// Convert points to millimeters.
pub fn pt_to_mm(pt: f64) -> f64 {
    pt * MM_PER_INCH / PT_PER_INCH
}

/// Convert inches to points.
pub fn inch_to_pt(inch: f64) -> f64 {
    inch * PT_PER_INCH
}

/// Convert points to inches.
pub fn pt_to_inch(pt: f64) -> f64 {
    pt / PT_PER_INCH
}

/// Convert points to screen pixels at a 96 DPI preview scaled by zoom.
///
/// `zoom` is a scale factor (1.0 means 100 percent). Preview DPI never
/// changes the stored document size used for PDF export.
pub fn pt_to_px(pt: f64, zoom: f64) -> f64 {
    pt * 96.0 / 72.0 * zoom
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn mm_to_pt_of_one_inch() {
        assert!(approx(mm_to_pt(25.4), 72.0, 1e-9));
    }

    #[test]
    fn pt_to_mm_of_one_inch() {
        assert!(approx(pt_to_mm(72.0), 25.4, 1e-9));
    }

    #[test]
    fn pt_to_px_at_full_and_half_zoom() {
        assert!(approx(pt_to_px(72.0, 1.0), 96.0, 1e-9));
        assert!(approx(pt_to_px(72.0, 1.5), 144.0, 1e-9));
    }

    #[test]
    fn a4_roundtrip_within_tolerance() {
        assert!(approx(mm_to_pt(A4_WIDTH_MM), A4_WIDTH_PT, 0.05));
        assert!(approx(mm_to_pt(A4_HEIGHT_MM), A4_HEIGHT_PT, 0.05));
        assert!(approx(pt_to_mm(A4_WIDTH_PT), A4_WIDTH_MM, 0.05));
        assert!(approx(pt_to_mm(A4_HEIGHT_PT), A4_HEIGHT_MM, 0.05));
    }

    #[test]
    fn inch_helpers_are_inverse() {
        assert!(approx(pt_to_inch(inch_to_pt(2.5)), 2.5, 1e-9));
    }
}
