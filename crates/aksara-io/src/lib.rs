//! Format-dispatch facade over per-format IO backends.
//!
//! [`Format`] picks the backend; [`read_bytes`] and [`write_bytes`]
//! dispatch on it. Only Docx is wired (via `aksara-io-docx`); every
//! other format returns [`IoError::Unsupported`] so later backends
//! (`aksara-io-doc`, `aksara-io-odt`, `aksara-io-pdf`) can plug in
//! without changing this facade beyond a new dispatch branch.
//!
//! Detection rule: the file extension wins. Magic bytes are only a
//! fallback when the path has no (or an unknown) extension. A ZIP
//! magic (`PK..`) without an extension is assumed to be Docx; Odt
//! shares the same container, so callers should always use a real
//! extension for Odt files.

use aksara_model::Document;
use std::fmt;
use std::path::Path;

/// Document formats known to the facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// Office Open XML (`.docx`), backend: `aksara-io-docx`.
    Docx,
    /// Legacy OLE Word (`.doc`), no backend yet.
    Doc,
    /// OpenDocument Text (`.odt`), no backend yet.
    Odt,
    /// Portable Document Format (`.pdf`), no backend yet.
    Pdf,
    /// Markdown (`.md`), no backend yet.
    Markdown,
    /// Plain text (`.txt`), no backend yet.
    Text,
}

impl Format {
    /// Canonical extension for the format (no leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Docx => "docx",
            Self::Doc => "doc",
            Self::Odt => "odt",
            Self::Pdf => "pdf",
            Self::Markdown => "md",
            Self::Text => "txt",
        }
    }

    /// Parse an extension (case-insensitive, no leading dot).
    ///
    /// Accepts `md` and `markdown` for Markdown, `txt` and `text`
    /// for Text. Returns `None` for anything else.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "docx" => Some(Self::Docx),
            "doc" => Some(Self::Doc),
            "odt" => Some(Self::Odt),
            "pdf" => Some(Self::Pdf),
            "md" | "markdown" => Some(Self::Markdown),
            "txt" | "text" => Some(Self::Text),
            _ => None,
        }
    }
}

/// Errors from facade dispatch, detection, and file IO.
#[derive(Debug)]
pub enum IoError {
    /// The Docx backend failed; wraps the backend error type.
    Docx(aksara_io_docx::IoDocxError),
    /// The format is recognized but has no backend yet.
    /// Carries a message naming the format and the path.
    Unsupported(String),
    /// A file operation failed. Carries a message with the path.
    Io(String),
    /// No format could be detected. Carries a message with the path.
    UnknownFormat(String),
}

impl fmt::Display for IoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Docx(error) => write!(formatter, "docx backend error: {}", error),
            Self::Unsupported(message) | Self::Io(message) | Self::UnknownFormat(message) => {
                write!(formatter, "{}", message)
            }
        }
    }
}

impl std::error::Error for IoError {}

impl From<std::io::Error> for IoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl From<aksara_io_docx::IoDocxError> for IoError {
    fn from(error: aksara_io_docx::IoDocxError) -> Self {
        Self::Docx(error)
    }
}

/// Best-effort magic-byte hint from a byte prefix.
///
/// This is a candidate, not a verdict: ZIP covers Docx, Odt, and
/// other OOXML packages alike, so the extension always wins in
/// [`detect`]. Only used as a fallback for extensionless paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicHint {
    /// `50 4B 03 04` (`PK..`): ZIP container (Docx/Odt/...).
    Zip,
    /// `D0 CF 11 E0 A1 B1 1A E1`: OLE container (legacy Doc).
    Ole,
    /// `25 50 44 46` (`%PDF`): PDF document.
    Pdf,
}

/// Detect a format from the path extension (lower-cased text after
/// the last dot). Fails with [`IoError::UnknownFormat`] when the
/// path has no extension or an unrecognized one.
pub fn detect_from_extension(path: &Path) -> Result<Format, IoError> {
    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    match Format::from_extension(ext) {
        Some(format) => Ok(format),
        None => Err(IoError::UnknownFormat(format!(
            "unknown format for path {} (extension '{}')",
            path.display(),
            ext
        ))),
    }
}

/// Sniff a magic-byte hint from a byte prefix (no parsing).
/// Returns `None` when the prefix matches nothing known.
pub fn detect_from_bytes(bytes: &[u8]) -> Option<MagicHint> {
    if bytes.len() >= 4 && bytes[0] == 0x50 && bytes[1] == 0x4B {
        if bytes[2] == 0x03 && bytes[3] == 0x04 {
            return Some(MagicHint::Zip);
        }
        return None;
    }
    if bytes.len() >= 8
        && bytes[0] == 0xD0
        && bytes[1] == 0xCF
        && bytes[2] == 0x11
        && bytes[3] == 0xE0
        && bytes[4] == 0xA1
        && bytes[5] == 0xB1
        && bytes[6] == 0x1A
        && bytes[7] == 0xE1
    {
        return Some(MagicHint::Ole);
    }
    if bytes.len() >= 4
        && bytes[0] == 0x25
        && bytes[1] == 0x50
        && bytes[2] == 0x44
        && bytes[3] == 0x46
    {
        return Some(MagicHint::Pdf);
    }
    None
}

/// Detect the format for a path, with optional bytes for fallback.
///
/// The extension always wins, even when the magic disagrees (a
/// mismatch is left for the backend to reject naturally). Magic is
/// only consulted when the extension is missing or unknown: ZIP
/// maps to Docx (best effort, documented above), OLE to Doc, and
/// `%PDF` to Pdf. With no usable hint, the extension error is kept.
pub fn detect(path: &Path, bytes: Option<&[u8]>) -> Result<Format, IoError> {
    match detect_from_extension(path) {
        Ok(format) => Ok(format),
        Err(first_error) => {
            if let Some(prefix) = bytes {
                match detect_from_bytes(prefix) {
                    Some(MagicHint::Zip) => Ok(Format::Docx),
                    Some(MagicHint::Ole) => Ok(Format::Doc),
                    Some(MagicHint::Pdf) => Ok(Format::Pdf),
                    None => Err(first_error),
                }
            } else {
                Err(first_error)
            }
        }
    }
}

fn unsupported(format: Format, context: &str) -> IoError {
    IoError::Unsupported(format!(
        "unsupported format {} for {}",
        format.extension(),
        context
    ))
}

/// Read a document from in-memory bytes of the given format.
pub fn read_bytes(format: Format, bytes: &[u8]) -> Result<Document, IoError> {
    match format {
        Format::Docx => aksara_io_docx::read_docx(bytes).map_err(IoError::from),
        other => Err(unsupported(other, "bytes input")),
    }
}

/// Write a document to in-memory bytes of the given format.
pub fn write_bytes(format: Format, doc: &Document) -> Result<Vec<u8>, IoError> {
    match format {
        Format::Docx => aksara_io_docx::write_docx(doc).map_err(IoError::from),
        other => Err(unsupported(other, "bytes input")),
    }
}

/// Read a document from a file; the format comes from [`detect`]
/// over the path plus the file bytes as magic fallback.
pub fn read_file(path: &Path) -> Result<Document, IoError> {
    let bytes = std::fs::read(path)
        .map_err(|error| IoError::Io(format!("io error for {}: {}", path.display(), error)))?;
    let format = detect(path, Some(&bytes))?;
    match format {
        Format::Docx => read_bytes(format, &bytes),
        other => Err(unsupported(other, &path.display().to_string())),
    }
}

/// Write a document to a file; the format comes from the path
/// extension. Fails with [`IoError::UnknownFormat`] for paths
/// without a known extension.
pub fn write_file(path: &Path, doc: &Document) -> Result<(), IoError> {
    let format = detect(path, None)?;
    let bytes = match format {
        Format::Docx => write_bytes(format, doc)?,
        other => return Err(unsupported(other, &path.display().to_string())),
    };
    std::fs::write(path, &bytes)
        .map_err(|error| IoError::Io(format!("io error for {}: {}", path.display(), error)))
}
