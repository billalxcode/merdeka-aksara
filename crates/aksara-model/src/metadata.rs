//! Document-level metadata (title, author, timestamps).
//!
//! Timestamps are Unix seconds as plain `i64` so the crate needs no
//! date library. The default uses 0 for both stamps to stay
//! deterministic in tests.

/// Descriptive metadata stored with a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentMetadata {
    /// Display title of the document.
    pub title: String,
    /// Author name, may be empty.
    pub author: String,
    /// Creation time as Unix seconds.
    pub created_unix: i64,
    /// Last modification time as Unix seconds.
    pub modified_unix: i64,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            title: String::from("Untitled"),
            author: String::new(),
            created_unix: 0,
            modified_unix: 0,
        }
    }
}

impl DocumentMetadata {
    /// Create metadata with explicit fields.
    pub fn new(title: &str, author: &str, created_unix: i64, modified_unix: i64) -> Self {
        Self {
            title: String::from(title),
            author: String::from(author),
            created_unix,
            modified_unix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_metadata_is_sane() {
        let meta = DocumentMetadata::default();
        assert_eq!(meta.title, "Untitled");
        assert_eq!(meta.author, "");
        assert_eq!(meta.created_unix, 0);
        assert_eq!(meta.modified_unix, 0);
    }

    #[test]
    fn constructor_stores_fields() {
        let meta = DocumentMetadata::new("Report", "Ayu", 100, 200);
        assert_eq!(meta.title, "Report");
        assert_eq!(meta.author, "Ayu");
        assert_eq!(meta.created_unix, 100);
        assert_eq!(meta.modified_unix, 200);
    }
}
