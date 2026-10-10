//! Aksara document model - pure block-based foundation (std only).
//!
//! Phase 1: units, page setup, metadata, node ids, text, blocks,
//! tables, images, sections, styles, and documents. No layout,
//! no rendering, no IO. Pages are computed by layout, not stored.

pub mod block;
pub mod document;
pub mod id;
pub mod image;
pub mod metadata;
pub mod page;
pub mod section;
pub mod style;
pub mod table;
pub mod text;
pub mod units;

pub use block::{Block, SectionBreak};
pub use document::Document;
pub use id::NodeId;
pub use image::Image;
pub use metadata::DocumentMetadata;
pub use page::{Orientation, PageSetup};
pub use section::Section;
pub use style::{
    NORMAL_STYLE, NamedStyle, PartialCharacterStyle, PartialParagraphStyle, StyleSheet,
};
pub use table::{MAX_COLS, MAX_ROWS, Table, TableCell, TableRow};
pub use text::{Alignment, CharacterStyle, Paragraph, ParagraphStyle, TextRun};
pub use units::{
    A4_HEIGHT_MM, A4_HEIGHT_PT, A4_WIDTH_MM, A4_WIDTH_PT, MM_PER_INCH, PT_PER_INCH, inch_to_pt,
    mm_to_pt, pt_to_inch, pt_to_mm, pt_to_px,
};
