//! Block nodes: the content units inside a section.

use crate::id::NodeId;
use crate::image::Image;
use crate::page::PageSetup;
use crate::table::Table;
use crate::text::Paragraph;

/// A break that starts a new section with its own page setup.
///
/// The `page_setup` describes the section that follows the break,
/// mirroring how `PageBreak` anchors a page boundary for editing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionBreak {
    /// Stable node id of the break marker.
    pub id: NodeId,
    /// Page setup of the section following the break.
    pub page_setup: PageSetup,
}

impl SectionBreak {
    /// Create a section break carrying the next section's setup.
    pub fn new(id: NodeId, page_setup: PageSetup) -> Self {
        Self { id, page_setup }
    }
}

/// One content unit inside a section.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Text paragraph.
    Paragraph(Paragraph),
    /// Grid table with recursive cell content.
    Table(Table),
    /// Image placeholder (no decoding in Phase 1).
    Image(Image),
    /// Forced break to the next page.
    PageBreak,
    /// Break that starts a new section with its own page setup.
    SectionBreak(SectionBreak),
}

impl Block {
    /// Stable id of the block, if it carries one.
    ///
    /// Returns `None` for `PageBreak`, which holds no node.
    pub fn id(&self) -> Option<NodeId> {
        match self {
            Self::Paragraph(para) => Some(para.id),
            Self::Table(table) => Some(table.id),
            Self::Image(image) => Some(image.id),
            Self::SectionBreak(section_break) => Some(section_break.id),
            Self::PageBreak => None,
        }
    }

    /// Words in the block. Images and breaks count zero.
    pub fn word_count(&self) -> usize {
        match self {
            Self::Paragraph(para) => para.word_count(),
            Self::Table(table) => table.word_count(),
            Self::Image(_) | Self::PageBreak | Self::SectionBreak(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{CharacterStyle, TextRun};

    fn para_with_text(id: u64, text: &str) -> Paragraph {
        let mut para = Paragraph::new(NodeId::new(id));
        para.push_run(TextRun::new(
            NodeId::new(id + 100),
            text,
            CharacterStyle::default(),
        ));
        para
    }

    #[test]
    fn dispatch_word_count() {
        assert_eq!(
            Block::Paragraph(para_with_text(1, "one two")).word_count(),
            2
        );
        assert_eq!(
            Block::Image(Image::new(NodeId::new(2), "a.png", 10.0, 10.0)).word_count(),
            0
        );
        assert_eq!(Block::PageBreak.word_count(), 0);
    }

    #[test]
    fn id_returns_inner_or_none() {
        assert_eq!(
            Block::Paragraph(para_with_text(1, "x")).id(),
            Some(NodeId::new(1))
        );
        assert_eq!(Block::PageBreak.id(), None);
    }
}
