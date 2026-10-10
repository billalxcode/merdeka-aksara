//! Sections: one page setup plus an ordered block list.
//!
//! A section groups content sharing paper size, margins, and
//! orientation. Computed pages are not stored here.

use crate::block::Block;
use crate::id::NodeId;
use crate::page::PageSetup;

/// A document section with its own page setup.
#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    /// Stable node id.
    pub id: NodeId,
    /// Paper size, margins, and orientation for this section.
    pub page_setup: PageSetup,
    /// Ordered content blocks.
    pub blocks: Vec<Block>,
}

impl Section {
    /// Create an empty section with the given page setup.
    pub fn new(id: NodeId, page_setup: PageSetup) -> Self {
        Self {
            id,
            page_setup,
            blocks: Vec::new(),
        }
    }

    /// Append a block to the section.
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Words across all blocks in the section.
    pub fn word_count(&self) -> usize {
        self.blocks.iter().map(Block::word_count).sum()
    }

    /// Number of top-level blocks.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{CharacterStyle, Paragraph, TextRun};

    #[test]
    fn push_and_count_words() {
        let mut section = Section::new(NodeId::new(1), PageSetup::a4_portrait());
        let mut para = Paragraph::new(NodeId::new(2));
        para.push_run(TextRun::new(
            NodeId::new(3),
            "a b c",
            CharacterStyle::default(),
        ));
        section.push_block(Block::Paragraph(para));
        section.push_block(Block::PageBreak);
        assert_eq!(section.word_count(), 3);
        assert_eq!(section.block_count(), 2);
    }

    #[test]
    fn empty_section_counts_zero() {
        let section = Section::new(NodeId::new(1), PageSetup::a4_portrait());
        assert_eq!(section.word_count(), 0);
        assert_eq!(section.block_count(), 0);
    }
}
