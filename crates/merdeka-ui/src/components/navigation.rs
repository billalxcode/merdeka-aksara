//! Navigasi - bar judul + seksi sidebar.

use crate::icons::{Icon, Outline};
use crate::theme::MERAH;
use egui::{RichText, Ui};

/// Bar judul: logo merah + nama + versi + kolom cari.
pub fn title_bar(ui: &mut Ui, nama_app: &str, versi: &str, cari: &mut String) {
    ui.horizontal(|ui| {
        let logo = egui::Button::new(RichText::new("M").color(egui::Color32::WHITE).strong())
            .fill(MERAH)
            .frame(true);
        ui.add_sized([28.0, 28.0], logo)
            .on_hover_text("Merdeka - bebas pakai, bebas salin, bebas edit");
        ui.vertical(|ui| {
            ui.label(RichText::new(nama_app).strong());
            ui.label(RichText::new(format!("v{versi}")).small().weak());
        });
        ui.separator();
        ui.add(Icon::Outline(Outline::Search).gambar(14.0));
        crate::components::inputs::search_field(ui, cari);
    });
}

/// Seksi lipat di sidebar (Navigasi, Gaya, ...).
pub fn sidebar_section<R>(
    ui: &mut Ui,
    judul: &str,
    terbuka_default: bool,
    isi: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    egui::CollapsingHeader::new(judul)
        .default_open(terbuka_default)
        .show(ui, isi)
        .body_returned
}
