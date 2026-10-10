//! Permukaan - kartu, seksi, empty-state, banner.

use crate::theme::{GALAT, INFO, PERINGATAN, SUKSES};
use egui::{Frame, Response, RichText, Ui};

/// Kartu grup: judul + garis + isi.
pub fn card(ui: &mut Ui, judul: &str, isi: impl FnOnce(&mut Ui)) {
    Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(judul);
        ui.separator();
        isi(ui);
    });
}

/// Seksi berpenjelasan: judul + deskripsi redah + isi.
pub fn section(ui: &mut Ui, judul: &str, deskripsi: &str, isi: impl FnOnce(&mut Ui)) {
    ui.label(RichText::new(judul).strong());
    ui.label(RichText::new(deskripsi).small().weak());
    ui.add_space(4.0);
    isi(ui);
}

/// Tampilan kosong + tombol aksi. Mengembalikan `true` bila aksi diklik.
pub fn empty_state(ui: &mut Ui, ikon: &str, judul: &str, subjudul: &str, aksi: &str) -> bool {
    let mut klik = false;
    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.label(RichText::new(ikon).size(32.0));
        ui.heading(judul);
        ui.label(subjudul);
        ui.add_space(8.0);
        klik = crate::components::buttons::primary_button(ui, aksi).clicked();
    });
    klik
}

/// Jenis banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerKind {
    Info,
    Sukses,
    Peringatan,
    Galat,
}

impl BannerKind {
    fn warna(self) -> egui::Color32 {
        match self {
            BannerKind::Info => INFO,
            BannerKind::Sukses => SUKSES,
            BannerKind::Peringatan => PERINGATAN,
            BannerKind::Galat => GALAT,
        }
    }

    fn ikon(self) -> &'static str {
        match self {
            BannerKind::Info => "(i)",
            BannerKind::Sukses => "+",
            BannerKind::Peringatan => "(!)",
            BannerKind::Galat => "(x)",
        }
    }
}

/// Banner satu baris: ikon + teks, garis kiri berwarna.
pub fn notice(ui: &mut Ui, jenis: BannerKind, teks: &str) -> Response {
    let bingkai = Frame::group(ui.style()).stroke(egui::Stroke::new(1.0, jenis.warna()));
    bingkai
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(jenis.ikon()).color(jenis.warna()).strong());
                ui.label(teks);
            });
        })
        .response
}
