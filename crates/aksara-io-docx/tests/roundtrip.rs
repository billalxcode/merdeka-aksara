//! Round-trip test: file bytes to Document to bytes to Document.

use aksara_io_docx::{document_to_xml, read_docx, write_docx};
use aksara_model::Block;

#[test]
fn file_bytes_survive_reread() {
    let path = format!("{}/tests/data/mixed.docx", env!("CARGO_MANIFEST_DIR"));
    let first = std::fs::read(&path).unwrap_or_else(|_| panic!("missing {}", path));
    let first_doc = read_docx(&first).expect("fixture parses");
    let second = write_docx(&first_doc).expect("repack succeeds");
    let second_doc = read_docx(&second).expect("repacked bytes parse");

    assert_eq!(first_doc.block_count(), second_doc.block_count());
    assert_eq!(first_doc.word_count(), second_doc.word_count());
    assert_eq!(document_to_xml(&first_doc), document_to_xml(&second_doc));
    assert_eq!(first_doc.block_count(), 4);
    assert_eq!(second_doc.sections[0].blocks[2], Block::PageBreak);
}
