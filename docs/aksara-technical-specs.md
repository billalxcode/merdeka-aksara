# Merdeka Aksara: Arsitektur Teknis Layout Word dengan Pure Rust

Kalau Merdeka Aksara ingin menjadi alternatif Microsoft Word, hal terpenting adalah memahami bahwa Word bukan sekadar text editor dengan toolbar. Word adalah document editor yang memiliki layout engine, model dokumen, sistem pagination, dan renderer sendiri.

Karena targetmu adalah pure Rust implementation dan open source, saya menyarankan agar Merdeka Aksara dibangun sebagai aplikasi native Rust, dengan model dokumen dan layout engine yang tidak bergantung pada browser atau WebView.

Kita perlu memisahkan tiga hal yang sering dianggap sama:

- UI layout: Ribbon, toolbar, sidebar, status bar, dan panel navigasi.
- Document layout: halaman A4, margin, paragraf, tabel, gambar, header/footer, dan pemenggalan halaman.
- Rendering engine: bagaimana teks dan elemen dokumen dihitung posisinya lalu digambar ke layar dan diekspor ke PDF.

Ketiganya berhubungan, tetapi secara teknis sebaiknya menjadi subsistem yang berbeda.

## 1. UI layout: komponen yang terlihat pengguna

Microsoft Word desktop secara umum menggunakan konsep Ribbon UI + document canvas + status bar. Kamu tidak perlu menyalin semua detail visual Word, tetapi struktur kerjanya bisa dijadikan referensi.

[Word Workspace - Microsoft Word 2013 Basics - ULibraries Research Guides at University of Utah](https://images.openai.com/static-rsc-4/fVMaPmD6_ehy4s9ll5x_gPDNUskfc2XVUSG2s5qS9u3yQkvL8FMjCKtlrxE2GhNLg4JJFEz6dyyHPXhZv4nT_Qyaz1L9HIDioojFj265CbvBvg5atIEox0Sa7MVjvM2TsubvwNCyUue-6oWRwKQ7j7JE1VGuumCLVoAcy2p9yfQ?purpose=inline)

[campusguides.lib.utah.edu](https://campusguides.lib.utah.edu/c.php?g=160452\&p=1051033)

Secara teknis, layout utamanya dapat dibagi menjadi bagian berikut.

1\. Title bar

Nama dokumen, save status, window controls, account atau app menu.

2\. Quick Access Toolbar

Save, Undo, Redo, dan shortcut yang dapat dikustomisasi.

3\. Ribbon

File

Home

Insert

Layout

References

Review

View

Tab mengubah kumpulan command yang tampil di area ribbon.

Navigation

Opsional

Page 1

Document content

Text · Images · Tables

Page margins

Document canvas

4\. Status bar

Page X of Y, word count, language, zoom, view mode.

### Komponen UI dan tanggung jawabnya

| Komponen             | Fungsi teknis                                 |
| -------------------- | --------------------------------------------- |
| `TitleBar`           | Judul file dan status penyimpanan             |
| `QuickAccessToolbar` | Aksi umum yang selalu tersedia                |
| `Ribbon`             | Container untuk tab dan command groups        |
| `RibbonTab`          | Mengelompokkan command berdasarkan tugas      |
| `CommandButton`      | Menjalankan satu aksi                         |
| `FontControls`       | Font family, size, bold, italic, underline    |
| `ParagraphControls`  | Alignment, spacing, indent, bullets           |
| `DocumentCanvas`     | Area tampilan dokumen                         |
| `PageView`           | Menampilkan satu halaman beserta kontennya    |
| `Ruler`              | Menampilkan dan mengubah indent/tab stops     |
| `NavigationPane`     | Search, headings, navigasi halaman            |
| `SelectionToolbar`   | Aksi kontekstual berdasarkan seleksi          |
| `StatusBar`          | Statistik dokumen dan status editor           |
| `ZoomControl`        | Mengubah skala tampilan, bukan ukuran dokumen |

Catatan arsitektur: `PageView` bukan tempat untuk menyimpan isi dokumen. Komponen itu hanya menampilkan hasil layout dari model dokumen. Dengan pemisahan ini, tampilan yang sama dapat digunakan oleh mode Print Layout, Read Mode, atau export preview.

## 2. Struktur Ribbon secara teknis

Jangan implementasikan Ribbon sebagai satu komponen raksasa yang berisi semua tombol. Gunakan struktur hierarkis.

Ribbon

Tab

Home

Tab

Insert

Tab

Layout

Home → Groups

Clipboard

Font

Paragraph

Setiap group berisi command buttons, dropdowns, atau controls.

### Tab yang sebaiknya disiapkan

| Tab        | Command utama                                              |
| ---------- | ---------------------------------------------------------- |
| File       | New, Open, Save, Save As, Export, Print                    |
| Home       | Clipboard, font, paragraph, styles, find/replace           |
| Insert     | Table, Picture, Link, Shapes, Page Number, Header/Footer   |
| Layout     | Margins, Orientation, Paper Size, Columns, Indent, Spacing |
| References | Table of Contents, Footnotes, Citations, Captions          |
| Review     | Spelling, Comments, Track Changes                          |
| View       | Zoom, Ruler, Navigation Pane, page view                    |
| Help       | Documentation, About, shortcuts                            |

Mulailah dengan File, Home, Insert, Layout, dan View. Tab References dan Review dapat ditambahkan setelah model dokumen serta editing engine stabil.

## 3. Document model: representasi isi dokumen

Ini bagian paling penting untuk Merdeka Aksara.

Jangan menyimpan dokumen sebagai satu `String` besar. Ketika pengguna membuat teks bold di tengah paragraf, memasukkan tabel, mengubah margin, atau menambahkan halaman, kamu membutuhkan struktur yang mampu merepresentasikan semua itu.

Microsoft Word menggunakan format OOXML untuk DOCX. Struktur intinya mencakup dokumen, paragraf, text runs, tabel, style, serta bagian dokumen lain seperti header dan footer.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://learn.microsoft.com\&sz=32)

Microsoft Learn

+1



Model internal Merdeka Aksara bisa dirancang seperti ini:

Document

Metadata, settings, styles

Section

Page size, margins, orientation, columns

Block nodes

Paragraph · Table · Image · Page Break

Paragraph

Text runs

Bold · Italic · Links

Table

Rows → Cells

Cell content → Blocks

Contoh tipe data Rust sebagai titik awal:

```
struct Document {
    metadata: DocumentMetadata,
    sections: Vec<Section>,
    styles: StyleSheet,
}

struct Section {
    page_setup: PageSetup,
    blocks: Vec<Block>,
}

enum Block {
    Paragraph(Paragraph),
    Table(Table),
    Image(Image),
    PageBreak,
}

struct Paragraph {
    runs: Vec<TextRun>,
    style: ParagraphStyle,
}

struct TextRun {
    text: String,
    style: CharacterStyle,
}

struct PageSetup {
    width: f32,
    height: f32,
    margin_top: f32,
    margin_bottom: f32,
    margin_left: f32,
    margin_right: f32,
}
```

Ini hanya sketsa awal, bukan model lengkap. Implementasi sesungguhnya juga memerlukan stable node IDs, selection ranges, inline objects, numbering, bookmarks, section breaks, dan format style inheritance.

Penting: halaman bukan node permanen dalam dokumen. Halaman merupakan hasil perhitungan layout dari konten dan konfigurasi section. Satu paragraf bisa berlanjut ke halaman berikutnya. Konsep section di OOXML juga mendefinisikan pengaturan halaman untuk kelompok konten, bukan menyimpan daftar halaman hasil rendering.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://ooxml.info\&sz=32)

OOXML Info

+1



## 4. Layout engine: bagaimana teks berubah menjadi halaman?

Layout engine adalah subsistem yang mengubah model dokumen menjadi posisi visual.

Alur sederhananya:

1\. Document model

Paragraphs, styles, tables, images

2\. Style resolution

Default → named style → direct formatting

3\. Text shaping

Font metrics, glyphs, script shaping

4\. Line breaking

Wrap teks sesuai lebar area

5\. Block layout

Hitung tinggi paragraf, tabel, gambar

6\. Pagination

Tempatkan blok dan baris pada halaman

7\. Display list

Glyphs, paths, images, clipping, coordinates

Display list kemudian diteruskan ke renderer layar atau PDF.

### Urutan implementasi yang benar

1. Text shaping: menentukan glyph, ukuran, dan posisi teks berdasarkan font.
2. Line breaking: memecah teks menjadi baris berdasarkan lebar yang tersedia.
3. Paragraph layout: menghitung line height, spacing, alignment, dan indentation.
4. Block layout: menempatkan paragraf, tabel, serta gambar secara berurutan.
5. Pagination: menentukan apakah baris atau blok muat di halaman saat ini atau harus berpindah.
6. Rendering: menggambar hasil layout menggunakan koordinat yang telah dihitung.

Jangan mulai dengan pagination sebelum text shaping dan line breaking berfungsi dengan benar. Jika lebar teks atau tinggi baris berubah, posisi semua konten setelahnya berpotensi ikut berubah.

### Library Rust yang layak dievaluasi

Parley

Untuk shaping, line breaking, alignment, dan rich-text layout. API-nya menyediakan `FontContext`, `LayoutContext`, dan `Layout`, yang dapat menjadi fondasi subsystem teks.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://docs.rs\&sz=32)

Rust



Dokumentasi: [docs.rs/parley](https://docs.rs/parley/latest/parley/)

[Linebender in November 2025 - Linebender](https://images.openai.com/static-rsc-4/EceLBSDxdMBz2sTFUuDND_uqwqsAcYdLydp2aOqLDyNUyqCpJ2T18E0SuQ8l3EsR4B-Asi4LCCINkoCdju4l4fMNdrx3JN0eTWO_QBmtidZxUXUyd1v3qCkSK2Xa-7UU2PWg7Q9B_zKrsvtZwqUGx4-SMr2UdK8cRI3570TiL5Q?purpose=inline)

Vello

Kandidat untuk rendering 2D yang dipercepat GPU. Evaluasi integrasi, text rendering, kebutuhan platform, dan kesiapan produksinya sebelum menjadikannya dependensi inti.

Dokumentasi: [Vello di GitHub](https://github.com/linebender/vello)

Pure Rust tidak berarti harus menulis ulang seluruh algoritma font shaping dan rendering dari nol. Kamu tetap dapat memakai crate Rust sebagai dependency sambil mempertahankan implementasi aplikasi dan core engine dalam Rust. Tentukan secara eksplisit apakah ketentuan pure Rust juga melarang native system libraries atau hanya melarang penggunaan core engine berbasis C/C++.

## 5. Sistem koordinat dan ukuran halaman

Gunakan unit dokumen yang independen dari resolusi layar. Salah satu pendekatan yang mudah adalah menyimpan ukuran dalam point (pt) atau unit internal tetap, lalu mengubahnya ke pixel ketika rendering.

Contoh halaman A4:

### A4 portrait

210 × 297 mm

Margin di luar area konten, tidak termasuk teks dokumen

Lebar halaman

## 595.28 pt

Tinggi halaman

## 841.89 pt

Konversi dasar:

\\[ 1\text{ inch}=72\text{ pt} \\]

\\[ 1\text{ inch}=25.4\text{ mm} \\]

Untuk preview pada layar, jika memakai DPI 96:

\\[ px=pt\times\frac{96}{72}\times zoom \\]

`zoom` di sini adalah faktor skala, misalnya `1.0` untuk 100%. DPI preview tidak boleh mengubah ukuran fisik dokumen saat diekspor ke PDF.

Sebaiknya simpan koordinat dalam satuan logis dan gunakan `f64` atau representasi fixed-point untuk ukuran dokumen, lalu konversi ke koordinat renderer. Hindari menjadikan pixel layar sebagai sumber kebenaran ukuran dokumen.

## 6. Editing engine: mengetik, selection, dan undo

Satu subsistem lagi yang tidak boleh dilupakan adalah editing engine. Renderer hanya menggambar dokumen; editing engine mengubah dokumen secara konsisten.

Komponen utamanya:

| Komponen             | Tanggung jawab                                  |
| -------------------- | ----------------------------------------------- |
| `DocumentModel`      | Menyimpan struktur konten                       |
| `Selection`          | Menyimpan anchor dan cursor                     |
| `InputController`    | Menangani keyboard, mouse, clipboard            |
| `CommandDispatcher`  | Mengubah aksi pengguna menjadi command          |
| `TransactionManager` | Menggabungkan operasi menjadi perubahan logis   |
| `UndoRedoManager`    | Membatalkan dan mengulangi perubahan            |
| `LayoutCache`        | Menyimpan hasil layout yang masih valid         |
| `HitTester`          | Mengubah koordinat mouse menjadi posisi dokumen |

Contohnya, ketika pengguna menekan tombol Bold:

1. Ambil selection aktif.
2. Jika selection mencakup teks, ubah atribut style pada range tersebut.
3. Jika selection berupa caret kosong, ubah pending typing style untuk teks berikutnya.
4. Tandai paragraf terkait sebagai dirty.
5. Hitung ulang layout yang terdampak.
6. Render ulang area yang berubah.
7. Catat transaksi untuk undo/redo.

Hindari merender ulang seluruh dokumen untuk setiap karakter. Gunakan invalidation dan cache per paragraf atau blok, lalu perluas perhitungan ulang hanya ketika pagination memerlukan perubahan pada konten berikutnya.

## 7. Susunan crate yang saya rekomendasikan

Untuk Merdeka Aksara, saya akan memisahkan workspace berdasarkan tanggung jawab, bukan berdasarkan setiap widget UI.

```
merdeka-aksara/
├── Cargo.toml
├── crates/
│   ├── aksara-model/
│   ├── aksara-edit/
│   ├── aksara-style/
│   ├── aksara-layout/
│   ├── aksara-render/
│   ├── aksara-fonts/
│   ├── aksara-io-docx/
│   ├── aksara-io-pdf/
│   └── aksara-commands/
├── apps/
│   └── desktop/
│       └── src/
│           ├── shell/
│           ├── ribbon/
│           ├── canvas/
│           ├── panels/
│           └── main.rs
├── tests/
│   ├── layout/
│   ├── pagination/
│   └── docx-compat/
└── docs/
    ├── architecture.md
    └── file-format.md
```

Batas dependensi yang diinginkan:

- `aksara-model` tidak bergantung pada UI atau renderer.
- `aksara-edit` mengubah model melalui command/transaction.
- `aksara-layout` menghitung geometri tanpa menggambar ke layar.
- `aksara-render` menggambar hasil layout.
- `aksara-io-docx` menerjemahkan antara model internal dan OOXML.
- `apps/desktop` menghubungkan seluruh subsystem.

## 8. Roadmap implementasi yang realistis

1. Fondasi editor: window, ribbon minimal, model paragraf, caret, selection, keyboard input, copy/paste, undo/redo.
2. Layout halaman: ukuran kertas, margin, font metrics, wrapping, alignment, multipage pagination, zoom.
3. Formatting: bold, italic, underline, paragraph spacing, styles, heading, list dan numbering.
4. Dokumen kompleks: tabel, gambar, header/footer, section breaks, page numbering, footnotes.
5. File interoperability: pembacaan dan penulisan DOCX, export PDF, font fallback, round-trip tests.
6. Fitur produksi: find/replace, spell checking, accessibility, print support, crash recovery, dan optimasi dokumen besar.

Jangan menargetkan kompatibilitas sempurna dengan Microsoft Word pada tahap awal. Tetapkan subset fitur dan format yang didukung, kemudian perluas berdasarkan test corpus dokumen nyata.

## Rekomendasi final

Untuk Merdeka Aksara, saya akan menggunakan native Rust GUI + model dokumen sendiri + layout engine sendiri + Parley untuk dievaluasi sebagai text-layout dependency. Rendering dapat memakai backend 2D yang sesuai dengan kebutuhan platform, sedangkan interoperabilitas DOCX dipisahkan ke crate khusus.

Ada satu keputusan arsitektur yang perlu kamu tentukan sebelum masuk ke implementasi detail:

1\. Framework GUI native Rust

iced: UI deklaratif dan struktur aplikasi terorganisasi

egui: iterasi cepat, immediate-mode UI

Framework Rust lain atau custom renderer

2\. Target format dokumen

Format native Merdeka Aksara terlebih dahulu

DOCX sebagai format utama

Format native dan interoperabilitas DOCX sejak awal

3\. Kedalaman rancangan

UI layout dan komponen saja

Layout engine dan pagination secara mendalam

Arsitektur lengkap, crate boundaries, API Rust, dan roadmap

Rancang arsitektur Merdeka Aksara