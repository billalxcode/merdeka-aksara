//! Transaction: one labeled batch of commands plus dispatch.
//!
//! A transaction groups commands into a single logical unit: it is
//! applied in order and committed to undo history as one entry.

use crate::command::EditCommand;
use aksara_model::Document;

/// Labeled batch of edit commands applied as one logical unit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Transaction {
    /// Human-readable label (shown in undo menus, logs).
    pub label: String,
    /// Commands applied in order.
    pub ops: Vec<EditCommand>,
}

impl Transaction {
    /// Create an empty transaction with a label.
    pub fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            ops: Vec::new(),
        }
    }

    /// Append one command to the batch.
    pub fn push(&mut self, op: EditCommand) {
        self.ops.push(op);
    }

    /// Apply every command in order; stop at the first error.
    pub fn apply(&self, doc: &mut Document) -> Result<(), &'static str> {
        for op in &self.ops {
            op.apply(doc)?;
        }
        Ok(())
    }

    /// True when the batch holds no commands.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

/// Apply one transaction to a document (single logical dispatch).
pub fn dispatch(doc: &mut Document, tx: &Transaction) -> Result<(), &'static str> {
    tx.apply(doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aksara_model::{Block, NodeId, PageSetup, Section, text::Paragraph};

    fn doc_with_para() -> (Document, NodeId) {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        let para_id = doc.alloc_id();
        section.push_block(Block::Paragraph(Paragraph::new(para_id)));
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
        let (mut doc, para) = doc_with_para();
        let mut tx = Transaction::new("type abc");
        for (offset, ch) in [(0, "a"), (1, "b"), (2, "c")] {
            tx.push(EditCommand::InsertText {
                para,
                offset,
                text: String::from(ch),
            });
        }
        dispatch(&mut doc, &tx).expect("tx ok");
        assert_eq!(para_text(&doc, para), "abc");
    }

    #[test]
    fn tx_applies_in_order() {
        let (mut doc, para) = doc_with_para();
        let mut tx = Transaction::new("order");
        tx.push(EditCommand::InsertText {
            para,
            offset: 0,
            text: String::from("b"),
        });
        tx.push(EditCommand::InsertText {
            para,
            offset: 0,
            text: String::from("a"),
        });
        dispatch(&mut doc, &tx).expect("tx ok");
        assert_eq!(para_text(&doc, para), "ab");
    }

    #[test]
    fn empty_tx_is_noop_ok() {
        let (mut doc, para) = doc_with_para();
        let tx = Transaction::new("empty");
        assert!(tx.is_empty());
        dispatch(&mut doc, &tx).expect("empty tx ok");
        assert_eq!(para_text(&doc, para), "");
    }

    #[test]
    fn dispatch_bad_para_id_errs() {
        let (mut doc, para) = doc_with_para();
        let before = doc.clone();
        let mut tx = Transaction::new("bad");
        tx.push(EditCommand::DeleteRange {
            para: NodeId::new(999_999),
            start: 0,
            len: 1,
        });
        assert!(dispatch(&mut doc, &tx).is_err());
        assert_eq!(para_text(&doc, para), "");
        assert_eq!(doc.next_id, before.next_id);
    }
}
