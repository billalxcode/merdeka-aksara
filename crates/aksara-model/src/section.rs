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
    /// Repeating header blocks, shown on every page of this section.
    pub header: Vec<Block>,
    /// Repeating footer blocks, shown on every page of this section.
    pub footer: Vec<Block>,
}

impl Section {
    /// Create an empty section with the given page setup.
    pub fn new(id: NodeId, page_setup: PageSetup) -> Self {
        Self {
            id,
            page_setup,
            blocks: Vec::new(),
            header: Vec::new(),
            footer: Vec::new(),
        }
    }

    /// Append a block to the section.
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Insert a block at the given index, shifting later blocks right.
    ///
    /// Panics when `index > self.blocks.len()`, like `Vec::insert`.
    /// Ids of existing blocks are left untouched.
    pub fn insert_block(&mut self, index: usize, block: Block) {
        self.blocks.insert(index, block);
    }

    /// Remove and return the block at the given index.
    ///
    /// Panics when `index` is out of bounds, like `Vec::remove`.
    /// Ids of the remaining blocks are left untouched.
    pub fn remove_block(&mut self, index: usize) -> Block {
        self.blocks.remove(index)
    }

    /// Words across all blocks in the section.
    ///
    /// Header and footer blocks are excluded: they repeat on every
    /// page and must not change the body word count.
    pub fn word_count(&self) -> usize {
        self.blocks.iter().map(Block::word_count).sum()
    }

    /// Number of top-level blocks (header/footer excluded).
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Replace the header blocks.
    pub fn with_header(mut self, header: Vec<Block>) -> Self {
        self.header = header;
        self
    }

    /// Replace the footer blocks.
    pub fn with_footer(mut self, footer: Vec<Block>) -> Self {
        self.footer = footer;
        self
    }

    /// Replace the header blocks in place.
    pub fn set_header(&mut self, header: Vec<Block>) {
        self.header = header;
    }

    /// Replace the footer blocks in place.
    pub fn set_footer(&mut self, footer: Vec<Block>) {
        self.footer = footer;
    }

    /// Remove all header blocks.
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    /// Remove all footer blocks.
    pub fn clear_footer(&mut self) {
        self.footer.clear();
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

    fn para_block(id: u64, text: &str) -> Block {
        let mut para = Paragraph::new(NodeId::new(id));
        para.push_run(TextRun::new(
            NodeId::new(id + 1000),
            text,
            CharacterStyle::default(),
        ));
        Block::Paragraph(para)
    }

    #[test]
    fn insert_preserves_ids() {
        let mut section = Section::new(NodeId::new(1), PageSetup::a4_portrait());
        section.push_block(para_block(10, "a"));
        section.push_block(para_block(11, "b"));
        section.push_block(para_block(12, "c"));
        let before: Vec<Option<NodeId>> = section.blocks.iter().map(Block::id).collect();
        section.insert_block(1, para_block(20, "x"));
        assert_eq!(section.block_count(), 4);
        assert_eq!(section.blocks[0].id(), before[0]);
        assert_eq!(section.blocks[1].id(), Some(NodeId::new(20)));
        assert_eq!(section.blocks[2].id(), before[1]);
        assert_eq!(section.blocks[3].id(), before[2]);
    }

    #[test]
    fn remove_preserves_ids() {
        let mut section = Section::new(NodeId::new(1), PageSetup::a4_portrait());
        section.push_block(para_block(10, "a"));
        section.push_block(para_block(11, "b"));
        section.push_block(para_block(12, "c"));
        let before: Vec<Option<NodeId>> = section.blocks.iter().map(Block::id).collect();
        let removed = section.remove_block(1);
        assert_eq!(removed.id(), before[1]);
        assert_eq!(section.block_count(), 2);
        assert_eq!(section.blocks[0].id(), before[0]);
        assert_eq!(section.blocks[1].id(), before[2]);
    }
}
