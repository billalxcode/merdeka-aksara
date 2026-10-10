//! Read tests over the three DOCX fixtures in tests/data.
//!
//! The fixtures are hand-written minimal packages (see /tmp/gen_docx.py
//! used at creation time), so these tests do not depend on our writer.

use aksara_io_docx::read_docx;
use aksara_model::{Alignment, Block};

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/data/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read(&path).unwrap_or_else(|_| panic!("missing fixture {}", path))
}

fn paragraph_text(block: &Block) -> String {
    match block {
        Block::Paragraph(para) => para.text(),
        other => panic!("expected paragraph, got {:?}", other),
    }
}

#[test]
fn reads_paragraph_runs_and_alignment() {
    let document = read_docx(&fixture("para-runs.docx")).expect("fixture parses");
    assert_eq!(document.block_count(), 4);
    assert_eq!(document.word_count(), 15);

    let first = match &document.sections[0].blocks[0] {
        Block::Paragraph(para) => para,
        other => panic!("expected paragraph, got {:?}", other),
    };
    assert_eq!(first.text(), "Hello bold and italic plus underline");
    assert_eq!(first.runs.len(), 6);
    assert!(!first.runs[0].style.bold);
    assert!(first.runs[1].style.bold);
    assert!(first.runs[3].style.italic);
    assert!(!first.runs[3].style.bold);
    assert!(first.runs[5].style.underline);
    assert!(!first.runs[5].style.italic);
    assert_eq!(first.style.alignment, Alignment::Left);

    let alignments: Vec<Alignment> = document.sections[0]
        .blocks
        .iter()
        .map(|block| match block {
            Block::Paragraph(para) => para.style.alignment,
            other => panic!("expected paragraph, got {:?}", other),
        })
        .collect();
    assert_eq!(
        alignments,
        vec![
            Alignment::Left,
            Alignment::Center,
            Alignment::Right,
            Alignment::Justify
        ]
    );
    assert_eq!(
        paragraph_text(&document.sections[0].blocks[1]),
        "Centered title"
    );
    assert_eq!(
        paragraph_text(&document.sections[0].blocks[2]),
        "Right note"
    );
}

#[test]
fn reads_basic_table() {
    let document = read_docx(&fixture("table-basic.docx")).expect("fixture parses");
    assert_eq!(document.block_count(), 2);
    assert_eq!(document.word_count(), 6);
    assert_eq!(
        paragraph_text(&document.sections[0].blocks[0]),
        "Before table."
    );
    match &document.sections[0].blocks[1] {
        Block::Table(table) => {
            assert_eq!(table.row_count(), 2);
            assert_eq!(table.col_count(), 2);
            let texts: Vec<String> = table
                .rows
                .iter()
                .flat_map(|row| row.cells.iter())
                .map(|cell| match cell.blocks.first() {
                    Some(Block::Paragraph(para)) => para.text(),
                    other => panic!("expected paragraph, got {:?}", other),
                })
                .collect();
            assert_eq!(texts, vec!["A1", "B1", "A2", "B2"]);
        }
        other => panic!("expected table, got {:?}", other),
    }
}

#[test]
fn reads_mixed_content_with_page_break() {
    let document = read_docx(&fixture("mixed.docx")).expect("fixture parses");
    assert_eq!(document.block_count(), 4);
    assert_eq!(document.word_count(), 6);
    assert_eq!(
        paragraph_text(&document.sections[0].blocks[0]),
        "Intro paragraph."
    );
    match &document.sections[0].blocks[1] {
        Block::Table(table) => {
            assert_eq!(table.row_count(), 1);
            assert_eq!(table.col_count(), 2);
        }
        other => panic!("expected table, got {:?}", other),
    }
    assert_eq!(document.sections[0].blocks[2], Block::PageBreak);
    assert_eq!(
        paragraph_text(&document.sections[0].blocks[3]),
        "After break."
    );
}
