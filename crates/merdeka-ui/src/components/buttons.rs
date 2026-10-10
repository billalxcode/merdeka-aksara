//! Tombol - konsisten di semua aplikasi.

use crate::theme::{MERAH, TEKS_REDAH};
use egui::{Response, RichText, Ui};

/// Tombol primer merah. Untuk aksi utama tiap grup/dialog.
pub fn primary_button(ui: &mut Ui, label: &str) -> Response {
    ui.add(
        egui::Button::new(RichText::new(label).color(egui::Color32::WHITE).strong()).fill(MERAH),
    )
}

/// Tombol sekunder bingkai. Untuk aksi biasa.
pub fn secondary_button(ui: &mut Ui, label: &str) -> Response {
    ui.button(label)
}

/// Tombol ghost tanpa bingkai. Untuk aksi tersier.
pub fn ghost_button(ui: &mut Ui, label: &str) -> Response {
    ui.add(egui::Button::new(RichText::new(label).color(TEKS_REDAH)).frame(false))
}

/// Tombol ikon kecil dengan tooltip.
pub fn icon_button(ui: &mut Ui, ikon: &str, tooltip: &str) -> Response {
    ui.add(egui::Button::new(ikon).frame(false)).on_hover_text(tooltip)
}

/// Chip toggle - dipakai untuk B/I/U, rata teks, gaya. Mengembalikan Response;
/// status dibalik otomatis.
pub fn toggle_chip(ui: &mut Ui, aktif: &mut bool, label: &str) -> Response {
    ui.toggle_value(aktif, label)
}

/// Label yang bisa dipilih - dipakai untuk tab pita dan daftar gaya.
pub fn pilih_label(ui: &mut Ui, dipilih: bool, label: &str) -> Response {
    ui.selectable_label(dipilih, label)
}
