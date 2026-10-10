//! Merdeka UI — fondasi UI/UX desktop berbasis egui.
//!
//! Sementara di-vendor di `crates/merdeka-ui` (path lokal).
//! Nanti bisa dipindah ke repo/workspace bersama tanpa ubah API:
//! semua aplikasi Merdeka (Aksara, Layar, Matriks, Naskah, Pusat)
//! cukup depend ke crate ini + implement [`shell::MerdekaPage`].

pub mod components;
pub mod shell;
pub mod theme;

pub use shell::{MerdekaApp, MerdekaConfig, MerdekaPage, run_page};
pub use theme::{MerdekaTheme, ThemeMode};

/// Re-ekspor sekali pakai untuk aplikasi.
pub mod prelude {
    pub use crate::shell::{MerdekaApp, MerdekaConfig, MerdekaPage, run_page};
    pub use crate::theme::{MerdekaTheme, ThemeMode};
}
