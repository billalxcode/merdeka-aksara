# Merdeka Aksara File Formats

Status: Phase 5 subset. This file records the native format decision,
the DOCX subset supported by `aksara-io-docx`, and the mapping between
OOXML elements and `aksara-model` types.

## Decision: native first, DOCX as interop

- The native format is a JSON snapshot of the `aksara-model` document
  tree (one UTF-8 file, extension `.aksara.json`, mirroring `Document`,
  `Section`, `Block`, `Paragraph`, `TextRun`, and `Table`). It is the
  format the apps will open and save by default once file IO is wired
  into them. Rationale: it is debuggable as text, diff-friendly under
  version control, and writable with std only (no new dependencies).
- The native reader/writer is explicitly NOT implemented in this phase;
  only the decision is recorded here so later work has a fixed target.
- DOCX (Office Open XML, `.docx` = ZIP + XML) is the interop format:
  `aksara-io-docx` translates between the model and a DOCX subset so
  users can exchange files with Microsoft Word and LibreOffice Writer.
  The model never depends on OOXML; all adaptation lives in the IO
  crate, and no `aksara-model` API change is allowed to fit OOXML.

## DOCX subset: supported on read and write

Paragraphs and runs:

- `w:p` maps to `Paragraph` (via `Paragraph::new` + `push_run`).
- `w:r` maps to one `TextRun` (via `TextRun::new`); consecutive `w:t`
  elements inside one `w:r` are concatenated into that single run.
- Inline flags: `w:b` to `bold`, `w:i` to `italic`, `w:u` to
  `underline`. A missing flag element means false; an explicit
  `w:val="false"` (for `w:b`/`w:i`) or `w:val="none"` (for `w:u`)
  also means false. Other run properties (size, fonts, color,
  highlight, strike) are ignored.
- Paragraph alignment: `w:pPr/w:jc w:val` maps to `Alignment`
  (`left`/`start` to Left, `center` to Center, `right`/`end` to
  Right, `both`/`distribute` to Justify; missing means Left).
  Other paragraph properties (spacing, indent, numbering) are ignored.
- `xml:space="preserve"` is always written on `w:t` and honored on
  read; `&`, `<`, `>`, quotes are escaped on write and unescaped on
  read. A `w:tab` inside a run becomes one tab character.
- A soft break (`w:br` without `w:type="page"`) becomes one newline
  character in the run text; other inline markers carry no model
  meaning and are dropped. A page-type break is handled as below.

Tables:

- `w:tbl` maps to `Table` (via `Table::new`, so dimensions clamp to
  1..=MAX_ROWS rows and 1..=MAX_COLS columns); `w:tr` to rows,
  `w:tc` to cells. Cell content is paragraphs only
  (via `TableCell::push_block`); anything else in a cell is skipped.
- On write every cell holds at least one `w:p` (empty when the model
  cell is empty), because Word expects a block in each `w:tc`.

Breaks and sections:

- `Block::PageBreak` is written as its own paragraph containing
  `w:r`/`w:br w:type="page"`. On read, a paragraph that holds only
  page breaks (no text) becomes that many `PageBreak` blocks; a text
  paragraph followed by page breaks yields the paragraph first, then
  the breaks.
- Sections are written sequentially with plain content only. The
  final `w:sectPr` (page size and margins) comes from the last
  section's `PageSetup`; per-section `sectPr` details are not
  emitted. `SectionBreak` blocks, headers, footers, and images are
  skipped explicitly on write (never silently: see below).

## DOCX: explicitly NOT in the subset

Images and drawings (`w:drawing`, `wp:*`, `a:blip`), headers and
footers, footnotes and endnotes, comments, tracked changes, embedded
fonts, OLE objects, text boxes, SmartArt, charts, equations, section
breaks with per-section geometry, named styles beyond direct
formatting, and numbering/bullets/lists. Numbered paragraphs import
as plain paragraphs (numbering dropped); nested tables import as
nothing (the nested `w:tbl` is skipped and counted, the outer cell
keeps its paragraphs). Anything skipped on read is counted in
`ReadStats` (nested tables, images) so callers can warn instead of
losing content silently; anything skipped on write is documented
here and never mapped to a wrong element.

## Minimal valid DOCX layout (writer contract)

The writer emits a ZIP with exactly these entries, all with stored
(no-compression) packing: `[Content_Types].xml`, `_rels/.rels`, and
`word/document.xml`. No other parts are needed for Word or Writer to
open the file without a repair prompt, because the document uses no
images, hyperlinks, fonts, or numbered styles that would require
extra relationship parts. `word/document.xml` holds `w:document`
with the `w` (and `r`) namespaces, one `w:body` with the blocks in
order, and a closing `w:sectPr` with `pgSz`/`pgMar` converted from
`PageSetup` points to twentieths of a point (twips).

## Backend roadmap

- Now: Docx is fully wired through the `aksara-io` facade to the
  `aksara-io-docx` backend (read/write).
- Recognized but Unsupported: Doc, Odt, Pdf (enum variants exist;
  dispatch returns `IoError::Unsupported` naming format + path).
- Reserved variants: Markdown, Text (no backend planned yet).
- Rule: the extension wins on detect; magic bytes are fallback only
  (ZIP magic without an extension is assumed Docx).
