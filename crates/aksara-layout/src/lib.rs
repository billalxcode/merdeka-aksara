//! Aksara layout engine - geometry only (std plus aksara-model).
//!
//! Pipeline order per spec section 4:
//! shaping (stub) -> line breaking -> paragraph layout -> block layout
//! -> pagination -> render/display list. Coordinates are points (pt)
//! with the origin at the top-left of the content area. Pages are
//! computed results and are never stored in the model.

pub mod block;
pub mod line_break;
pub mod measure;
pub mod page;
pub mod paragraph;

pub use block::{LaidBlock, layout_blocks};
pub use line_break::break_greedy;
pub use measure::{FixedMeasurer, TextMeasurer};
pub use page::{LaidPage, PlacedLine, paginate};
pub use paragraph::{LaidLine, LaidParagraph, layout_paragraph};
