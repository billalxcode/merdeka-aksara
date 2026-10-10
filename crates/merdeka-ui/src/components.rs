//! Komponen bersama: bar atas, bar status, kartu, empty-state.

use crate::theme::MerdekaTheme;
use egui::{Align, CentralPanel, Layout, Panel};

/// Bar atas standar semua aplikasi Merdeka.
///
/// egui 0.36: panel di-show dari root `Ui` (bukan `Context`).
pub fn top_bar(ui: &mut egui::Ui, app_name: &str, version: &str, theme: &mut MerdekaTheme) {
    let icon = if theme.mode.is_dark() { "☀" } else { "🌙" };
    let clicked = Panel::top("merdeka_top")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(app_name);
                ui.label(format!("v{version}"));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.button(icon).clicked()
                })
                .inner
            })
            .inner
        })
        .inner;

    if clicked {
        theme.toggle();
        let ctx = ui.ctx().clone();
        theme.apply(&ctx);
    }
}

/// Bar status bawah standar.
pub fn status_bar(ui: &mut egui::Ui, text: &str) {
    Panel::bottom("merdeka_status").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(text);
        });
    });
}

/// Panel tengah standar (bungkus konten halaman).
pub fn central(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    CentralPanel::default().show(ui, add_contents);
}

/// Kartu grup untuk mengelompokkan konten.
pub fn card(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(title);
        ui.separator();
        add_contents(ui);
    });
}

/// Tampilan kosong standar (belum ada dokumen, dsb).
pub fn empty_state(ui: &mut egui::Ui, title: &str, subtitle: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.heading(title);
        ui.label(subtitle);
    });
}
