//! DOCX reader: ZIP container plus word/document.xml into a Document.
//!
//! Subset only: paragraphs, runs (bold/italic/underline), alignment,
//! plain text tables, and page breaks. Anything else is skipped and,
//! where it matters, counted in `ReadStats`. See docs/file-format.md.

use std::io::{Cursor, Read};

use aksara_model::{
    Alignment, Block, CharacterStyle, Document, PageSetup, Paragraph, Section, Table, TextRun,
};
use quick_xml::{XmlVersion, events::BytesStart, events::Event, reader::Reader};
use zip::ZipArchive;

use crate::IoDocxError;

/// Counts of content skipped while importing (subset boundary).
///
/// Callers can use these to warn instead of losing content silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReadStats {
    /// Nested w:tbl elements ignored inside table cells.
    pub nested_tables_skipped: usize,
    /// Drawing or picture runs ignored (no decoding in this phase).
    pub images_skipped: usize,
}

/// Read DOCX bytes into a Document, dropping import statistics.
pub fn read_docx(bytes: &[u8]) -> Result<Document, IoDocxError> {
    read_docx_with_stats(bytes).map(|(document, _)| document)
}

/// Read DOCX bytes into a Document plus skip counts.
pub fn read_docx_with_stats(bytes: &[u8]) -> Result<(Document, ReadStats), IoDocxError> {
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    let mut xml_text = String::new();
    match archive.by_name("word/document.xml") {
        Ok(mut entry) => {
            entry.read_to_string(&mut xml_text)?;
        }
        Err(zip::result::ZipError::FileNotFound) => {
            return Err(IoDocxError::MissingDocumentXml);
        }
        Err(other) => return Err(IoDocxError::from(other)),
    }
    document_from_xml_with_stats(&xml_text)
}

/// Parse a word/document.xml string into a Document.
pub fn document_from_xml(xml: &str) -> Result<Document, IoDocxError> {
    document_from_xml_with_stats(xml).map(|(document, _)| document)
}

/// Parse a word/document.xml string into a Document plus skip counts.
pub fn document_from_xml_with_stats(xml: &str) -> Result<(Document, ReadStats), IoDocxError> {
    let root = build_dom(xml.as_bytes())?;
    let document_el = find_child(&root, "document")
        .ok_or_else(|| IoDocxError::Xml(String::from("missing w:document root")))?;
    let body = find_child(document_el, "body")
        .ok_or_else(|| IoDocxError::Xml(String::from("missing w:body")))?;
    let mut document = Document::new();
    let mut stats = ReadStats::default();
    let section_id = document.alloc_id();
    let mut section = Section::new(section_id, PageSetup::a4_portrait());
    for block in parse_block_list(&body.children, &mut document, &mut stats, false) {
        section.push_block(block);
    }
    document.push_section(section);
    Ok((document, stats))
}

/// Minimal element tree for the subset (names keep the w: prefix).
struct XmlNode {
    name: String,
    attrs: Vec<(String, String)>,
    children: Vec<XmlNode>,
    text: String,
}

impl XmlNode {
    fn empty(name: String) -> Self {
        Self {
            name,
            attrs: Vec::new(),
            children: Vec::new(),
            text: String::new(),
        }
    }
}

/// Local element name without the namespace prefix (w:p becomes p).
fn local_name(name: &str) -> &str {
    match name.rfind(':') {
        Some(index) => &name[index + 1..],
        None => name,
    }
}

fn find_child<'a>(node: &'a XmlNode, want: &str) -> Option<&'a XmlNode> {
    node.children
        .iter()
        .find(|child| local_name(&child.name) == want)
}

fn attr_value<'a>(attrs: &'a [(String, String)], want: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|(key, _)| local_name(key) == want)
        .map(|(_, value)| value.as_str())
}

fn open_node(element: &BytesStart<'_>) -> Result<XmlNode, IoDocxError> {
    let qname = element.name();
    let name: &str = qname.as_ref();
    let mut node = XmlNode::empty(name.to_owned());
    for attr in element.attributes() {
        let attr = attr.map_err(|error| IoDocxError::Xml(error.to_string()))?;
        let key: &str = attr.key.as_ref();
        let value = attr
            .normalized_value(XmlVersion::Implicit1_0)
            .map_err(|error| IoDocxError::Xml(error.to_string()))?
            .into_owned();
        node.attrs.push((key.to_owned(), value));
    }
    Ok(node)
}

fn push_child(stack: &mut [XmlNode], node: XmlNode) -> Result<(), IoDocxError> {
    match stack.last_mut() {
        Some(parent) => {
            parent.children.push(node);
            Ok(())
        }
        None => Err(IoDocxError::Xml(String::from("unbalanced end tag"))),
    }
}

/// Resolve one entity or character reference (GeneralRef content).
fn resolve_reference(raw: &str, out: &mut String) {
    let text = raw;
    let predefined = match text {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => None,
    };
    if let Some(single) = predefined {
        out.push(single);
        return;
    }
    let code = text
        .strip_prefix("#x")
        .or_else(|| text.strip_prefix("#X"))
        .and_then(|hex| u32::from_str_radix(hex, 16).ok())
        .or_else(|| {
            text.strip_prefix('#')
                .and_then(|digits| digits.parse::<u32>().ok())
        });
    match code.and_then(char::from_u32) {
        Some(found) => out.push(found),
        None => {
            out.push('&');
            out.push_str(text);
            out.push(';');
        }
    }
}

/// Build the element tree. Whitespace between elements is kept in the
/// parent text but only w:t node text is ever read, so pretty printed
/// XML stays harmless. Spaces inside w:t are never trimmed.
fn build_dom(xml: &[u8]) -> Result<XmlNode, IoDocxError> {
    let mut reader = Reader::from_reader(xml);
    let mut buf = Vec::new();
    let mut stack = vec![XmlNode::empty(String::from("root"))];
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(element)) => {
                stack.push(open_node(&element)?);
            }
            Ok(Event::Empty(element)) => {
                push_child(&mut stack, open_node(&element)?)?;
            }
            Ok(Event::End(_)) => {
                let node = stack
                    .pop()
                    .ok_or_else(|| IoDocxError::Xml(String::from("unbalanced end tag")))?;
                push_child(&mut stack, node)?;
            }
            Ok(Event::Text(element)) => {
                let content = element.xml_content(XmlVersion::Implicit1_0);
                if let Some(top) = stack.last_mut() {
                    top.text.push_str(content.as_ref());
                }
            }
            Ok(Event::CData(element)) => {
                let content = element.into_inner();
                if let Some(top) = stack.last_mut() {
                    top.text.push_str(content.as_ref());
                }
            }
            Ok(Event::GeneralRef(element)) => {
                let mut decoded = String::new();
                resolve_reference(element.into_inner().as_ref(), &mut decoded);
                if let Some(top) = stack.last_mut() {
                    top.text.push_str(&decoded);
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(error) => return Err(IoDocxError::from(error)),
        }
        buf.clear();
    }
    if stack.len() != 1 {
        return Err(IoDocxError::Xml(String::from(
            "unbalanced tags at end of document",
        )));
    }
    Ok(stack.pop().expect("single root"))
}

/// True unless the flag carries an explicit off value.
fn is_on(value: Option<&str>) -> bool {
    match value {
        None => true,
        Some(raw) => !matches!(raw, "false" | "0" | "off" | "none"),
    }
}

fn parse_run_properties(node: &XmlNode) -> CharacterStyle {
    let mut style = CharacterStyle::default();
    for child in &node.children {
        match local_name(&child.name) {
            "b" => style.bold = is_on(attr_value(&child.attrs, "val")),
            "i" => style.italic = is_on(attr_value(&child.attrs, "val")),
            "u" => style.underline = is_on(attr_value(&child.attrs, "val")),
            _ => {}
        }
    }
    style
}

struct ParsedRun {
    style: CharacterStyle,
    text: String,
}

/// Parse one w:r: concatenated text plus page break count.
fn parse_run(node: &XmlNode, stats: &mut ReadStats) -> (ParsedRun, usize) {
    let mut style = CharacterStyle::default();
    let mut text = String::new();
    let mut page_breaks = 0;
    for child in &node.children {
        match local_name(&child.name) {
            "rPr" => style = parse_run_properties(child),
            "t" => text.push_str(&child.text),
            "tab" => text.push('\t'),
            "br" => {
                if attr_value(&child.attrs, "type").unwrap_or("") == "page" {
                    page_breaks += 1;
                } else {
                    text.push('\n');
                }
            }
            "drawing" | "pict" => stats.images_skipped += 1,
            "noBreakHyphen" => text.push('-'),
            _ => {}
        }
    }
    (ParsedRun { style, text }, page_breaks)
}

/// Collect runs, descending into wrappers (hyperlink, sdt, smartTag).
fn collect_runs(
    node: &XmlNode,
    stats: &mut ReadStats,
    runs: &mut Vec<ParsedRun>,
    breaks: &mut usize,
) {
    for child in &node.children {
        match local_name(&child.name) {
            "r" => {
                let (run, found) = parse_run(child, stats);
                runs.push(run);
                *breaks += found;
            }
            "hyperlink" | "sdt" | "sdtContent" | "smartTag" | "ins" => {
                collect_runs(child, stats, runs, breaks);
            }
            _ => {}
        }
    }
}

fn parse_alignment(node: &XmlNode) -> Alignment {
    let raw = find_child(node, "jc")
        .and_then(|jc| attr_value(&jc.attrs, "val"))
        .unwrap_or("left");
    match raw.to_ascii_lowercase().as_str() {
        "center" => Alignment::Center,
        "right" | "end" => Alignment::Right,
        "both" | "distribute" => Alignment::Justify,
        _ => Alignment::Left,
    }
}

struct ParsedParagraph {
    runs: Vec<ParsedRun>,
    alignment: Alignment,
    page_breaks: usize,
}

fn parse_paragraph(node: &XmlNode, stats: &mut ReadStats) -> ParsedParagraph {
    let mut parsed = ParsedParagraph {
        runs: Vec::new(),
        alignment: Alignment::Left,
        page_breaks: 0,
    };
    for child in &node.children {
        if local_name(&child.name) == "pPr" {
            parsed.alignment = parse_alignment(child);
        }
    }
    collect_runs(node, stats, &mut parsed.runs, &mut parsed.page_breaks);
    parsed
}

fn push_page_breaks(blocks: &mut Vec<Block>, count: usize) {
    for _ in 0..count {
        blocks.push(Block::PageBreak);
    }
}

fn materialize_paragraph(
    parsed: ParsedParagraph,
    document: &mut Document,
    blocks: &mut Vec<Block>,
) {
    let has_text = parsed.runs.iter().any(|run| !run.text.is_empty());
    if !has_text {
        if parsed.page_breaks == 0 {
            let id = document.alloc_id();
            blocks.push(Block::Paragraph(Paragraph::new(id)));
        } else {
            push_page_breaks(blocks, parsed.page_breaks);
        }
        return;
    }
    let id = document.alloc_id();
    let mut paragraph = Paragraph::new(id);
    paragraph.style.alignment = parsed.alignment;
    for run in parsed.runs {
        let run_id = document.alloc_id();
        paragraph.push_run(TextRun::new(run_id, &run.text, run.style));
    }
    blocks.push(Block::Paragraph(paragraph));
    push_page_breaks(blocks, parsed.page_breaks);
}

/// Parse body or cell level children. Inside cells nested tables are
/// skipped and counted instead of recursing.
fn parse_block_list(
    nodes: &[XmlNode],
    document: &mut Document,
    stats: &mut ReadStats,
    in_table: bool,
) -> Vec<Block> {
    let mut blocks = Vec::new();
    for node in nodes {
        match local_name(&node.name) {
            "p" => {
                let parsed = parse_paragraph(node, stats);
                materialize_paragraph(parsed, document, &mut blocks);
            }
            "tbl" => {
                if in_table {
                    stats.nested_tables_skipped += 1;
                } else {
                    blocks.push(Block::Table(parse_table(node, document, stats)));
                }
            }
            "sdt" => {
                if let Some(content) = find_child(node, "sdtContent") {
                    blocks.extend(parse_block_list(
                        &content.children,
                        document,
                        stats,
                        in_table,
                    ));
                }
            }
            _ => {}
        }
    }
    blocks
}

fn parse_table(node: &XmlNode, document: &mut Document, stats: &mut ReadStats) -> Table {
    let rows_xml: Vec<&XmlNode> = node
        .children
        .iter()
        .filter(|child| local_name(&child.name) == "tr")
        .collect();
    let cols = rows_xml
        .iter()
        .map(|row| {
            row.children
                .iter()
                .filter(|cell| local_name(&cell.name) == "tc")
                .count()
        })
        .max()
        .unwrap_or(0);
    let mut table = Table::new(document.alloc_id(), rows_xml.len(), cols);
    for (row_idx, row) in rows_xml.iter().enumerate() {
        if row_idx >= table.row_count() {
            break;
        }
        let cells_xml: Vec<&XmlNode> = row
            .children
            .iter()
            .filter(|cell| local_name(&cell.name) == "tc")
            .collect();
        for (col_idx, cell) in cells_xml.iter().enumerate() {
            if col_idx >= table.col_count() {
                break;
            }
            for block in parse_block_list(&cell.children, document, stats, true) {
                table.rows[row_idx].cells[col_idx].push_block(block);
            }
        }
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARA_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:p><w:r><w:t xml:space="preserve">Hello </w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t>bold</w:t></w:r><w:r><w:rPr><w:i/><w:u w:val="none"/></w:rPr><w:t>mixed</w:t></w:r></w:p>
<w:p><w:pPr><w:jc w:val="center"/></w:pPr><w:r><w:t>Title</w:t></w:r></w:p>
<w:sectPr/>
</w:body>
</w:document>"#;

    fn first_paragraph(document: &Document, index: usize) -> &Paragraph {
        match &document.sections[0].blocks[index] {
            Block::Paragraph(para) => para,
            other => panic!("expected paragraph, got {:?}", other),
        }
    }

    #[test]
    fn parses_multi_run_paragraph() {
        let document = document_from_xml(PARA_XML).expect("valid xml");
        assert_eq!(document.block_count(), 2);
        let first = first_paragraph(&document, 0);
        assert_eq!(first.text(), "Hello boldmixed");
        assert_eq!(first.runs.len(), 3);
        assert!(!first.runs[0].style.bold);
        assert!(first.runs[1].style.bold);
        assert!(!first.runs[1].style.italic);
        assert!(first.runs[2].style.italic);
        assert!(!first.runs[2].style.underline);
        assert_eq!(first.style.alignment, Alignment::Left);
        let second = first_paragraph(&document, 1);
        assert_eq!(second.text(), "Title");
        assert_eq!(second.style.alignment, Alignment::Center);
    }

    const TABLE_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:tbl><w:tblPr/><w:tblGrid/><w:tr><w:tc><w:p><w:r><w:t>A1</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>B1</w:t></w:r></w:p></w:tc></w:tr><w:tr><w:tc><w:p><w:r><w:t>A2</w:t></w:r></w:p></w:tc><w:tc><w:p><w:r><w:t>B2</w:t></w:r></w:p></w:tc></w:tr></w:tbl>
<w:sectPr/>
</w:body>
</w:document>"#;

    #[test]
    fn parses_basic_table() {
        let document = document_from_xml(TABLE_XML).expect("valid xml");
        assert_eq!(document.block_count(), 1);
        match &document.sections[0].blocks[0] {
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
    fn rejects_broken_xml() {
        assert!(document_from_xml("<w:document>").is_err());
        assert!(document_from_xml("").is_err());
        assert!(document_from_xml("<w:document><w:body>").is_err());
    }

    #[test]
    fn counts_skipped_content() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>
<w:p><w:r><w:rPr/><w:drawing/></w:r></w:p>
<w:tbl><w:tr><w:tc><w:p><w:r><w:t>outer</w:t></w:r></w:p><w:tbl><w:tr><w:tc><w:p><w:r><w:t>inner</w:t></w:r></w:p></w:tc></w:tr></w:tbl></w:tc></w:tr></w:tbl>
<w:sectPr/>
</w:body>
</w:document>"#;
        let (document, stats) = document_from_xml_with_stats(xml).expect("valid xml with skips");
        assert_eq!(stats.images_skipped, 1);
        assert_eq!(stats.nested_tables_skipped, 1);
        assert_eq!(document.block_count(), 2);
    }
}
