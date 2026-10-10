//! Block layout: ordered blocks to laid blocks with gaps/breaks.

use crate::measure::TextMeasurer;
use crate::paragraph::{LaidParagraph, layout_paragraph};
use aksara_model::Block;

/// One laid-out block.
#[derive(Debug, Clone, PartialEq)]
pub enum LaidBlock {
    /// Laid-out paragraph.
    Paragraph(LaidParagraph),
    /// Vertical gap (paragraph spacing between blocks) in points.
    Gap {
        /// Gap height in points.
        height_pt: f64,
    },
    /// Page or section boundary (no height, forces a new page).
    Break,
}

/// Lay out `blocks` in order.
///
/// Paragraphs use `layout_paragraph`; tables and images are
/// deterministic 0.0-height placeholders for this phase (TODO: real
/// table/image layout); PageBreak and SectionBreak become `Break`.
/// A `para_gap_pt` gap is inserted between two adjacent paragraphs.
pub fn layout_blocks(
    blocks: &[Block],
    measurer: &dyn TextMeasurer,
    max_width_pt: f64,
    para_gap_pt: f64,
) -> Vec<LaidBlock> {
    let mut out = Vec::new();
    let mut prev_was_para = false;
    for block in blocks {
        match block {
            Block::Paragraph(para) => {
                if prev_was_para && para_gap_pt > 0.0 {
                    out.push(LaidBlock::Gap {
                        height_pt: para_gap_pt,
                    });
                }
                out.push(LaidBlock::Paragraph(layout_paragraph(
                    para,
                    measurer,
                    max_width_pt,
                )));
                prev_was_para = true;
            }
            Block::Table(_) | Block::Image(_) => {
                // TODO: real table/image measurement; placeholder only.
                out.push(LaidBlock::Gap { height_pt: 0.0 });
                prev_was_para = false;
            }
            Block::PageBreak | Block::SectionBreak(_) => {
                out.push(LaidBlock::Break);
                prev_was_para = false;
            }
        }
    }
    out
}

/// Total stacked height of laid blocks (breaks count zero).
pub fn laid_height(blocks: &[LaidBlock]) -> f64 {
    blocks
        .iter()
        .map(|b| match b {
            LaidBlock::Paragraph(p) => p.height_pt,
            LaidBlock::Gap { height_pt } => *height_pt,
            LaidBlock::Break => 0.0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measure::FixedMeasurer;
    use aksara_model::{CharacterStyle, Image, NodeId, Paragraph, TextRun};

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
    fn order_is_preserved_with_gap() {
        let m = FixedMeasurer::default();
        let blocks = vec![para_block(1, "a"), para_block(2, "b")];
        let laid = layout_blocks(&blocks, &m, 451.28, 6.0);
        assert_eq!(laid.len(), 3);
        assert!(matches!(laid[1], LaidBlock::Gap { .. }));
    }

    #[test]
    fn page_break_becomes_break() {
        let m = FixedMeasurer::default();
        let blocks = vec![para_block(1, "a"), Block::PageBreak, para_block(2, "b")];
        let laid = layout_blocks(&blocks, &m, 451.28, 0.0);
        assert_eq!(laid.len(), 3);
        assert!(matches!(laid[1], LaidBlock::Break));
    }

    #[test]
    fn image_and_table_do_not_panic() {
        let m = FixedMeasurer::default();
        let table = aksara_model::Table::new(NodeId::new(9), 1, 1);
        let blocks = vec![
            Block::Image(Image::new(NodeId::new(7), "a.png", 10.0, 10.0)),
            Block::Table(table),
        ];
        let laid = layout_blocks(&blocks, &m, 451.28, 0.0);
        assert_eq!(laid.len(), 2);
        assert_eq!(laid_height(&laid), 0.0);
    }

    #[test]
    fn total_height_sums_parts() {
        let m = FixedMeasurer::default();
        let blocks = vec![para_block(1, "a"), para_block(2, "b")];
        let laid = layout_blocks(&blocks, &m, 451.28, 6.0);
        // Two single lines at 14.4 pt plus one 6.0 gap.
        assert!((laid_height(&laid) - (14.4 * 2.0 + 6.0)).abs() < 1e-9);
    }
}
