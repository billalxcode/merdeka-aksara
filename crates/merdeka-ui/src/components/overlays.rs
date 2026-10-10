//! Overlay - dialog modal + konfirmasi.
//!
//! Semua dialog memakai `egui::Window` modal di tengah layar.

use egui::{Context, Ui};

/// Hasil dialog konfirmasi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Konfirmasi {
    BelumDipilih,
    Ya,
    Tidak,
}

/// Dialog modal generik. `terbuka` dikontrol pemanggil.
/// Isi + tombol aksi digambar oleh closure; klik luar tidak menutup.
pub fn modal(
    ctx: &Context,
    terbuka: &mut bool,
    judul: &str,
    isi: impl FnOnce(&mut Ui),
    aksi: impl FnOnce(&mut Ui),
) {
    egui::Window::new(judul)
        .open(terbuka)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            isi(ui);
            ui.separator();
            ui.horizontal(aksi);
        });
}

/// Dialog konfirmasi Ya/Tidak. Mengembalikan pilihan; menutup otomatis.
pub fn confirm(ctx: &Context, terbuka: &mut bool, judul: &str, pesan: &str) -> Konfirmasi {
    let mut hasil = Konfirmasi::BelumDipilih;
    let mut tutup = false;
    egui::Window::new(judul)
        .open(terbuka)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(pesan);
            ui.separator();
            ui.horizontal(|ui| {
                if crate::components::buttons::primary_button(ui, "Ya").clicked() {
                    hasil = Konfirmasi::Ya;
                    tutup = true;
                }
                if crate::components::buttons::secondary_button(ui, "Tidak").clicked() {
                    hasil = Konfirmasi::Tidak;
                    tutup = true;
                }
            });
        });
    if tutup {
        *terbuka = false;
    }
    hasil
}
