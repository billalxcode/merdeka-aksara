//! Facade tests: detect, dispatch, round-trips, and error paths.

use aksara_io::{
    Format, IoError, MagicHint, detect, detect_from_bytes, detect_from_extension, read_bytes,
    read_file, write_bytes, write_file,
};
use aksara_model::{Block, CharacterStyle, Document, Paragraph, TextRun};
use std::path::{Path, PathBuf};

fn sample_doc() -> Document {
    let mut doc = Document::default();
    let para_id = doc.alloc_id();
    let run_id = doc.alloc_id();
    let mut para = Paragraph::new(para_id);
    para.push_run(TextRun::new(
        run_id,
        "hello facade",
        CharacterStyle::default(),
    ));
    doc.sections[0].blocks[0] = Block::Paragraph(para);
    doc
}

fn paragraph_text(doc: &Document) -> String {
    match &doc.sections[0].blocks[0] {
        Block::Paragraph(para) => para.text(),
        other => panic!("expected paragraph, got {:?}", other),
    }
}

fn unique_temp_path(ext: &str) -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock before epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "aksara-io-facade-{}-{}-{}.{}",
        std::process::id(),
        stamp,
        ext.replace('.', "_"),
        ext
    ))
}

#[test]
fn detect_known_extensions() {
    let cases = [
        ("a.docx", Format::Docx),
        ("a.DOCX", Format::Docx),
        ("a.doc", Format::Doc),
        ("a.DOC", Format::Doc),
        ("a.odt", Format::Odt),
        ("a.ODT", Format::Odt),
        ("a.pdf", Format::Pdf),
        ("a.md", Format::Markdown),
        ("a.MD", Format::Markdown),
        ("a.markdown", Format::Markdown),
        ("a.txt", Format::Text),
        ("a.TXT", Format::Text),
        ("a.text", Format::Text),
    ];
    for (name, expected) in cases {
        assert_eq!(
            detect_from_extension(Path::new(name)).expect("known ext"),
            expected,
            "for {}",
            name
        );
    }
}

#[test]
fn detect_unknown_or_missing_extension_errors() {
    for name in ["a.xyz", "a", "a.", ".hidden"] {
        let error = detect_from_extension(Path::new(name)).expect_err("must fail");
        assert!(
            matches!(error, IoError::UnknownFormat(_)),
            "for {}: {:?}",
            name,
            error
        );
    }
}

#[test]
fn detect_magic_pk_ole_pdf() {
    assert_eq!(
        detect_from_bytes(&[0x50, 0x4B, 0x03, 0x04, 0x00]),
        Some(MagicHint::Zip)
    );
    assert_eq!(
        detect_from_bytes(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]),
        Some(MagicHint::Ole)
    );
    assert_eq!(
        detect_from_bytes(&[0x25, 0x50, 0x44, 0x46, 0x2D]),
        Some(MagicHint::Pdf)
    );
    assert_eq!(detect_from_bytes(&[0x00, 0x01, 0x02, 0x03]), None);
    assert_eq!(detect_from_bytes(&[]), None);
}

#[test]
fn detect_extension_wins_over_magic() {
    // .docx path holding OLE bytes still dispatches as Docx.
    let ole = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
    assert_eq!(
        detect(Path::new("a.docx"), Some(&ole)).expect("ext wins"),
        Format::Docx
    );
}

#[test]
fn detect_magic_fallback_without_extension() {
    let dir = std::env::temp_dir();
    let no_ext = dir.join("aksara-io-noext-probe");
    assert_eq!(
        detect(&no_ext, Some(&[0x50, 0x4B, 0x03, 0x04])).expect("zip fallback"),
        Format::Docx
    );
    assert_eq!(
        detect(
            &no_ext,
            Some(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1])
        )
        .expect("ole fallback"),
        Format::Doc
    );
    assert_eq!(
        detect(&no_ext, Some(&[0x25, 0x50, 0x44, 0x46])).expect("pdf fallback"),
        Format::Pdf
    );
    assert!(
        detect(&no_ext, Some(&[0x00, 0x01])).is_err(),
        "unknown magic keeps the extension error"
    );
    assert!(detect(&no_ext, None).is_err(), "no bytes means no fallback");
}

#[test]
fn docx_bytes_round_trip() {
    let doc = sample_doc();
    let bytes = write_bytes(Format::Docx, &doc).expect("write docx");
    let back = read_bytes(Format::Docx, &bytes).expect("read docx");
    assert_eq!(paragraph_text(&back), "hello facade");
    assert_eq!(back.word_count(), doc.word_count());
}

#[test]
fn docx_file_round_trip_via_temp_dir() {
    let path = unique_temp_path("docx");
    let doc = sample_doc();
    write_file(&path, &doc).expect("write file");
    let back = read_file(&path).expect("read file");
    assert_eq!(paragraph_text(&back), "hello facade");
    std::fs::remove_file(&path).expect("cleanup temp file");
    assert!(!path.exists(), "temp file removed");
}

#[test]
fn unsupported_read_each_non_docx() {
    for format in [
        Format::Doc,
        Format::Odt,
        Format::Pdf,
        Format::Markdown,
        Format::Text,
    ] {
        let error = read_bytes(format, b"data").expect_err("must be unsupported");
        match &error {
            IoError::Unsupported(message) => {
                assert!(
                    message.contains(format.extension()),
                    "{:?} names the format",
                    error
                );
            }
            other => panic!("expected Unsupported, got {:?}", other),
        }
    }
}

#[test]
fn unsupported_write_each_non_docx() {
    let doc = sample_doc();
    for format in [
        Format::Doc,
        Format::Odt,
        Format::Pdf,
        Format::Markdown,
        Format::Text,
    ] {
        let error = write_bytes(format, &doc).expect_err("must be unsupported");
        assert!(
            matches!(error, IoError::Unsupported(_)),
            "{:?} is Unsupported",
            error
        );
    }
}

#[test]
fn unsupported_file_error_names_format_and_path() {
    let path = unique_temp_path("odt");
    let doc = sample_doc();
    let error = write_file(&path, &doc).expect_err("odt has no backend");
    match &error {
        IoError::Unsupported(message) => {
            assert!(message.contains("odt"), "names format: {}", message);
            assert!(
                message.contains(&path.display().to_string()),
                "names path: {}",
                message
            );
        }
        other => panic!("expected Unsupported, got {:?}", other),
    }
    assert!(!path.exists(), "no file left behind");
}

#[test]
fn missing_file_is_io_error() {
    let path = std::env::temp_dir().join("aksara-io-missing-8f3a2c1d.docx");
    let _ = std::fs::remove_file(&path);
    let error = read_file(&path).expect_err("missing file fails");
    match &error {
        IoError::Io(message) => {
            assert!(
                message.contains(&path.display().to_string()),
                "names path: {}",
                message
            );
        }
        other => panic!("expected Io, got {:?}", other),
    }
}
