//! Write tests: build a Document via the model API, pack it, unpack
//! it with a ZIP reader, and parse it back into the model.

use aksara_io_docx::{read_docx, write_docx};
use aksara_model::{
    Alignment, Block, CharacterStyle, Document, PageSetup, Paragraph, Section, Table, TextRun,
};

fn sample_document() -> Document {
    let mut document = Document::new();
    let section_id = document.alloc_id();
    let mut section = Section::new(section_id, PageSetup::a4_portrait());

    let mut title = Paragraph::new(document.alloc_id());
    title.style.alignment = Alignment::Center;
    title.push_run(TextRun::new(
        document.alloc_id(),
        "Title ",
        CharacterStyle::default(),
    ));
    let bold = CharacterStyle {
        bold: true,
        ..CharacterStyle::default()
    };
    title.push_run(TextRun::new(document.alloc_id(), "bold", bold));
    title.push_run(TextRun::new(
        document.alloc_id(),
        " a & b <c>",
        CharacterStyle::default(),
    ));
    section.push_block(Block::Paragraph(title));

    let mut table = Table::new(document.alloc_id(), 2, 2);
    for (row, col, text) in [(0, 0, "A1"), (0, 1, "B1"), (1, 0, "A2"), (1, 1, "B2")] {
        let mut cell_para = Paragraph::new(document.alloc_id());
        cell_para.push_run(TextRun::new(
            document.alloc_id(),
            text,
            CharacterStyle::default(),
        ));
        table.rows[row].cells[col].push_block(Block::Paragraph(cell_para));
    }
    section.push_block(Block::Table(table));
    section.push_block(Block::PageBreak);
    section.push_block(Block::Paragraph(Paragraph::new(document.alloc_id())));

    document.push_section(section);
    document
}

#[test]
fn writes_valid_package_with_required_entries() {
    let bytes = write_docx(&sample_document()).expect("write succeeds");
    let cursor = std::io::Cursor::new(&bytes);
    let mut archive = zip::ZipArchive::new(cursor).expect("readable zip");
    assert_eq!(archive.len(), 3);
    for name in ["[Content_Types].xml", "_rels/.rels", "word/document.xml"] {
        assert!(
            archive.by_name(name).is_ok(),
            "package must hold entry {}",
            name
        );
    }
    let mut body = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("word/document.xml").expect("entry"),
        &mut body,
    )
    .expect("text entry");
    assert!(body.contains("<w:body>"), "body element present");
    assert!(body.contains("<w:tbl>"), "table element present");
    assert!(body.contains("w:type=\"page\""), "page break present");
}

#[test]
fn write_then_read_preserves_content() {
    let before = sample_document();
    let bytes = write_docx(&before).expect("write succeeds");
    let after = read_docx(&bytes).expect("written bytes parse");
    assert_eq!(after.block_count(), before.block_count());
    assert_eq!(after.word_count(), before.word_count());

    match &after.sections[0].blocks[0] {
        Block::Paragraph(para) => {
            assert_eq!(para.text(), "Title bold a & b <c>");
            assert_eq!(para.style.alignment, Alignment::Center);
            assert!(para.runs[1].style.bold);
        }
        other => panic!("expected paragraph, got {:?}", other),
    }
    match &after.sections[0].blocks[1] {
        Block::Table(table) => {
            assert_eq!(table.row_count(), 2);
            assert_eq!(table.col_count(), 2);
            match table.rows[1].cells[0].blocks.first() {
                Some(Block::Paragraph(para)) => assert_eq!(para.text(), "A2"),
                other => panic!("expected paragraph, got {:?}", other),
            }
        }
        other => panic!("expected table, got {:?}", other),
    }
    assert_eq!(after.sections[0].blocks[2], Block::PageBreak);
    match &after.sections[0].blocks[3] {
        Block::Paragraph(para) => assert!(para.is_empty()),
        other => panic!("expected paragraph, got {:?}", other),
    }
}
