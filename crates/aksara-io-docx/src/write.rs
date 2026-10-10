//! DOCX writer: Document into a minimal valid ZIP package.
//!
//! The package holds exactly three parts, all stored (uncompressed):
//! `[Content_Types].xml`, `_rels/.rels`, and `word/document.xml`.
//! Images, section breaks, headers, and footers are skipped
//! explicitly, as documented in docs/file-format.md.

use std::io::{Cursor, Write};

use aksara_model::{Alignment, Block, Document, PageSetup, Paragraph, Table};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::IoDocxError;

/// Pack a Document into DOCX bytes.
pub fn write_docx(document: &Document) -> Result<Vec<u8>, IoDocxError> {
    let cursor = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("[Content_Types].xml", options)?;
    zip.write_all(content_types_xml().as_bytes())?;
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(rels_xml().as_bytes())?;
    zip.start_file("word/document.xml", options)?;
    zip.write_all(document_to_xml(document).as_bytes())?;
    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}

/// Render a Document to a word/document.xml string.
///
/// Split out for tests: the ZIP shell is fixed, the XML carries the
/// content, so string comparison covers the mapping.
pub fn document_to_xml(document: &Document) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<w:body>"#,
    );
    for section in &document.sections {
        for block in &section.blocks {
            render_block(block, &mut xml);
        }
    }
    let setup = document
        .sections
        .last()
        .map(|section| section.page_setup)
        .unwrap_or_else(PageSetup::a4_portrait);
    render_section_properties(setup, &mut xml);
    xml.push_str("</w:body></w:document>");
    xml
}

fn content_types_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#
}

fn rels_xml() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#
}

/// Escape text for XML element content and attribute values.
fn escape_xml(text: &str, out: &mut String) {
    for single in text.chars() {
        match single {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(single),
        }
    }
}

fn alignment_value(alignment: Alignment) -> &'static str {
    match alignment {
        Alignment::Left => "left",
        Alignment::Center => "center",
        Alignment::Right => "right",
        Alignment::Justify => "both",
    }
}

fn render_paragraph(paragraph: &Paragraph, out: &mut String) {
    if paragraph.runs.is_empty() {
        out.push_str("<w:p/>");
        return;
    }
    out.push_str("<w:p>");
    if paragraph.style.alignment != Alignment::Left {
        out.push_str("<w:pPr><w:jc w:val=\"");
        out.push_str(alignment_value(paragraph.style.alignment));
        out.push_str("\"/></w:pPr>");
    }
    for run in &paragraph.runs {
        out.push_str("<w:r>");
        if run.style.bold || run.style.italic || run.style.underline {
            out.push_str("<w:rPr>");
            if run.style.bold {
                out.push_str("<w:b/>");
            }
            if run.style.italic {
                out.push_str("<w:i/>");
            }
            if run.style.underline {
                out.push_str("<w:u/>");
            }
            out.push_str("</w:rPr>");
        }
        out.push_str("<w:t xml:space=\"preserve\">");
        escape_xml(&run.text, out);
        out.push_str("</w:t></w:r>");
    }
    out.push_str("</w:p>");
}

fn render_table(table: &Table, out: &mut String) {
    out.push_str("<w:tbl><w:tblPr><w:tblW w:w=\"0\" w:type=\"auto\"/></w:tblPr><w:tblGrid>");
    for _ in 0..table.col_count() {
        out.push_str("<w:gridCol w:w=\"2880\"/>");
    }
    out.push_str("</w:tblGrid>");
    for row in &table.rows {
        out.push_str("<w:tr>");
        for cell in &row.cells {
            out.push_str("<w:tc><w:tcPr><w:tcW w:w=\"0\" w:type=\"auto\"/></w:tcPr>");
            let mut wrote_paragraph = false;
            for block in &cell.blocks {
                if let Block::Paragraph(paragraph) = block {
                    render_paragraph(paragraph, out);
                    wrote_paragraph = true;
                }
            }
            if !wrote_paragraph {
                out.push_str("<w:p/>");
            }
            out.push_str("</w:tc>");
        }
        out.push_str("</w:tr>");
    }
    out.push_str("</w:tbl>");
}

fn render_block(block: &Block, out: &mut String) {
    match block {
        Block::Paragraph(paragraph) => render_paragraph(paragraph, out),
        Block::Table(table) => render_table(table, out),
        Block::PageBreak => {
            out.push_str("<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>");
        }
        Block::Image(_) | Block::SectionBreak(_) => {}
    }
}

fn render_section_properties(setup: PageSetup, out: &mut String) {
    let to_twips = |points: f64| (points * 20.0).round() as i64;
    let orient = if setup.width_pt >= setup.height_pt {
        "portrait"
    } else {
        "landscape"
    };
    out.push_str("<w:sectPr><w:pgSz w:w=\"");
    out.push_str(&to_twips(setup.width_pt).to_string());
    out.push_str("\" w:h=\"");
    out.push_str(&to_twips(setup.height_pt).to_string());
    out.push_str("\" w:orient=\"");
    out.push_str(orient);
    out.push_str("\"/><w:pgMar w:top=\"");
    out.push_str(&to_twips(setup.margin_top_pt).to_string());
    out.push_str("\" w:right=\"");
    out.push_str(&to_twips(setup.margin_right_pt).to_string());
    out.push_str("\" w:bottom=\"");
    out.push_str(&to_twips(setup.margin_bottom_pt).to_string());
    out.push_str("\" w:left=\"");
    out.push_str(&to_twips(setup.margin_left_pt).to_string());
    out.push_str("\" w:header=\"0\" w:footer=\"0\"/></w:sectPr>");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aksara_model::{CharacterStyle, TextRun};

    fn sample_paragraph() -> Paragraph {
        use aksara_model::NodeId;
        let mut paragraph = Paragraph::new(NodeId::new(1));
        paragraph.push_run(TextRun::new(
            NodeId::new(2),
            "Fish & Chips <tasty>",
            CharacterStyle::default(),
        ));
        paragraph
    }

    #[test]
    fn escapes_reserved_characters() {
        let mut out = String::new();
        escape_xml("Fish & Chips <tasty> \"quoted\"", &mut out);
        assert_eq!(out, "Fish &amp; Chips &lt;tasty&gt; &quot;quoted&quot;");
    }

    #[test]
    fn renders_paragraph_with_alignment() {
        use aksara_model::NodeId;
        let mut paragraph = sample_paragraph();
        paragraph.style.alignment = Alignment::Center;
        let mut second = Paragraph::new(NodeId::new(9));
        second.push_run(TextRun::new(
            NodeId::new(10),
            "plain",
            CharacterStyle::default(),
        ));
        let mut xml = String::new();
        render_paragraph(&paragraph, &mut xml);
        assert!(xml.contains("<w:jc w:val=\"center\"/>"), "got: {}", xml);
        assert!(
            xml.contains("Fish &amp; Chips &lt;tasty&gt;"),
            "got: {}",
            xml
        );
        assert!(xml.contains("xml:space=\"preserve\""), "got: {}", xml);
    }
}
