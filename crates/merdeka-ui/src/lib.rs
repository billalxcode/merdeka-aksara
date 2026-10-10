//! Merdeka UI - fondasi UI/UX desktop berbasis egui (tema terang).
//!
//! Sementara di-vendor di `crates/merdeka-ui` (path lokal).
//! Nanti bisa dipindah ke repo/workspace bersama tanpa ubah API:
//! semua aplikasi Merdeka (Aksara, Layar, Matriks, Naskah, Pusat)
//! cukup depend ke crate ini + implement [`shell::MerdekaPage`].

pub mod components;
pub mod icons;
pub mod shell;
pub mod theme;

pub use components::ribbon::Pita;
pub use shell::{MerdekaApp, MerdekaConfig, MerdekaPage, run_page};
pub use theme::MerdekaTheme;

/// Re-ekspor sekali pakai untuk aplikasi.
pub mod prelude {
    pub use crate::components::ribbon::Pita;
    pub use crate::icons::{icon, svg_path};
    pub use crate::shell::{MerdekaApp, MerdekaConfig, MerdekaPage, run_page};
    pub use crate::theme::MerdekaTheme;
}
