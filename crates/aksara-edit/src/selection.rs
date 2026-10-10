//! Selection anchor + cursor for one document.
//!
//! A position is a paragraph block id plus a char offset relative to
//! the concatenated `Paragraph::text()`. Cross-block ranges are not
//! normalized across sections in this phase.

use aksara_model::NodeId;

/// Caret or range endpoint: paragraph block plus char offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocPosition {
    /// Id of the paragraph block.
    pub block: NodeId,
    /// Char offset into the paragraph text.
    pub offset: usize,
}

impl DocPosition {
    /// Create a position from a block id and char offset.
    pub fn new(block: NodeId, offset: usize) -> Self {
        Self { block, offset }
    }
}

/// Anchor + cursor selection; equal endpoints mean a caret.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Fixed endpoint of the selection.
    pub anchor: DocPosition,
    /// Moving endpoint of the selection.
    pub cursor: DocPosition,
}

impl Selection {
    /// Collapsed selection (caret) at one position.
    pub fn caret(pos: DocPosition) -> Self {
        Self {
            anchor: pos,
            cursor: pos,
        }
    }

    /// Selection spanning two endpoints, order kept as given.
    pub fn range(anchor: DocPosition, cursor: DocPosition) -> Self {
        Self { anchor, cursor }
    }

    /// True when anchor equals cursor (no selected range).
    pub fn is_caret(&self) -> bool {
        self.anchor == self.cursor
    }

    /// Endpoints ordered by offset: `(start, end)`.
    ///
    /// Only meaningful when both endpoints share one block; the
    /// smaller offset comes first.
    pub fn ordered(&self) -> (DocPosition, DocPosition) {
        if self.anchor.offset <= self.cursor.offset {
            (self.anchor, self.cursor)
        } else {
            (self.cursor, self.anchor)
        }
    }

    /// Collapse to the cursor endpoint.
    pub fn collapse(&self) -> Self {
        Self::caret(self.cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caret_is_empty() {
        let pos = DocPosition::new(NodeId::new(1), 3);
        let sel = Selection::caret(pos);
        assert_eq!(sel.anchor, sel.cursor);
        assert!(sel.is_caret());
    }

    #[test]
    fn range_orders_anchor_cursor() {
        let anchor = DocPosition::new(NodeId::new(1), 5);
        let cursor = DocPosition::new(NodeId::new(1), 2);
        let sel = Selection::range(anchor, cursor);
        assert!(!sel.is_caret());
        let (start, end) = sel.ordered();
        assert_eq!(start.offset, 2);
        assert_eq!(end.offset, 5);
    }

    #[test]
    fn collapse_to_cursor() {
        let anchor = DocPosition::new(NodeId::new(1), 1);
        let cursor = DocPosition::new(NodeId::new(1), 4);
        let sel = Selection::range(anchor, cursor);
        let collapsed = sel.collapse();
        assert_eq!(collapsed, Selection::caret(cursor));
        assert!(collapsed.is_caret());
    }
}
