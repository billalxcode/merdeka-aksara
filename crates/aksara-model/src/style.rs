//! Named styles: a minimal std-only style sheet.
//!
//! Resolution order (default, named style, direct formatting) is a
//! later phase. Here we only store and look up named styles.

use crate::text::{CharacterStyle, ParagraphStyle};
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
}
