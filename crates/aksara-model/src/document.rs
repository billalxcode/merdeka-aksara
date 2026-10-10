//! The document root: metadata, sections, styles, and id counter.
//!
//! The document owns a monotone id counter. Call `alloc_id` for every
//! new node so ids stay unique inside one document.

use crate::block::Block;
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
}
