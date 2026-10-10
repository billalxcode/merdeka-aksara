//! Merdeka UI - fondasi UI/UX desktop berbasis egui (tema terang).
//!
//! Sementara di-vendor di `crates/merdeka-ui` (path lokal).
//! Nanti bisa dipindah ke repo/workspace bersama tanpa ubah API:
//! semua aplikasi Merdeka (Aksara, Layar, Matriks, Naskah, Pusat)
//! cukup depend ke crate ini + implement [`shell::MerdekaPage`].

pub mod components;
pub mod shell;
pub mod theme;

/// Seluruh ikon Tabler (crate `merdeka-icons`, enum + autoload).
pub use merdeka_icons as icons;

pub use components::ribbon::Pita;
pub use icons::{Filled, Icon, Outline, icon, icon_named, svg_path};
pub use shell::{MerdekaApp, MerdekaConfig, MerdekaPage, run_page};
pub use theme::MerdekaTheme;

/// Re-ekspor sekali pakai untuk aplikasi.
pub mod prelude {
    pub use crate::components::ribbon::Pita;
    pub use crate::icons::{Filled, Icon, Outline, icon, icon_named, svg_path};
    pub use crate::shell::{MerdekaApp, MerdekaConfig, MerdekaPage, run_page};
    pub use crate::theme::MerdekaTheme;
}
