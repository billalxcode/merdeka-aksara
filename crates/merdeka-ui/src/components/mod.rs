//! Komponen reusable Merdeka - satu tampilan, dipakai semua aplikasi.
//!
//! Struktur:
//! - `buttons` - tombol primer/sekunder/ghost/ikon + chip toggle
//! - `inputs` - field teks, cari, angka, checkbox, radio, slider, dropdown, warna
//! - `ribbon` - tab pita + grup + tombol alat ala Word
//! - `navigation` - bar judul + seksi sidebar
//! - `surfaces` - kartu, seksi, empty-state, banner
//! - `overlays` - dialog modal + konfirmasi
//! - `status` - bar status + zoom
//! - `editor` - kanvas dokumen (scroll + kertas + ruler)

pub mod buttons;
pub mod editor;
pub mod inputs;
pub mod navigation;
pub mod overlays;
pub mod ribbon;
pub mod status;
pub mod surfaces;

pub use buttons::*;
pub use editor::*;
pub use inputs::*;
pub use navigation::*;
pub use overlays::*;
pub use ribbon::*;
pub use status::*;
pub use surfaces::*;
