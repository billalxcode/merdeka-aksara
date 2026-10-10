# Merdeka Aksara Architecture

Status: Phase 2 (layout engine). This file maps crates, states
dependency boundaries, and marks what is done versus later work.
Details stay in `docs/aksara-technical-specs.md`.

## Crate map

Workspace members are `apps/*` (binaries) and `crates/*` (libs).

```text
apps/
  aksara   # word processor (legacy domain::Dokumen, egui UI)
  matriks  # spreadsheet app
  naskah   # text editor app
  pusat    # launcher app
crates/
  merdeka-ui     # shared egui foundation (theme, shell, components)
  merdeka-icons  # Tabler icon enum + autoloader (build.rs generated)
   aksara-model   # Phase 1: pure block-based document model (std only)
   aksara-layout  # Phase 2: layout geometry (std + aksara-model only)
   aksara-edit    # Phase 3: selection, command/tx, undo/redo (std + aksara-model only)
```

Future crates (names only, not created yet):

```text
crates/
  aksara-edit      # commands, transactions, undo/redo
  aksara-style     # style resolution (default -> named -> direct)
  aksara-render    # draw layout display lists (screen, PDF)
  aksara-fonts     # font loading and metrics
  aksara-io-docx   # model <-> OOXML translation
  aksara-io-pdf    # PDF export
  aksara-commands  # user-facing command dispatch
```

## Dependency boundaries

```text
apps/aksara  -->  merdeka-ui  -->  merdeka-icons
apps/*       -->  aksara-model   (later phases; NOT wired in Phase 1)
aksara-edit/render/io  -->  aksara-model  (later phases)
aksara-layout -->  aksara-model  (std only, no egui)
aksara-model -->  (nothing: std only)
```

Rules:

1. `aksara-model` has zero dependencies: no `[dependencies]` in its
   `Cargo.toml`, no `egui`, no native system libs. It must never
   import from `apps/*`, `merdeka-ui`, or `merdeka-icons`.
2. Later crates depend toward the model, never the reverse: layout,
   edit, render, and IO crates may use `aksara-model`; the model
   must not know about them.
3. The legacy app model (`apps/aksara` `domain::Dokumen`, a single
   `String` plus style) keeps running untouched in Phase 1. The new
   model uses English names (`Document`, `Section`, `Block`,
   `Paragraph`, `TextRun`); mapping or unifying the two style
   systems is explicitly later work, not this phase.
4. Shared UI stays in `merdeka-ui` (theme tokens, shell, components).
   The model holds no colors, no theme, no widgets.

## Phase position

Done in Phase 1 (this turn):

- `aksara-model`: units (mm/pt/inch/px) and A4 constants, `PageSetup`
  with portrait/landscape and margin validation, `DocumentMetadata`,
  per-document monotone `NodeId`, `CharacterStyle`/`ParagraphStyle`/
  `TextRun`/`Paragraph` with word counts, `Block` (Paragraph, Table,
  Image placeholder, PageBreak), recursive `Table` (rows 1-10, cols
  1-8), `Section`, minimal `StyleSheet` (built-in "Normal"),
  `Document` with `alloc_id`, word/block counts, and a sane default
  (one A4 portrait section plus one empty paragraph).
- Unit tests for every module (see plan `.caches/aksara-model-plan.md`
  for the per-module list).

Done in Phase 2 (this turn):

- `aksara-model`: `Section.header`/`footer` (`Vec<Block>`,
  excluded from `word_count`/`block_count`) with
  `with_header`/`with_footer`/`set_header`/`set_footer`/
  `clear_header`/`clear_footer`; old tests untouched.
- `aksara-layout` (new, std + `aksara-model` only): `FixedMeasurer`
  stub, greedy line breaking, paragraph layout (height, spacing,
  alignment, first-line indent), ordered block layout, A4
  multi-section pagination with per-page display lists, repeating
  header/footer chrome measured once per section and subtracted
  from the body budget.
- Ribbon: B/I/U command buttons now `tool_toggle` icons
  (`Outline::Bold`/`Italic`/`Underline`); style chips stay text,
  dialog buttons stay text.

Done in Phase 3 (this turn):

- `aksara-edit` (new, std + `aksara-model` only): `Selection`
  anchor + cursor with char offsets, `EditCommand` insert/delete
  with `Transaction` dispatch, `UndoRedoManager` snapshot pairs
  (cap 100); single-run text paragraphs only.

Explicit non-goals (later phases, do not add here):

- Layout remainder: real shaping, font metrics, layout cache,
  hit-testing, hyphenation, widow/orphan control, full
  table/image layout (only 0.0-height placeholders now).
- Rendering: screen drawing, GPU backends, PDF output.
- File IO: DOCX/OOXML, PDF, EPUB, import/export of any kind.
- Editing engine: selection, caret, undo/redo, transactions.
- Rewiring `apps/aksara` onto the new model.
- New third-party dependencies in `aksara-model`.
