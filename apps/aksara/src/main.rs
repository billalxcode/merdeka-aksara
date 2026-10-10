//! Merdeka Aksara - aplikasi tulis dokumen yang bebas.
//!
//! Struktur layer:
//! - `domain`: aturan bisnis murni (dokumen, gaya)
//! - `application`: state + kasus pakai
//! - `infrastructure`: berkas dan dunia luar
//! - `presentation`: seluruh kode egui

mod application;
mod domain;
mod infrastructure;
mod presentation;

use merdeka_ui::{MerdekaConfig, MerdekaTheme, run_page};
use presentation::page::AksaraPage;

fn main() -> eframe::Result {
    let mut config = MerdekaConfig::new("Merdeka Aksara", env!("CARGO_PKG_VERSION"));
    config.tema = MerdekaTheme::aksara();
    run_page(config, |_cc| AksaraPage::baru())
}
