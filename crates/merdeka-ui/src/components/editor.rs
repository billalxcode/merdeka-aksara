//! Kanvas editor - scroll + kertas A4 + ruler.

use crate::theme::{GARIS, KERTAS, TEKS_REDAH};
use egui::{Frame, Margin, RichText, Ui};

/// Bungkus area dokumen dengan scroll vertikal + lebar maksimum konsisten.
pub fn document_scroll(ui: &mut Ui, lebar_max: f32, isi: impl FnOnce(&mut Ui)) {
    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(lebar_max);
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), isi);
            });
        });
}

/// Kertas putih tempat teks dokumen ditulis.
pub fn page_paper(ui: &mut Ui, isi: impl FnOnce(&mut Ui)) {
    let bingkai = Frame::default()
        .fill(KERTAS)
        .stroke(egui::Stroke::new(1.0, GARIS))
        .corner_radius(egui::CornerRadius::same(4))
        .inner_margin(Margin::same(28));
    bingkai.show(ui, isi);
}

/// Penggaris tipis di atas kertas (stub visual, belum interaktif).
pub fn ruler_stub(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("0  1  2  3  4  5  6  7  8  9  10 cm").small().color(TEKS_REDAH));
    });
}
