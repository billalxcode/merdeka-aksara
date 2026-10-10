//! Image placeholder: source reference plus logical size.
//!
//! Phase 1 stores no pixels and performs no decoding. Layout and
//! rendering phases will resolve `source` later.

use crate::id::NodeId;

/// Image reference with a fixed logical size in points.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// Stable node id.
    pub id: NodeId,
    /// Source path or key, resolved by a later phase.
    pub source: String,
    /// Display width in points.
    pub width_pt: f64,
    /// Display height in points.
    pub height_pt: f64,
}

impl Image {
    /// Create an image placeholder.
    pub fn new(id: NodeId, source: &str, width_pt: f64, height_pt: f64) -> Self {
        Self {
            id,
            source: String::from(source),
            width_pt,
            height_pt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_fields() {
        let img = Image::new(NodeId::new(1), "logo.png", 100.0, 50.0);
        assert_eq!(img.source, "logo.png");
        assert_eq!(img.width_pt, 100.0);
        assert_eq!(img.height_pt, 50.0);
    }
}
