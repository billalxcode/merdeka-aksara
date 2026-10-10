//! Paragraph layout: lines plus offsets and total height.

use crate::line_break::break_greedy;
use crate::measure::TextMeasurer;
use aksara_model::{Alignment, Paragraph};

/// One laid-out line: text, measured width, x offset in points.
#[derive(Debug, Clone, PartialEq)]
pub struct LaidLine {
    /// Line text (separator spaces already resolved).
    pub text: String,
    /// Measured width in points.
    pub width_pt: f64,
    /// X offset from the content-area left edge in points.
    pub offset_x_pt: f64,
}

/// One laid-out paragraph: lines plus total height in points.
#[derive(Debug, Clone, PartialEq)]
pub struct LaidParagraph {
    /// Lines in reading order.
    pub lines: Vec<LaidLine>,
    /// Total height: space_before + lines + space_after.
    pub height_pt: f64,
    /// Uniform height of one line (size * spacing * ratio).
    pub line_height_pt: f64,
}

/// Lay out `para` inside `max_width_pt`.
///
/// Style note: runs are joined via `para.text()` and measured with
/// the first run's size/bold/italic when styles mix. Alignment sets
/// `offset_x_pt` (Justify keeps 0 with last line ragged left);
/// `indent_first_pt` shifts only the first line.
pub fn layout_paragraph(
    para: &Paragraph,
    measurer: &dyn TextMeasurer,
    max_width_pt: f64,
) -> LaidParagraph {
    let (size_pt, bold, italic) = para
        .runs
        .first()
        .map(|run| (run.style.font_size_pt, run.style.bold, run.style.italic))
        .unwrap_or((12.0, false, false));
    let line_h = measurer.line_height_pt(size_pt, para.style.line_spacing);
    let texts = break_greedy(&para.text(), measurer, size_pt, bold, italic, max_width_pt);
    let mut lines = Vec::with_capacity(texts.len());
    for (idx, text) in texts.iter().enumerate() {
        let width = measurer.width_pt(text, size_pt, bold, italic);
        let rest = (max_width_pt - width).max(0.0);
        let base = match para.style.alignment {
            Alignment::Left | Alignment::Justify => 0.0,
            Alignment::Center => rest / 2.0,
            Alignment::Right => rest,
        };
        let indent = if idx == 0 {
            f64::from(para.style.indent_first_pt)
        } else {
            0.0
        };
        lines.push(LaidLine {
            text: text.clone(),
            width_pt: width,
            offset_x_pt: base + indent,
        });
    }
    let height = f64::from(para.style.space_before_pt)
        + line_h * lines.len() as f64
        + f64::from(para.style.space_after_pt);
    LaidParagraph {
        lines,
        height_pt: height,
        line_height_pt: line_h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measure::FixedMeasurer;
    use aksara_model::{CharacterStyle, NodeId, ParagraphStyle, TextRun};

    fn para_with(text: &str) -> Paragraph {
        let mut para = Paragraph::new(NodeId::new(1));
        para.push_run(TextRun::new(
            NodeId::new(2),
            text,
            CharacterStyle::default(),
        ));
        para
    }

    #[test]
    fn single_line_height_matches_line() {
        let m = FixedMeasurer::default();
        let laid = layout_paragraph(&para_with("hi"), &m, 451.28);
        assert_eq!(laid.lines.len(), 1);
        assert!((laid.height_pt - 14.4).abs() < 1e-9);
    }

    #[test]
    fn space_before_after_add_to_total() {
        let m = FixedMeasurer::default();
        let mut para = para_with("hi");
        para.style.space_before_pt = 6.0;
        para.style.space_after_pt = 6.0;
        let laid = layout_paragraph(&para, &m, 451.28);
        assert!((laid.height_pt - (14.4 + 12.0)).abs() < 1e-9);
    }

    #[test]
    fn center_offset_is_half_remainder() {
        let m = FixedMeasurer::default();
        let mut para = para_with("hi");
        para.style.alignment = Alignment::Center;
        let laid = layout_paragraph(&para, &m, 100.0);
        let width = m.width_pt("hi", 12.0, false, false);
        assert!((laid.lines[0].offset_x_pt - (100.0 - width) / 2.0).abs() < 1e-9);
    }

    #[test]
    fn indent_only_first_line() {
        let m = FixedMeasurer::default();
        let mut para = para_with("satu dua tiga empat lima enam tujuh");
        para.style.indent_first_pt = 18.0;
        let laid = layout_paragraph(&para, &m, 72.0);
        assert!(laid.lines.len() > 1);
        assert!((laid.lines[0].offset_x_pt - 18.0).abs() < 1e-9);
        assert_eq!(laid.lines[1].offset_x_pt, 0.0);
    }

    #[test]
    fn empty_paragraph_height_is_space_only() {
        let m = FixedMeasurer::default();
        let mut para = Paragraph::new(NodeId::new(1));
        para.style = ParagraphStyle {
            space_before_pt: 3.0,
            space_after_pt: 4.0,
            ..ParagraphStyle::default()
        };
        let laid = layout_paragraph(&para, &m, 100.0);
        assert!(laid.lines.is_empty());
        assert!((laid.height_pt - 7.0).abs() < 1e-9);
    }

    #[test]
    fn right_offset_pushes_to_edge() {
        let m = FixedMeasurer::default();
        let mut para = para_with("hi");
        para.style.alignment = Alignment::Right;
        let laid = layout_paragraph(&para, &m, 100.0);
        let width = m.width_pt("hi", 12.0, false, false);
        assert!((laid.lines[0].offset_x_pt - (100.0 - width)).abs() < 1e-9);
    }
}
