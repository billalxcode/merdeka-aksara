//! DOCX import and export for the Aksara document model.
//!
//! Phase 5 subset: paragraphs, runs (bold/italic/underline), paragraph
//! alignment, plain text tables, and page breaks. The crate translates
//! between `aksara-model` types and Office Open XML (`word/document.xml`
//! inside a ZIP package). All OOXML adaptation lives here; the model
//! crate stays dependency-free and its API is used as-is.
//!
//! Entry points: `read_docx` and `write_docx` for bytes, plus
//! `document_from_xml` and `document_to_xml` for the XML part alone
//! (handy for unit tests). Failures come back as `IoDocxError`; the
//! IO path never panics. See `docs/file-format.md` for the subset.

pub mod read;
pub mod write;

pub use read::{
    ReadStats, document_from_xml, document_from_xml_with_stats, read_docx, read_docx_with_stats,
};
pub use write::{document_to_xml, write_docx};

use std::fmt;

/// Errors from DOCX reading and writing.
///
/// Variants carry rendered messages (not the dependency error types)
// so the public API stays stable if dependencies change.
#[derive(Debug)]
pub enum IoDocxError {
    /// The ZIP container could not be opened or written.
    Zip(String),
    /// The document XML could not be parsed or is missing parts.
    Xml(String),
    /// An underlying IO operation failed.
    Io(String),
    /// The package has no `word/document.xml` part.
    MissingDocumentXml,
}

impl fmt::Display for IoDocxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zip(message) => write!(formatter, "docx zip error: {}", message),
            Self::Xml(message) => write!(formatter, "docx xml error: {}", message),
            Self::Io(message) => write!(formatter, "docx io error: {}", message),
            Self::MissingDocumentXml => {
                write!(formatter, "docx package has no word/document.xml")
            }
        }
    }
}

impl std::error::Error for IoDocxError {}

impl From<zip::result::ZipError> for IoDocxError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error.to_string())
    }
}

impl From<std::io::Error> for IoDocxError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl From<quick_xml::Error> for IoDocxError {
    fn from(error: quick_xml::Error) -> Self {
        Self::Xml(error.to_string())
    }
}
