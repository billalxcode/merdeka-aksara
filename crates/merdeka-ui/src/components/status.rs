//! Bar status - kiri info dokumen, kanan kata + zoom + tema.

use egui::Ui;

/// Bar status bawah: teks kiri (halaman, bahasa) + kanan (kata, zoom, tema terang).
pub fn status_bar(ui: &mut Ui, kiri: &str, kanan: &str) {
    egui::Panel::bottom("merdeka_status").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(kiri);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(kanan);
            });
        });
    });
}

/// Slider zoom 50%-200% untuk bar status / tab Tampil.
pub fn zoom_slider(ui: &mut Ui, zoom: &mut f32) {
    ui.horizontal(|ui| {
        ui.label("Zoom");
        ui.add(egui::Slider::new(zoom, 0.5..=2.0).suffix("x").show_value(true));
    });
}
