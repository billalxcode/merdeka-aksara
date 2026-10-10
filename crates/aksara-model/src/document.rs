//! The document root: metadata, sections, styles, and id counter.
//!
//! The document owns a monotone id counter. Call `alloc_id` for every
//! new node so ids stay unique inside one document.

use crate::block::{Block, SectionBreak};
use crate::id::NodeId;
use crate::metadata::DocumentMetadata;
use crate::page::PageSetup;
use crate::section::Section;
use crate::style::StyleSheet;
use crate::text::Paragraph;

/// Block-based document root.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// Title, author, and timestamps.
    pub metadata: DocumentMetadata,
    /// Ordered sections.
    pub sections: Vec<Section>,
    /// Named styles.
    pub styles: StyleSheet,
    /// Next counter value for `alloc_id`.
    pub next_id: u64,
}

impl Document {
    /// Create an empty document with default metadata and styles.
    pub fn new() -> Self {
        Self {
            metadata: DocumentMetadata::default(),
            sections: Vec::new(),
            styles: StyleSheet::new(),
            next_id: 1,
        }
    }

    /// Hand out the next stable id (monotone, per document).
    pub fn alloc_id(&mut self) -> NodeId {
        let id = NodeId::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Append a section to the document.
    pub fn push_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// Insert a block into one section.
    ///
    /// Panics when the section index or block index is out of bounds.
    /// Ids of existing blocks are left untouched.
    pub fn insert_block(&mut self, section_idx: usize, block_idx: usize, block: Block) {
        self.sections[section_idx].insert_block(block_idx, block);
    }

    /// Remove and return a block from one section.
    ///
    /// Panics when the section index or block index is out of bounds.
    /// Ids of the remaining blocks are left untouched.
    pub fn remove_block(&mut self, section_idx: usize, block_idx: usize) -> Block {
        self.sections[section_idx].remove_block(block_idx)
    }

    /// Split one section in two at a block boundary, joined by a break.
    ///
    /// Blocks from `block_idx` on move to a new section using
    /// `new_setup`; a `SectionBreak` marker carrying `new_setup` is
    /// appended to the first half. Returns the break id, or a static
    /// message when the section index, block index, or setup is bad.
    pub fn insert_section_break(
        &mut self,
        section_idx: usize,
        block_idx: usize,
        new_setup: PageSetup,
    ) -> Result<NodeId, &'static str> {
        if section_idx >= self.sections.len() {
            return Err("section index out of bounds");
        }
        if !new_setup.is_valid() {
            return Err("page setup is invalid");
        }
        if block_idx > self.sections[section_idx].blocks.len() {
            return Err("block index out of bounds");
        }
        let break_id = self.alloc_id();
        let new_section_id = self.alloc_id();
        let tail = self.sections[section_idx].blocks.split_off(block_idx);
        self.sections[section_idx]
            .push_block(Block::SectionBreak(SectionBreak::new(break_id, new_setup)));
        let mut new_section = Section::new(new_section_id, new_setup);
        for block in tail {
            new_section.push_block(block);
        }
        self.sections.insert(section_idx + 1, new_section);
        Ok(break_id)
    }

    /// Words across all sections.
    pub fn word_count(&self) -> usize {
        self.sections.iter().map(Section::word_count).sum()
    }

    /// Top-level blocks across all sections.
    pub fn block_count(&self) -> usize {
        self.sections.iter().map(Section::block_count).sum()
    }

    /// True when no section holds a block.
    pub fn is_empty(&self) -> bool {
        self.sections.iter().all(|s| s.blocks.is_empty())
    }
}

impl Default for Document {
    fn default() -> Self {
        let mut doc = Self::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        let para_id = doc.alloc_id();
        section.push_block(Block::Paragraph(Paragraph::new(para_id)));
        doc.push_section(section);
        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{CharacterStyle, TextRun};

    #[test]
    fn default_doc_shape() {
        let doc = Document::default();
        assert_eq!(doc.sections.len(), 1);
        assert_eq!(doc.block_count(), 1);
        assert_eq!(doc.word_count(), 0);
        assert!(!doc.is_empty());
    }

    #[test]
    fn empty_new_doc() {
        let doc = Document::new();
        assert!(doc.is_empty());
        assert_eq!(doc.word_count(), 0);
        assert_eq!(doc.block_count(), 0);
    }

    #[test]
    fn alloc_id_is_monotone() {
        let mut doc = Document::new();
        let a = doc.alloc_id();
        let b = doc.alloc_id();
        assert!(b.value() > a.value());
        assert_ne!(a, b);
    }

    #[test]
    fn word_count_spans_sections() {
        let mut doc = Document::new();
        for text in ["one two", "three"] {
            let section_id = doc.alloc_id();
            let mut section = Section::new(section_id, PageSetup::a4_portrait());
            let para_id = doc.alloc_id();
            let mut para = Paragraph::new(para_id);
            let run_id = doc.alloc_id();
            para.push_run(TextRun::new(run_id, text, CharacterStyle::default()));
            section.push_block(Block::Paragraph(para));
            doc.push_section(section);
        }
        assert_eq!(doc.word_count(), 3);
        assert_eq!(doc.block_count(), 2);
    }

    #[test]
    fn two_sections_mixed_orientation() {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        for text in ["one two", "three four"] {
            let para_id = doc.alloc_id();
            let mut para = Paragraph::new(para_id);
            let run_id = doc.alloc_id();
            para.push_run(TextRun::new(run_id, text, CharacterStyle::default()));
            section.push_block(Block::Paragraph(para));
        }
        doc.push_section(section);

        let break_id = doc
            .insert_section_break(0, 1, PageSetup::a4_landscape())
            .expect("valid split");
        assert_eq!(doc.sections.len(), 2);

        let portrait = PageSetup::a4_portrait();
        let landscape = PageSetup::a4_landscape();
        assert_eq!(doc.sections[0].page_setup, portrait);
        assert_eq!(doc.sections[1].page_setup, landscape);
        assert!((landscape.width_pt - portrait.height_pt).abs() <= 0.05);
        assert!((landscape.height_pt - portrait.width_pt).abs() <= 0.05);

        match doc.sections[0].blocks.last().expect("break marker") {
            Block::SectionBreak(section_break) => {
                assert_eq!(section_break.id, break_id);
                assert_eq!(section_break.page_setup, landscape);
            }
            other => panic!("expected section break, got {:?}", other),
        }
        assert_ne!(doc.sections[0].id, doc.sections[1].id);

        assert_eq!(doc.sections[0].block_count(), 2);
        assert_eq!(doc.sections[1].block_count(), 1);
        assert_eq!(doc.block_count(), 3);
        assert_eq!(doc.word_count(), 4);
    }

    #[test]
    fn insert_section_break_rejects_bad_input() {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        doc.push_section(Section::new(section_id, PageSetup::a4_portrait()));
        assert!(
            doc.insert_section_break(3, 0, PageSetup::a4_landscape())
                .is_err()
        );
        assert!(
            doc.insert_section_break(0, 9, PageSetup::a4_landscape())
                .is_err()
        );
        let bad_setup = PageSetup {
            width_pt: 0.0,
            height_pt: 0.0,
            ..PageSetup::a4_portrait()
        };
        assert!(doc.insert_section_break(0, 0, bad_setup).is_err());
        assert_eq!(doc.sections.len(), 1);
    }

    #[test]
    fn document_insert_delete_preserve_ids() {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        let mut first_ids = Vec::new();
        for text in ["a", "b", "c"] {
            let para_id = doc.alloc_id();
            first_ids.push(para_id);
            let mut para = Paragraph::new(para_id);
            let run_id = doc.alloc_id();
            para.push_run(TextRun::new(run_id, text, CharacterStyle::default()));
            section.push_block(Block::Paragraph(para));
        }
        doc.push_section(section);

        let max_before = first_ids.iter().map(|id| id.value()).max().expect("ids");
        let fresh = doc.alloc_id();
        assert!(fresh.value() > max_before);
        let mut extra = Paragraph::new(fresh);
        let run_id = doc.alloc_id();
        extra.push_run(TextRun::new(run_id, "x", CharacterStyle::default()));
        doc.insert_block(0, 1, Block::Paragraph(extra));

        let ids: Vec<Option<NodeId>> = doc.sections[0].blocks.iter().map(Block::id).collect();
        assert_eq!(
            ids,
            vec![
                Some(first_ids[0]),
                Some(fresh),
                Some(first_ids[1]),
                Some(first_ids[2]),
            ]
        );

        let removed = doc.remove_block(0, 2);
        assert_eq!(removed.id(), Some(first_ids[1]));
        let rest: Vec<Option<NodeId>> = doc.sections[0].blocks.iter().map(Block::id).collect();
        assert_eq!(
            rest,
            vec![Some(first_ids[0]), Some(fresh), Some(first_ids[2])]
        );
        assert!(doc.alloc_id().value() > fresh.value());
    }
}
