//! Named styles: a minimal std-only style sheet.
//!
//! Effective styles resolve in three levels: built-in default, then
//! the named style, then direct formatting. Direct wins over named,
//! named wins over default.

use crate::text::{Alignment, CharacterStyle, ParagraphStyle};
use std::collections::HashMap;

/// Name of the built-in default style.
pub const NORMAL_STYLE: &str = "Normal";

/// A named pair of paragraph and character formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedStyle {
    /// Style name used for lookup.
    pub name: String,
    /// Paragraph-level formatting.
    pub paragraph: ParagraphStyle,
    /// Character-level formatting.
    pub character: CharacterStyle,
}

impl NamedStyle {
    /// Create a named style from both levels.
    pub fn new(name: &str, paragraph: ParagraphStyle, character: CharacterStyle) -> Self {
        Self {
            name: String::from(name),
            paragraph,
            character,
        }
    }

    /// The built-in default style.
    pub fn normal() -> Self {
        Self::new(
            NORMAL_STYLE,
            ParagraphStyle::default(),
            CharacterStyle::default(),
        )
    }
}

/// Direct character formatting that overrides only the fields it sets.
///
/// Every field is optional: `None` keeps the value coming from the
/// default or named style, `Some` replaces it. This struct is purely
/// additive; the concrete `CharacterStyle` is unchanged.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartialCharacterStyle {
    /// Direct bold override.
    pub bold: Option<bool>,
    /// Direct italic override.
    pub italic: Option<bool>,
    /// Direct underline override.
    pub underline: Option<bool>,
    /// Direct font size override in points.
    pub font_size_pt: Option<f32>,
    /// Direct font family override.
    pub font_family: Option<String>,
}

/// Direct paragraph formatting that overrides only the fields it sets.
///
/// Every field is optional: `None` keeps the value coming from the
/// default or named style, `Some` replaces it. This struct is purely
/// additive; the concrete `ParagraphStyle` is unchanged.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartialParagraphStyle {
    /// Direct alignment override.
    pub alignment: Option<Alignment>,
    /// Direct space-before override in points.
    pub space_before_pt: Option<f32>,
    /// Direct space-after override in points.
    pub space_after_pt: Option<f32>,
    /// Direct line spacing override.
    pub line_spacing: Option<f32>,
    /// Direct first-line indent override in points.
    pub indent_first_pt: Option<f32>,
}

/// Minimal collection of named styles keyed by name.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
    /// Styles by name.
    pub styles: HashMap<String, NamedStyle>,
}

impl StyleSheet {
    /// Create a sheet holding only the built-in "Normal" style.
    pub fn new() -> Self {
        let mut styles = HashMap::new();
        styles.insert(String::from(NORMAL_STYLE), NamedStyle::normal());
        Self { styles }
    }

    /// Insert or replace a style by its name.
    pub fn insert(&mut self, style: NamedStyle) {
        self.styles.insert(style.name.clone(), style);
    }

    /// Look up a style by name.
    pub fn get(&self, name: &str) -> Option<&NamedStyle> {
        self.styles.get(name)
    }

    /// Resolve the effective paragraph style.
    ///
    /// Start from `ParagraphStyle::default()`, overwrite with the
    /// named style when it exists, then overwrite with every direct
    /// field that is `Some`. A missing name falls back to default
    /// plus direct; this never panics.
    pub fn resolve_paragraph(&self, name: &str, direct: &PartialParagraphStyle) -> ParagraphStyle {
        let mut out = ParagraphStyle::default();
        if let Some(named) = self.get(name) {
            out = named.paragraph.clone();
        }
        if let Some(alignment) = direct.alignment {
            out.alignment = alignment;
        }
        if let Some(space_before_pt) = direct.space_before_pt {
            out.space_before_pt = space_before_pt;
        }
        if let Some(space_after_pt) = direct.space_after_pt {
            out.space_after_pt = space_after_pt;
        }
        if let Some(line_spacing) = direct.line_spacing {
            out.line_spacing = line_spacing;
        }
        if let Some(indent_first_pt) = direct.indent_first_pt {
            out.indent_first_pt = indent_first_pt;
        }
        out
    }

    /// Resolve the effective character style.
    ///
    /// Start from `CharacterStyle::default()`, overwrite with the
    /// named style when it exists, then overwrite with every direct
    /// field that is `Some`. A missing name falls back to default
    /// plus direct; this never panics.
    pub fn resolve_character(&self, name: &str, direct: &PartialCharacterStyle) -> CharacterStyle {
        let mut out = CharacterStyle::default();
        if let Some(named) = self.get(name) {
            out = named.character.clone();
        }
        if let Some(bold) = direct.bold {
            out.bold = bold;
        }
        if let Some(italic) = direct.italic {
            out.italic = italic;
        }
        if let Some(underline) = direct.underline {
            out.underline = underline;
        }
        if let Some(font_size_pt) = direct.font_size_pt {
            out.font_size_pt = font_size_pt;
        }
        if let Some(ref font_family) = direct.font_family {
            out.font_family = font_family.clone();
        }
        out
    }
}

impl Default for StyleSheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ships_with_normal() {
        let sheet = StyleSheet::new();
        assert!(sheet.get(NORMAL_STYLE).is_some());
        assert!(sheet.get("Missing").is_none());
    }

    #[test]
    fn insert_and_get() {
        let mut sheet = StyleSheet::new();
        let character = CharacterStyle {
            bold: true,
            ..CharacterStyle::default()
        };
        sheet.insert(NamedStyle::new(
            "Heading",
            ParagraphStyle::default(),
            character,
        ));
        let found = sheet.get("Heading").expect("just inserted");
        assert!(found.character.bold);
    }

    fn heading_sheet() -> StyleSheet {
        let mut sheet = StyleSheet::new();
        sheet.insert(NamedStyle::new(
            "Heading",
            ParagraphStyle {
                alignment: Alignment::Center,
                space_before_pt: 6.0,
                ..ParagraphStyle::default()
            },
            CharacterStyle {
                bold: true,
                font_size_pt: 14.0,
                ..CharacterStyle::default()
            },
        ));
        sheet
    }

    #[test]
    fn resolve_defaults_without_named_or_direct() {
        let sheet = StyleSheet::new();
        assert_eq!(
            sheet.resolve_paragraph("Missing", &PartialParagraphStyle::default()),
            ParagraphStyle::default()
        );
        assert_eq!(
            sheet.resolve_character("Missing", &PartialCharacterStyle::default()),
            CharacterStyle::default()
        );
    }

    #[test]
    fn resolve_named_overrides_default() {
        let sheet = heading_sheet();
        let para = sheet.resolve_paragraph("Heading", &PartialParagraphStyle::default());
        assert_eq!(para.alignment, Alignment::Center);
        assert_eq!(para.space_before_pt, 6.0);
        assert_eq!(para.line_spacing, 1.0);
        let character = sheet.resolve_character("Heading", &PartialCharacterStyle::default());
        assert!(character.bold);
        assert!(!character.italic);
        assert_eq!(character.font_size_pt, 14.0);
    }

    #[test]
    fn resolve_direct_overrides_named() {
        let sheet = heading_sheet();
        let direct_para = PartialParagraphStyle {
            alignment: Some(Alignment::Right),
            ..PartialParagraphStyle::default()
        };
        let para = sheet.resolve_paragraph("Heading", &direct_para);
        assert_eq!(para.alignment, Alignment::Right);
        assert_eq!(para.space_before_pt, 6.0);
        let direct_char = PartialCharacterStyle {
            bold: Some(false),
            ..PartialCharacterStyle::default()
        };
        let character = sheet.resolve_character("Heading", &direct_char);
        assert!(!character.bold);
        assert_eq!(character.font_size_pt, 14.0);
    }

    #[test]
    fn resolve_mixes_fields_from_each_level() {
        let sheet = heading_sheet();
        let direct_para = PartialParagraphStyle {
            space_after_pt: Some(9.0),
            ..PartialParagraphStyle::default()
        };
        let para = sheet.resolve_paragraph("Heading", &direct_para);
        assert_eq!(para.alignment, Alignment::Center);
        assert_eq!(para.space_before_pt, 6.0);
        assert_eq!(para.space_after_pt, 9.0);
        assert_eq!(para.line_spacing, 1.0);
        let direct_char = PartialCharacterStyle {
            italic: Some(true),
            ..PartialCharacterStyle::default()
        };
        let character = sheet.resolve_character("Heading", &direct_char);
        assert!(character.bold);
        assert!(character.italic);
        assert_eq!(character.font_size_pt, 14.0);
    }

    #[test]
    fn resolve_missing_name_falls_back_to_default_plus_direct() {
        let sheet = StyleSheet::new();
        let direct_para = PartialParagraphStyle {
            space_before_pt: Some(3.0),
            ..PartialParagraphStyle::default()
        };
        let para = sheet.resolve_paragraph("Gone", &direct_para);
        assert_eq!(para.space_before_pt, 3.0);
        assert_eq!(para.alignment, Alignment::Left);
        let direct_char = PartialCharacterStyle {
            bold: Some(true),
            ..PartialCharacterStyle::default()
        };
        let character = sheet.resolve_character("Gone", &direct_char);
        assert!(character.bold);
        assert_eq!(character.font_family, "Inter");
    }
}
