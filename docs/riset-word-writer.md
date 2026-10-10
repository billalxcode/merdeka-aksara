# Riset Microsoft Word & LibreOffice Writer untuk Merdeka Aksara

> Tujuan: fitur dan tampilan Aksara disamakan dengan yang sudah dikenal pengguna,
> agar migrasi dari Microsoft Office / LibreOffice ke Merdeka Aksara semudah mungkin.
> Dokumen ini hasil riset internet (Oktober 2026) + ringkasan dua sumber primer.

**Sumber primer:**
- Word Ribbon: https://www.computerhope.com/issues/ch002078.htm (diperbarui 09/07/2025)
- Writer UI variants (Writer Guide 26.2, Bab 21): https://books.libreoffice.org/en/WG262/WG26221-UserInterfaceVariants.html
- Perbandingan fitur (LO 26.8.1 vs MS 2024/365): https://wiki.documentfoundation.org/Feature_Comparison:_LibreOffice_-_Microsoft_Office
- Dukungan format ODT vs DOCX: https://support.microsoft.com/en-us/word/differences-between-the-opendocument-text-odt-format-and-the-word-docx-format

---

## 1. Ringkasan eksekutif

- **Microsoft Word** memakai satu pola UI: **Ribbon** (pita tab -> grup perintah).
  Tab baku: File, Home, Insert, Draw, Design, Layout, References, Mailings, Review, View (+ Help/Acrobat bila terinstal).
- **LibreOffice Writer** memakai konsep **MUFFIN** (banyak varian, bisa diganti via View -> User Interface),
  tapi varian **Tabbed** sengaja dibuat mirip Ribbon agar pengguna Word betah.
  Tab tetap Writer: File, Home, Insert, Layout, References, Review, View, Extension, Tools
  + tab kontekstual: Draw, Image, Media, Object, Table, Form.
- **Kesimpulan untuk Aksara:** tiru susunan **Tabbed ala Word** sebagai default
  (File-Home-Insert-Layout-References-Review-View), sediakan mode **Classic** (menu + toolbar)
  sebagai opsi. Prioritaskan parity fitur menulis harian dulu (format teks, styles,
  tabel, gambar, header/footer, TOC, find/replace, export PDF), baru fitur kolaborasi lanjutan.

---

## 2. Tampilan & fitur Microsoft Word (Ribbon)

### 2.1 Home - dipakai 80% waktu
- Clipboard: Paste, Cut, Copy, Format Painter.
- Font: jenis + ukuran, Bold/Italic/Underline, Strikethrough, Sub/Superscript,
  Text Effects, Highlight, Font Color.
- Paragraph: Bullets, Numbering, Multilevel List, Indent, Sort,
  Align (kiri/tengah/kanan/justify), Spacing, Shading, Borders.
- Styles: galeri style + preview (Normal, Heading 1-3, Title, ...).
- Editing: Find, Replace, Select. Plus: Dictate (suara->teks), Editor (ejaan/tata bahasa).

### 2.2 Insert - tambah objek
- Pages: Cover Page, Blank Page, Page Break.
- Tables: sisip tabel.
- Illustrations: Pictures, Shapes, Icons, 3D Models, SmartArt, Chart, Screenshot.
- Media: Online Video. Links: Link, Bookmark, Cross-reference. Comments.
- Header & Footer: Header, Footer, Page Number.
- Text: Text Box, Quick Parts, WordArt, Drop Cap, Signature Line, Date & Time, Object.
- Symbols: Equation, Symbol.

### 2.3 Design - rupa dokumen
- Themes, Colors, Fonts, Paragraph Spacing, Effects, Set as Default.
- Page Background: Watermark, Page Color, Page Borders.

### 2.4 Layout - tata halaman
- Page Setup: Margins, Orientation (portrait/landscape), Size, Columns (s.d. 13),
  Breaks, Line Numbers, Hyphenation.
- Paragraph: Indent kiri/kanan, Spacing Before/After.
- Arrange: Position, Wrap Text, Bring Forward/Send Backward, Selection Pane, Align, Group, Rotate.

### 2.5 References - dokumen panjang
- Table of Contents (tambah teks, update).
- Footnotes: Insert Footnote/Endnote, navigasi, Show Notes.
- Citations & Bibliography: Insert/Manage Source, Style (APA/Chicago/MLA), Bibliography.
- Captions: Insert Caption, Table of Figures, Cross-reference.
- Index: Mark Entry, Insert, Update. Table of Authorities.

### 2.6 Mailings - surat massal
- Envelopes, Labels, Start Mail Merge, Select/Edit Recipients,
  Address Block, Greeting Line, Insert Merge Field, Preview Results, Finish & Merge.

### 2.7 Review - proofing & kolaborasi
- Editor, Thesaurus, Word Count, Read Aloud (TTS), Accessibility Checker, Translate.
- Comments (baru/hapus/navigasi), Track Changes + Reviewing Pane,
  Accept/Reject, Compare, Restrict Editing, Block Authors.

### 2.8 View - cara lihat
- Read Mode, Print Layout, Web Layout, Outline, Draft.
- Focus, Immersive Reader, Ruler, Gridlines, Navigation Pane,
  Zoom (100%, One/Multiple Page, Page Width),
  New Window, Arrange All, Split, Side-by-Side, Macros.

---

## 3. Tampilan & fitur LibreOffice Writer

### 3.1 Varian UI (View -> User Interface)
1. **Standard/Classic** - menu bar + toolbar Standard + Formatting + Sidebar. Paling stabil, favorit pengguna lama.
2. **Single Toolbar** - satu toolbar isi perintah favorit + Sidebar disembunyikan.
3. **Tabbed (Notebookbar)** - mirip Ribbon Word. Ada Icon bar (Open/Save/Undo/Redo/Print),
   Tab bar, ikon tab aktif, menu tiap tab, Quick Menu di ujung kanan. Bila jendela sempit,
   perintah berlebih disembunyikan di tombol `>>`.
4. **Tabbed Compact** - tab sama, tiap tab sebaris ikon + dropdown.
5. **Groupedbar Compact** - perintah dikelompokkan jadi set ikon + menu.
6. **Contextual Single** - satu toolbar yang isinya mengikuti konteks kursor/seleksi.

### 3.2 Tab tetap Writer (varian Tabbed)
| Tab | Isi |
|-----|-----|
| File | New/Open/Save/Print/Close, Templates, Export PDF & EPUB, Properties, Digital Signature, Sign PDF |
| Home | Cut/Copy/Paste, format teks, sisip gambar/tabel/karakter/page-break, styles |
| Insert | Semua objek sisipan (gambar, tabel, chart, media, hyperlink, bookmark, field, simbol) |
| Layout | Tata halaman (margin, orientasi, ukuran, kolom, break, indent, spacing) |
| References | TOC, index, footnote/endnote, cross-ref, bibliografi, field |
| Review | Record/Show/Hide/Manage Track Changes |
| View | Tampilan layar (layout, ruler, navigator, zoom) |
| Extension | Dialog ekstensi + ikon ekstensi |
| Tools | Macro, Mail Merge, pembuatan form |

### 3.3 Tab kontekstual (muncul bila objek dipilih)
Draw (objek gambar), Image (crop, border, wrap, filter), Media (audio/video),
Object (posisi/ukuran/link/warna frame), Table (saat kursor di tabel), Form (mode desain).

### 3.4 Fitur yang hanya ada di Writer (relevan untuk Aksara)
- Master document + template stabil untuk dokumen sangat panjang (di Word rawan corrupt/deprecated).
- Word completion, markdown otomatis (`*bold*`, `/italic/`, `-coret-`, `_underline_`), kolom tabel tak terbatas (Word max 63), ukuran halaman s.d. 300x300 cm.
- Text box multi-kolom, kalkulasi kompleks di tabel, redaction/sanitasi dokumen.
- Export EPUB, MediaWiki (Wikipedia), XHTML, DocBook, export halaman sebagai JPG/PNG, Hybrid PDF (PDF berisi ODT sehingga bisa dibuka-edit penuh).
- Recovery file `.doc` rusak lebih baik. Import format legacy/kompetitor luas (AbiWord, Pages, MacWrite, ClarisWorks, Word DOS/6.0/95, dsb).

### 3.5 Fitur yang (masih) hanya di Word
- Table Styles penuh, Picture Styles + efek Glow/Soft, diagonal borders, vertical alignment halaman.
- Draft view, TTS Read Aloud bawaan, Translate bawaan, Live Preview, Screenshot insert, hapus background foto.
- Ink/Pen (gambar, lasso, ubah ink->shape/teks), outline numbering kompleks, Find & Replace advanced.
- Kolaborasi real-time + chat + @mention + co-authoring Teams (Writer desktop belum ada; kolaborasi sinkron hanya via Collabora Online).

---

## 4. Format file: DOCX vs ODT (penting untuk migrasi)

- **Default Aksara harus dua-duanya:** baca/tulis `.docx` (kompatibilitas dunia) + `.odt` (format terbuka). Simpan-asli disarankan ODT; ekspor berbagi disarankan DOCX/PDF.
- Word membuka ODT tapi dukungan ODF-nya terbatas (Windows ODF 1.4 terbatas, online 1.3); fitur seperti document protection, IRM, Track Changes (di-accept semua), bibliografi, TOC/TOF/TOA, Themes, Watermark, macro, enkripsi **hilang/berubah** saat simpan Word->ODT (lihat tabel Microsoft di sumber).
- Writer membuka DOCX nyata dengan baik (import OOXML strict + transitional), tapi Table Styles, efek gambar, dan kolaborasi canggih bisa bergeser.
- PDF: keduanya bisa simpan PDF; Writer lebih kaya opsi (Tagged PDF, Hybrid, PAdES, PDF/UA, komen di margin/akhir halaman/dokumen).

---

## 5. Peta migrasi -> Merdeka Aksara

### 5.1 Prinsip UX: jangan paksa belajar ulang
1. **Default = Tabbed ala Word** (File-Home-Insert-Layout-References-Review-View).
   Urutan dan nama grup disamakan (Clipboard, Font, Paragraph, Styles, Editing, ...).
2. **Sediakan mode Classic** (menu File/Edit/View/Insert/Format/Styles/Table/Tools/Window/Help + toolbar + sidebar) untuk pengguna LibreOffice lama.
3. **Command palette + Tell Me** (`Ctrl+K` / `Shift+Esc`): ketik nama perintah, cocok untuk semua kebiasaan.
4. **Shortcut disamakan:** `Ctrl+B/I/U`, `Ctrl+C/X/V`, `Ctrl+Z/Y`, `Ctrl+S/O/P/F/H`, `Ctrl+Home/End`, `F7` ejaan, `F12` Save As. Di macOS petakan `Ctrl`->`Cmd`.

### 5.2 Prioritas fitur (MVP -> lengkap)
- **P0 (MVP mengetik):** dokumen multi-halaman, font + paragraf dasar, bullets/numbering,
  styles (Normal/Heading 1-3/Title), undo/redo, clipboard + format painter,
  find/replace, zoom, word count, simpan/buka DOCX+ODT+Markdown, export PDF.
- **P1 (tata letak):** margin/orientasi/ukuran/kolom, header/footer + nomor halaman,
  tabel (sisip/hapus baris-kolom, merge, borders/shading), gambar (sisip/resize/wrap/anchor),
  text box, page/section break, print preview.
- **P2 (dokumen panjang):** TOC otomatis, footnotes/endnotes, captions + cross-ref,
  index, bibliography dasar, templates, master document sederhana.
- **P3 (review):** comments, track changes dasar + reviewing pane, compare, thesaurus,
  ejaan dasar (hunspell), translate via layanan luar.
- **P4 (power):** mail merge wizard, form fields, macro/script sederhana, EPUB/MediaWiki export,
  Hybrid PDF, kolaborasi (nanti, via layanan daring).

### 5.3 Padanan cepat Word -> Aksara (untuk cheatsheet migrasi)
| Di Word | Di Aksara (rencana) |
|---|---|
| Home -> Font/Paragraph/Styles | Home -> grup sama, ikon + nama sama |
| Insert -> Table/Pictures/Header-Footer/Page Number | Insert -> sama |
| Layout -> Margins/Orientation/Columns | Layout -> sama |
| References -> Table of Contents/Footnotes | References -> sama |
| Review -> Track Changes/Comments | Review -> sama (bertahap, tanpa co-authoring dulu) |
| File -> Save As .docx/.pdf | File -> Save As .odt/.docx/.md/.pdf/.epub |
| Tell Me / Search commands | Command palette `Ctrl+K` |

### 5.4 Risiko kompatibilitas yang harus diuji
- Round-trip DOCX->Aksara->DOCX: styles bertambah (wajar, ODF style-based), spacing bullet/list bisa beda tipis.
- Tabel >64 kolom, text box bersarang, SmartArt->group shapes, gradient >2 stops, font tbrlv/btlr.
- Track Changes Word dibuka di Aksara: anggap accepted (seperti perilaku ODT) + beri peringatan ke pengguna.
- Selalu sediakan **pratinjau sebelum simpan** + mode **read-only** untuk file asing.

---

## 6. Langkah lanjut yang disarankan
1. Implementasi chrome Tabbed P0 di `crates/merdeka-ui` (tab + grup + command palette stub).
2. Model dokumen + render teks P0 (lihat bagian 5.2).
3. IO DOCX/ODT/Markdown + export PDF.
4. Cheatsheet migrasi 1 halaman (ID) + impor template `.dotx/.ott`.
5. Uji round-trip dengan 20 dokumen nyata Word/LibreOffice.
