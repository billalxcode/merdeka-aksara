//! Text content: character style, paragraph style, runs, paragraphs.
//!
//! Types here are intentionally separate from the legacy
//! `apps/aksara` domain styles. Unification is a later phase;
//! this crate must not import from the app.

use crate::id::NodeId;

/// Horizontal alignment of a paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Alignment {
    /// Flush left, ragged right.
    #[default]
    Left,
    /// Centered lines.
    Center,
    /// Flush right, ragged left.
    Right,
    /// Flush on both sides.
    Justify,
}

/// Inline character formatting for a single run.
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterStyle {
    /// Bold face.
    pub bold: bool,
    /// Italic face.
    pub italic: bool,
    /// Underlined text.
    pub underline: bool,
    /// Font size in points.
    pub font_size_pt: f32,
    /// Font family name.
    pub font_family: String,
}

impl Default for CharacterStyle {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            font_size_pt: 12.0,
            font_family: String::from("Inter"),
        }
    }
}

/// Block-level paragraph formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphStyle {
    /// Horizontal alignment.
    pub alignment: Alignment,
    /// Extra space before the paragraph in points.
    pub space_before_pt: f32,
    /// Extra space after the paragraph in points.
    pub space_after_pt: f32,
    /// Line height multiplier (1.0 is single spacing).
    pub line_spacing: f32,
    /// First-line indent in points.
    pub indent_first_pt: f32,
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self {
            alignment: Alignment::Left,
            space_before_pt: 0.0,
            space_after_pt: 0.0,
            line_spacing: 1.0,
            indent_first_pt: 0.0,
        }
    }
}

/// A run of text sharing one character style.
#[derive(Debug, Clone, PartialEq)]
pub struct TextRun {
    /// Stable node id.
    pub id: NodeId,
    /// Raw text of the run.
    pub text: String,
    /// Shared style of the whole run.
    pub style: CharacterStyle,
}

impl TextRun {
    /// Create a run with the given id, text, and style.
    pub fn new(id: NodeId, text: &str, style: CharacterStyle) -> Self {
        Self {
            id,
            text: String::from(text),
            style,
        }
    }

    /// Words in this run, split on whitespace.
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    /// Characters in this run (chars, not bytes).
    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }
}

/// A paragraph: ordered runs plus one paragraph style.
#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    /// Stable node id.
    pub id: NodeId,
    /// Ordered inline runs.
    pub runs: Vec<TextRun>,
    /// Block-level style.
    pub style: ParagraphStyle,
}

impl Paragraph {
    /// Create an empty paragraph with the default style.
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            runs: Vec::new(),
            style: ParagraphStyle::default(),
        }
    }

    /// Append a run to the paragraph.
    pub fn push_run(&mut self, run: TextRun) {
        self.runs.push(run);
    }

    /// Concatenated text of all runs.
    pub fn text(&self) -> String {
        self.runs
            .iter()
            .map(|run| run.text.as_str())
            .collect::<String>()
    }

    /// Words across all runs.
    pub fn word_count(&self) -> usize {
        self.runs.iter().map(TextRun::word_count).sum()
    }

    /// True when every run is empty or there are no runs.
    pub fn is_empty(&self) -> bool {
        self.runs.iter().all(|run| run.text.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_spans_runs() {
        let mut para = Paragraph::new(NodeId::new(1));
        para.push_run(TextRun::new(
            NodeId::new(2),
            "hello ",
            CharacterStyle::default(),
        ));
        para.push_run(TextRun::new(
            NodeId::new(3),
            "world",
            CharacterStyle::default(),
        ));
        assert_eq!(para.word_count(), 2);
        assert_eq!(para.text(), "hello world");
        assert!(!para.is_empty());
    }

    #[test]
    fn empty_paragraph_counts_zero() {
        let para = Paragraph::new(NodeId::new(1));
        assert!(para.is_empty());
        assert_eq!(para.word_count(), 0);
        assert_eq!(para.text(), "");
    }

    #[test]
    fn default_styles_are_sane() {
        let ch = CharacterStyle::default();
        assert_eq!(ch.font_family, "Inter");
        assert_eq!(ch.font_size_pt, 12.0);
        assert!(!ch.bold && !ch.italic && !ch.underline);
        let ps = ParagraphStyle::default();
        assert_eq!(ps.alignment, Alignment::Left);
        assert_eq!(ps.line_spacing, 1.0);
    }

    #[test]
    fn char_count_uses_chars_not_bytes() {
        let run = TextRun::new(NodeId::new(1), "ab", CharacterStyle::default());
        assert_eq!(run.char_count(), 2);
        assert_eq!(run.word_count(), 1);
    }
}
