//! Pagination: laid blocks to A4 pages with display lists.

use crate::block::{LaidBlock, layout_blocks};
use crate::measure::TextMeasurer;
use aksara_model::Document;

/// One placed line: text plus pt coordinates in the content area.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedLine {
    /// Line text.
    pub text: String,
    /// X in points from the content-area left edge (margin + offset).
    pub x_pt: f64,
    /// Y in points from the content-area top edge.
    pub y_pt: f64,
    /// Measured width in points.
    pub width_pt: f64,
}

/// One computed page: body lines plus repeated header/footer.
///
/// Coordinates use points with the origin at the top-left of the
/// content area (x = margin_left + offset, y = margin_top +
/// header_h + consumed). Pages are computed, never stored in model.
#[derive(Debug, Clone, PartialEq)]
pub struct LaidPage {
    /// Owning section id (as u64 value).
    pub section_id: u64,
    /// 1-based page number inside the document.
    pub page_no: usize,
    /// Body lines in reading order.
    pub lines: Vec<PlacedLine>,
    /// Repeated header lines (y inside the top margin).
    pub header: Vec<PlacedLine>,
    /// Repeated footer lines (y inside the bottom margin).
    pub footer: Vec<PlacedLine>,
}

/// Paginate `doc` with `measurer` into display-list pages.
///
/// Per section: the content area comes from
/// `page_setup.content_width/height()`; header/footer are measured
/// first and their height is subtracted from the body budget; a new
/// page starts on overflow or on `Break`. When the header/footer
/// alone fill the page, an empty page is returned and the overflow
/// is documented (no infinite loop).
pub fn paginate(doc: &Document, measurer: &dyn TextMeasurer) -> Vec<LaidPage> {
    let mut pages: Vec<LaidPage> = Vec::new();
    let mut page_no = 0usize;

    for section in &doc.sections {
        let setup = section.page_setup;
        let max_w = setup.content_width();
        let content_h = setup.content_height();

        // Measure header/footer once per section via block layout.
        let laid_header = layout_blocks(&section.header, measurer, max_w, 0.0);
        let laid_footer = layout_blocks(&section.footer, measurer, max_w, 0.0);
        let header_h: f64 = laid_height_of(&laid_header);
        let footer_h: f64 = laid_height_of(&laid_footer);
        let body_budget = (content_h - header_h - footer_h).max(0.0);

        let header_lines = place_chrome(&laid_header, section.id.value(), setup.margin_left_pt);
        let footer_lines = place_chrome(&laid_footer, section.id.value(), setup.margin_left_pt);

        // Guard: chrome taller than the page -> one empty page, documented.
        if body_budget <= 0.0 && (!section.blocks.is_empty()) {
            page_no += 1;
            pages.push(LaidPage {
                section_id: section.id.value(),
                page_no,
                lines: Vec::new(),
                header: header_lines,
                footer: footer_lines,
            });
            continue;
        }

        let laid = layout_blocks(&section.blocks, measurer, max_w, 0.0);
        // Start the first page lazily: empty sections yield no page
        // unless they carry a break or content.
        let mut current: Vec<PlacedLine> = Vec::new();
        let mut used_h = 0.0;
        let mut started = false;

        let flush = |pages: &mut Vec<LaidPage>,
                     page_no: &mut usize,
                     current: &mut Vec<PlacedLine>,
                     started: &mut bool,
                     header_lines: &[PlacedLine],
                     footer_lines: &[PlacedLine]| {
            if *started || !current.is_empty() {
                *page_no += 1;
                pages.push(LaidPage {
                    section_id: section.id.value(),
                    page_no: *page_no,
                    lines: std::mem::take(current),
                    header: header_lines.to_vec(),
                    footer: footer_lines.to_vec(),
                });
                *started = false;
            }
        };

        for lb in &laid {
            match lb {
                LaidBlock::Break => {
                    started = true;
                    flush(
                        &mut pages,
                        &mut page_no,
                        &mut current,
                        &mut started,
                        &header_lines,
                        &footer_lines,
                    );
                }
                LaidBlock::Gap { height_pt } => {
                    if used_h + height_pt > body_budget && !current.is_empty() {
                        started = true;
                        flush(
                            &mut pages,
                            &mut page_no,
                            &mut current,
                            &mut started,
                            &header_lines,
                            &footer_lines,
                        );
                        used_h = 0.0;
                    }
                    used_h += height_pt;
                    started = true;
                }
                LaidBlock::Paragraph(para) => {
                    for line in &para.lines {
                        let lh = para.line_height_pt;
                        if used_h + lh > body_budget && !current.is_empty() {
                            started = true;
                            flush(
                                &mut pages,
                                &mut page_no,
                                &mut current,
                                &mut started,
                                &header_lines,
                                &footer_lines,
                            );
                            used_h = 0.0;
                        }
                        current.push(PlacedLine {
                            text: line.text.clone(),
                            x_pt: setup.margin_left_pt + line.offset_x_pt,
                            y_pt: setup.margin_top_pt + header_h + used_h,
                            width_pt: line.width_pt,
                        });
                        used_h += lh;
                        started = true;
                    }
                }
            }
        }
        flush(
            &mut pages,
            &mut page_no,
            &mut current,
            &mut started,
            &header_lines,
            &footer_lines,
        );
    }
    pages
}

fn laid_height_of(blocks: &[LaidBlock]) -> f64 {
    blocks
        .iter()
        .map(|b| match b {
            LaidBlock::Paragraph(p) => p.height_pt,
            LaidBlock::Gap { height_pt } => *height_pt,
            LaidBlock::Break => 0.0,
        })
        .sum()
}

fn place_chrome(blocks: &[LaidBlock], _section_id: u64, margin_left: f64) -> Vec<PlacedLine> {
    let mut out = Vec::new();
    let mut y = 0.0;
    for lb in blocks {
        match lb {
            LaidBlock::Paragraph(para) => {
                for line in &para.lines {
                    out.push(PlacedLine {
                        text: line.text.clone(),
                        x_pt: margin_left + line.offset_x_pt,
                        y_pt: y,
                        width_pt: line.width_pt,
                    });
                    y += para.line_height_pt;
                }
            }
            LaidBlock::Gap { height_pt } => {
                y += *height_pt;
            }
            LaidBlock::Break => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measure::FixedMeasurer;
    use aksara_model::{Block, CharacterStyle, NodeId, PageSetup, Paragraph, Section, TextRun};

    fn doc_with_texts(texts: &[&str]) -> Document {
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        for text in texts {
            let para_id = doc.alloc_id();
            let mut para = Paragraph::new(para_id);
            let run_id = doc.alloc_id();
            para.push_run(TextRun::new(run_id, text, CharacterStyle::default()));
            section.push_block(Block::Paragraph(para));
        }
        doc.push_section(section);
        doc
    }

    #[test]
    fn short_paragraph_is_one_page() {
        let m = FixedMeasurer::default();
        let pages = paginate(&doc_with_texts(&["hello world"]), &m);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].page_no, 1);
    }

    #[test]
    fn long_content_spills_pages() {
        let m = FixedMeasurer::default();
        // Narrow trick: many paragraphs force several pages.
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        for i in 0..200u64 {
            let mut para = Paragraph::new(NodeId::new(100 + i));
            para.push_run(TextRun::new(
                NodeId::new(1000 + i),
                "satu dua tiga empat lima",
                CharacterStyle::default(),
            ));
            section.push_block(Block::Paragraph(para));
        }
        doc.push_section(section);
        let pages = paginate(&doc, &m);
        assert!(pages.len() > 1);
    }

    #[test]
    fn page_break_forces_new_page() {
        let m = FixedMeasurer::default();
        let mut doc = doc_with_texts(&["satu", "dua"]);
        doc.sections[0].blocks.insert(1, Block::PageBreak);
        let pages = paginate(&doc, &m);
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].lines[0].text, "satu");
        assert_eq!(pages[1].lines[0].text, "dua");
    }

    #[test]
    fn two_sections_use_own_widths() {
        let m = FixedMeasurer::default();
        let mut doc = doc_with_texts(&["satu"]);
        let section_id = doc.alloc_id();
        let mut second = Section::new(section_id, PageSetup::a4_landscape());
        let para_id = doc.alloc_id();
        let mut para = Paragraph::new(para_id);
        let run_id = doc.alloc_id();
        para.push_run(TextRun::new(run_id, "dua", CharacterStyle::default()));
        second.push_block(Block::Paragraph(para));
        doc.push_section(second);
        let portrait_w = PageSetup::a4_portrait().content_width();
        let landscape_w = PageSetup::a4_landscape().content_width();
        assert!((landscape_w - portrait_w).abs() > 1.0);
        let pages = paginate(&doc, &m);
        assert_eq!(pages.len(), 2);
        assert_ne!(pages[0].section_id, pages[1].section_id);
    }

    #[test]
    fn coords_stay_inside_margins() {
        let m = FixedMeasurer::default();
        let pages = paginate(&doc_with_texts(&["hello world test"]), &m);
        let setup = PageSetup::a4_portrait();
        for page in &pages {
            for line in &page.lines {
                assert!(line.x_pt >= setup.margin_left_pt - 1e-9);
                assert!(line.x_pt + line.width_pt <= setup.width_pt - setup.margin_right_pt + 1e-9);
                assert!(line.y_pt >= setup.margin_top_pt - 1e-9);
                assert!(line.y_pt <= setup.height_pt - setup.margin_bottom_pt + 1e-9);
            }
        }
    }

    #[test]
    fn header_repeats_on_every_page() {
        let m = FixedMeasurer::default();
        let mut doc = Document::new();
        let section_id = doc.alloc_id();
        let mut section = Section::new(section_id, PageSetup::a4_portrait());
        for i in 0..200u64 {
            let mut para = Paragraph::new(NodeId::new(100 + i));
            para.push_run(TextRun::new(
                NodeId::new(1000 + i),
                "isi baris tubuh",
                CharacterStyle::default(),
            ));
            section.push_block(Block::Paragraph(para));
        }
        let mut head = Paragraph::new(NodeId::new(9000));
        head.push_run(TextRun::new(
            NodeId::new(9001),
            "kop dokumen",
            CharacterStyle::default(),
        ));
        section.set_header(vec![Block::Paragraph(head)]);
        doc.push_section(section);
        let before = doc.word_count();
        let pages = paginate(&doc, &m);
        assert!(pages.len() > 1);
        for page in &pages {
            assert_eq!(page.header.len(), 1);
            assert_eq!(page.header[0].text, "kop dokumen");
        }
        assert_eq!(doc.word_count(), before);
    }

    #[test]
    fn section_without_header_has_empty_chrome() {
        let m = FixedMeasurer::default();
        let pages = paginate(&doc_with_texts(&["hi"]), &m);
        assert_eq!(pages.len(), 1);
        assert!(pages[0].header.is_empty());
        assert!(pages[0].footer.is_empty());
    }

    #[test]
    fn footer_repeats_and_word_count_stable() {
        let m = FixedMeasurer::default();
        let mut doc = doc_with_texts(&["satu dua tiga"]);
        let before = doc.word_count();
        let mut foot = Paragraph::new(NodeId::new(8000));
        foot.push_run(TextRun::new(
            NodeId::new(8001),
            "halaman kaki",
            CharacterStyle::default(),
        ));
        doc.sections[0].set_footer(vec![Block::Paragraph(foot)]);
        assert_eq!(doc.word_count(), before);
        let pages = paginate(&doc, &m);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].footer.len(), 1);
        assert_eq!(pages[0].footer[0].text, "halaman kaki");
    }
}
