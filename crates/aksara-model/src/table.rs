//! Tables: rows of cells holding recursive blocks.
//!
//! Cell content is `Vec<Block>`, so tables nest paragraphs, images,
//! page breaks, and even other tables.

use crate::block::Block;
use crate::id::NodeId;

/// Maximum table rows (matches the legacy app helper).
pub const MAX_ROWS: usize = 10;

/// Maximum table columns (matches the legacy app helper).
pub const MAX_COLS: usize = 8;

/// One cell holding a list of blocks.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableCell {
    /// Blocks inside the cell.
    pub blocks: Vec<Block>,
}

impl TableCell {
    /// Append a block to the cell.
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Words across all blocks in the cell.
    pub fn word_count(&self) -> usize {
        self.blocks.iter().map(Block::word_count).sum()
    }
}

/// One table row: an ordered list of cells.
#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    /// Cells in the row.
    pub cells: Vec<TableCell>,
}

/// Table block with a fixed grid of cells.
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// Stable node id.
    pub id: NodeId,
    /// Grid rows.
    pub rows: Vec<TableRow>,
}

impl Table {
    /// Create an empty `rows` x `cols` grid.
    ///
    /// Dimensions clamp to `1..=MAX_ROWS` rows and `1..=MAX_COLS`
    /// columns, matching the legacy `Dokumen::tabel_teks` helper.
    pub fn new(id: NodeId, rows: usize, cols: usize) -> Self {
        let rows = rows.clamp(1, MAX_ROWS);
        let cols = cols.clamp(1, MAX_COLS);
        let grid = (0..rows)
            .map(|_| TableRow {
                cells: (0..cols).map(|_| TableCell::default()).collect(),
            })
            .collect();
        Self { id, rows: grid }
    }

    /// Number of rows in the grid.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Number of columns (cells in the first row, or zero).
    pub fn col_count(&self) -> usize {
        self.rows.first().map_or(0, |row| row.cells.len())
    }

    /// Words across all cells, recursing into nested blocks.
    pub fn word_count(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.cells.iter())
            .map(TableCell::word_count)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{CharacterStyle, Paragraph, TextRun};

    fn para(id: u64, text: &str) -> Paragraph {
        let mut p = Paragraph::new(NodeId::new(id));
        p.push_run(TextRun::new(
            NodeId::new(id + 500),
            text,
            CharacterStyle::default(),
        ));
        p
    }

    #[test]
    fn clamps_dimensions() {
        let table = Table::new(NodeId::new(1), 0, 99);
        assert_eq!(table.row_count(), 1);
        assert_eq!(table.col_count(), MAX_COLS);
        let big = Table::new(NodeId::new(2), 99, 99);
        assert_eq!(big.row_count(), MAX_ROWS);
        assert_eq!(big.col_count(), MAX_COLS);
    }

    #[test]
    fn nested_cell_word_count() {
        let mut table = Table::new(NodeId::new(1), 2, 2);
        table.rows[0].cells[0].push_block(Block::Paragraph(para(10, "one two")));
        table.rows[1].cells[1].push_block(Block::Paragraph(para(11, "three")));
        assert_eq!(table.word_count(), 3);
        assert_eq!(Block::Table(table).word_count(), 3);
    }

    #[test]
    fn empty_table_counts_zero() {
        let table = Table::new(NodeId::new(1), 2, 2);
        assert_eq!(table.word_count(), 0);
    }
}
