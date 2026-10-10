//! Undo/redo manager: snapshot pairs per transaction.
//!
//! Each committed transaction stores `(before, after)` document
//! snapshots. Undo restores `before`, redo restores `after`. New
//! commits clear the redo stack. Capacity is bounded (`cap`,
//! default 100); oldest entries drop from the front.
//!
//! Typical use:
//!
//! ```text
//! let before = doc.clone();
//! tx.apply(&mut doc)?;
//! mgr.commit(before, doc.clone());
//! ```

use aksara_model::Document;

/// Default history capacity.
pub const DEFAULT_CAP: usize = 100;

/// Per-transaction undo/redo history with a bounded depth.
#[derive(Debug, Clone)]
pub struct UndoRedoManager {
    undo: Vec<(Document, Document)>,
    redo: Vec<(Document, Document)>,
    cap: usize,
}

impl UndoRedoManager {
    /// Create a manager holding at most `cap` entries.
    pub fn new(cap: usize) -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            cap,
        }
    }

    /// History depth limit.
    pub fn cap(&self) -> usize {
        self.cap
    }

    /// Entries currently available for undo.
    pub fn undo_len(&self) -> usize {
        self.undo.len()
    }

    /// Entries currently available for redo.
    pub fn redo_len(&self) -> usize {
        self.redo.len()
    }

    /// True when an undo step is available.
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    /// True when a redo step is available.
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Record one committed transaction; clears the redo stack.
    pub fn commit(&mut self, before: Document, after: Document) {
        self.undo.push((before, after));
        while self.undo.len() > self.cap {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    /// Restore the `before` snapshot of the latest entry.
    ///
    /// Returns false (document untouched) when history is empty.
    pub fn undo(&mut self, doc: &mut Document) -> bool {
        let Some((before, after)) = self.undo.pop() else {
            return false;
        };
        *doc = before.clone();
        self.redo.push((before, after));
        true
    }

    /// Restore the `after` snapshot of the latest undone entry.
    ///
    /// Returns false (document untouched) when history is empty.
    pub fn redo(&mut self, doc: &mut Document) -> bool {
        let Some((before, after)) = self.redo.pop() else {
            return false;
        };
        *doc = after.clone();
        self.undo.push((before, after));
        while self.undo.len() > self.cap {
            self.undo.remove(0);
        }
        true
    }
}

impl Default for UndoRedoManager {
    fn default() -> Self {
        Self::new(DEFAULT_CAP)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EditCommand, Transaction};
    use aksara_model::{Block, NodeId, PageSetup, Section, text::Paragraph};

    fn doc_with_text(text: &str) -> (Document, NodeId) {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        let para_id = doc.alloc_id();
        let mut para = Paragraph::new(para_id);
        if !text.is_empty() {
            let run_id = doc.alloc_id();
            para.push_run(aksara_model::TextRun::new(
                run_id,
                text,
                aksara_model::CharacterStyle::default(),
            ));
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

    fn insert_tx(para: NodeId, offset: usize, text: &str) -> Transaction {
        let mut tx = Transaction::new("insert");
        tx.push(EditCommand::InsertText {
            para,
            offset,
            text: String::from(text),
        });
        tx
    }

    fn apply_and_commit(mgr: &mut UndoRedoManager, doc: &mut Document, tx: &Transaction) {
        let before = doc.clone();
        tx.apply(doc).expect("tx ok");
        mgr.commit(before, doc.clone());
    }

    #[test]
    fn undo_restores_before() {
        let (mut doc, para) = doc_with_text("ab");
        let mut mgr = UndoRedoManager::default();
        assert!(!mgr.can_undo());
        apply_and_commit(&mut mgr, &mut doc, &insert_tx(para, 2, "c"));
        assert_eq!(para_text(&doc, para), "abc");
        assert!(mgr.can_undo());
        assert!(mgr.undo(&mut doc));
        assert_eq!(para_text(&doc, para), "ab");
        assert!(!mgr.can_undo());
        assert!(mgr.can_redo());
    }

    #[test]
    fn undo_twice_then_redo_once() {
        let (mut doc, para) = doc_with_text("ab");
        let mut mgr = UndoRedoManager::default();
        apply_and_commit(&mut mgr, &mut doc, &insert_tx(para, 2, "c"));
        apply_and_commit(&mut mgr, &mut doc, &insert_tx(para, 3, "d"));
        assert_eq!(para_text(&doc, para), "abcd");
        assert!(mgr.undo(&mut doc));
        assert!(mgr.undo(&mut doc));
        assert_eq!(para_text(&doc, para), "ab");
        assert!(!mgr.can_undo());
        assert!(mgr.redo(&mut doc));
        assert_eq!(para_text(&doc, para), "abc");
    }

    #[test]
    fn new_commit_clears_redo() {
        let (mut doc, para) = doc_with_text("ab");
        let mut mgr = UndoRedoManager::default();
        apply_and_commit(&mut mgr, &mut doc, &insert_tx(para, 2, "c"));
        assert!(mgr.undo(&mut doc));
        assert!(mgr.can_redo());
        apply_and_commit(&mut mgr, &mut doc, &insert_tx(para, 2, "X"));
        assert_eq!(para_text(&doc, para), "abX");
        assert!(!mgr.can_redo());
        assert!(!mgr.redo(&mut doc));
    }
}
