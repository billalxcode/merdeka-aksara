//! Edit commands applied to a `Document` without new model APIs.
//!
//! Offsets and lengths are char indices relative to the concatenated
//! `Paragraph::text()` of one single-run text paragraph. Multi-run
//! style splits are out of scope: edits splice `runs[0].text`.

use aksara_model::{Block, CharacterStyle, Document, NodeId, Paragraph, TextRun};

/// One atomic document edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditCommand {
    /// Insert text at a char offset in one paragraph.
    InsertText {
        /// Target paragraph block id.
        para: NodeId,
        /// Char offset where insertion starts.
        offset: usize,
        /// Text to insert.
        text: String,
    },
    /// Delete `len` chars starting at char offset `start`.
    DeleteRange {
        /// Target paragraph block id.
        para: NodeId,
        /// Char offset where deletion starts.
        start: usize,
        /// Number of chars to delete.
        len: usize,
    },
}

impl EditCommand {
    /// Apply this command to a document.
    ///
    /// Finds the paragraph by scanning `doc.sections[*].blocks[*]`
    /// for `Block::Paragraph` with a matching `para.id`. Returns
    /// `Err` and leaves the document unchanged when the id is
    /// unknown.
    pub fn apply(&self, doc: &mut Document) -> Result<(), &'static str> {
        match self {
            Self::InsertText { para, offset, text } => {
                if !para_exists(doc, *para) {
                    return Err("unknown paragraph id");
                }
                if para_is_empty(doc, *para) {
                    let run_id = doc.alloc_id();
                    let target = find_para_mut(doc, *para).ok_or("unknown paragraph id")?;
                    target.push_run(TextRun::new(run_id, "", CharacterStyle::default()));
                }
                let target = find_para_mut(doc, *para).ok_or("unknown paragraph id")?;
                let byte = char_to_byte(&target.runs[0].text, *offset);
                target.runs[0].text.insert_str(byte, text);
                Ok(())
            }
            Self::DeleteRange { para, start, len } => {
                let target = find_para_mut(doc, *para).ok_or("unknown paragraph id")?;
                delete_from_para(target, *start, *len);
                Ok(())
            }
        }
    }
}

/// Immutable existence check across all sections.
fn para_exists(doc: &Document, para: NodeId) -> bool {
    doc.sections.iter().any(|section| {
        section.blocks.iter().any(|block| match block {
            Block::Paragraph(p) => p.id == para,
            _ => false,
        })
    })
}

/// True when the paragraph exists and holds no runs.
fn para_is_empty(doc: &Document, para: NodeId) -> bool {
    doc.sections
        .iter()
        .flat_map(|section| section.blocks.iter())
        .find_map(|block| match block {
            Block::Paragraph(p) if p.id == para => Some(p.runs.is_empty()),
            _ => None,
        })
        .unwrap_or(false)
}

/// Mutable paragraph lookup across all sections.
fn find_para_mut(doc: &mut Document, para: NodeId) -> Option<&mut Paragraph> {
    for section in &mut doc.sections {
        for block in &mut section.blocks {
            if let Block::Paragraph(p) = block
                && p.id == para
            {
                return Some(p);
            }
        }
    }
    None
}

/// Convert a char offset to a byte offset (saturating at text end).
fn char_to_byte(text: &str, offset: usize) -> usize {
    if offset == 0 {
        return 0;
    }
    text.char_indices()
        .nth(offset)
        .map(|(byte, _)| byte)
        .unwrap_or_else(|| text.len())
}

/// Delete `len` chars at char offset `start`, clamped to text end.
fn delete_from_para(para: &mut Paragraph, start: usize, len: usize) {
    if para.runs.is_empty() || len == 0 {
        return;
    }
    let current: String = para.runs[0].text.clone();
    let total: usize = current.chars().count();
    if start >= total {
        return;
    }
    let end = start.saturating_add(len).min(total);
    let start_byte = char_to_byte(&current, start);
    let end_byte = char_to_byte(&current, end);
    para.runs[0].text.drain(start_byte..end_byte);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aksara_model::{PageSetup, Section};

    fn doc_with_para(text: &str) -> (Document, NodeId) {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        let para_id = doc.alloc_id();
        let mut para = Paragraph::new(para_id);
        if !text.is_empty() {
            let run_id = doc.alloc_id();
            para.push_run(TextRun::new(run_id, text, CharacterStyle::default()));
        }
        section.push_block(Block::Paragraph(para));
        doc.push_section(section);
        (doc, para_id)
    }

    fn para_text(doc: &Document, para: NodeId) -> String {
        for section in &doc.sections {
            for block in &section.blocks {
                if let Block::Paragraph(p) = block
                    && p.id == para
                {
                    return p.text();
                }
            }
        }
        panic!("paragraph missing in test helper");
    }

    #[test]
    fn type_three_chars_one_tx() {
        let (mut doc, para) = doc_with_para("");
        for (offset, ch) in [(0, "a"), (1, "b"), (2, "c")] {
            EditCommand::InsertText {
                para,
                offset,
                text: String::from(ch),
            }
            .apply(&mut doc)
            .expect("insert ok");
        }
        assert_eq!(para_text(&doc, para), "abc");
    }

    #[test]
    fn tx_applies_in_order() {
        let (mut doc, para) = doc_with_para("abc");
        EditCommand::DeleteRange {
            para,
            start: 1,
            len: 1,
        }
        .apply(&mut doc)
        .expect("delete ok");
        EditCommand::InsertText {
            para,
            offset: 1,
            text: String::from("X"),
        }
        .apply(&mut doc)
        .expect("insert ok");
        assert_eq!(para_text(&doc, para), "aXc");
    }

    #[test]
    fn empty_tx_is_noop_ok() {
        let (mut doc, para) = doc_with_para("hi");
        EditCommand::DeleteRange {
            para,
            start: 0,
            len: 0,
        }
        .apply(&mut doc)
        .expect("zero delete ok");
        EditCommand::DeleteRange {
            para,
            start: 99,
            len: 5,
        }
        .apply(&mut doc)
        .expect("clamped delete ok");
        assert_eq!(para_text(&doc, para), "hi");
    }

    #[test]
    fn dispatch_bad_para_id_errs() {
        let (mut doc, _) = doc_with_para("hi");
        let before = doc.clone();
        let bad = EditCommand::InsertText {
            para: NodeId::new(999_999),
            offset: 0,
            text: String::from("x"),
        };
        assert!(bad.apply(&mut doc).is_err());
        assert_eq!(doc, before);
    }

    #[test]
    fn insert_counts_combining_mark_as_chars() {
        let (mut doc, para) = doc_with_para("");
        EditCommand::InsertText {
            para,
            offset: 0,
            text: String::from("e\u{0301}"),
        }
        .apply(&mut doc)
        .expect("insert ok");
        EditCommand::InsertText {
            para,
            offset: 2,
            text: String::from("x"),
        }
        .apply(&mut doc)
        .expect("insert ok");
        assert_eq!(para_text(&doc, para), "e\u{0301}x");
    }
}
