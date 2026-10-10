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

use merdeka_ui::{MerdekaConfig, run_page};
use presentation::page::AksaraPage;

fn main() -> eframe::Result {
    let config = MerdekaConfig::new("Merdeka Aksara", env!("CARGO_PKG_VERSION"));
    run_page(config, |_cc| AksaraPage::baru())
}
