//! Presentation: kanvas editor (dialog, sidebar, kertas, banner).

use super::ribbon::ke_egui;
use crate::application::{AksaraState, PesanKind};
use merdeka_ui::{
    MerdekaTheme,
    components::{
        self, BannerKind, Konfirmasi, document_scroll, empty_state, ghost_button, modal, notice,
        page_paper, primary_button, ruler_stub, secondary_button, sidebar_section,
    },
};

/// Petakan jenis pesan aplikasi ke banner egui.
fn banner_dari(jenis: PesanKind) -> BannerKind {
    match jenis {
        PesanKind::Info => BannerKind::Info,
        PesanKind::Sukses => BannerKind::Sukses,
        PesanKind::Galat => BannerKind::Galat,
    }
}

/// Gambar seluruh area editor untuk state yang diberikan.
pub fn tampilkan(state: &mut AksaraState, ui: &mut egui::Ui) {
    // Konfirmasi dokumen baru (pakai dialog reusable).
    if state.konfirm_baru
        && components::confirm(
            ui.ctx(),
            &mut state.konfirm_baru,
            "Dokumen Baru",
            "Buang isi dokumen saat ini dan mulai kosong?",
        ) == Konfirmasi::Ya
    {
        state.jalankan_baru();
    }

    // Dialog sisip tabel (modal tengah).
    if state.tabel_dialog {
        let mut baris = state.tabel_baris;
        let mut kolom = state.tabel_kolom;
        let mut sisip = false;
        let mut batal = false;
        modal(
            ui.ctx(),
            &mut state.tabel_dialog,
            "Sisip Tabel",
            |ui| {
                ui.horizontal(|ui| {
                    ui.label("Baris");
                    ui.add(egui::DragValue::new(&mut baris).range(1..=10));
                    ui.label("Kolom");
                    ui.add(egui::DragValue::new(&mut kolom).range(1..=8));
                });
            },
            |ui| {
                if primary_button(ui, "Sisipkan").clicked() {
                    sisip = true;
                }
                if secondary_button(ui, "Batal").clicked() {
                    batal = true;
                }
            },
        );
        state.tabel_baris = baris;
        state.tabel_kolom = kolom;
        if batal {
            state.tabel_dialog = false;
        }
        if sisip {
            state.sisip_tabel();
        }
    }

    // Sidebar navigasi (opsional).
    if state.tampil_sidebar {
        egui::Panel::left("aksara_navigasi")
            .default_size(200.0)
            .show(ui, |ui| {
                sidebar_section(ui, "Navigasi", true, |ui| {
                    ui.label("Halaman 1");
                    ui.label("Cari di dokumen...");
                });
                let gaya = &state.dokumen.gaya;
                sidebar_section(ui, "Gaya", true, |ui| {
                    ui.label(format!("Aktif: {}", gaya.gaya));
                    ui.label(format!("Font: {} {:.0} pt", gaya.font, gaya.ukuran));
                });
            });
    }

    // Kanvas dokumen.
    egui::CentralPanel::default().show(ui, |ui| {
        if let Some((jenis, teks)) = state.pesan.clone() {
            ui.horizontal(|ui| {
                notice(ui, banner_dari(jenis), &teks);
                if ghost_button(ui, "Tutup").clicked() {
                    state.tutup_pesan();
                }
            });
            ui.add_space(4.0);
        }

        let lebar = MerdekaTheme::default().lebar_dokumen * state.zoom.clamp(0.5, 2.0);
        document_scroll(ui, lebar, |ui| {
            if state.tampil_ruler {
                ruler_stub(ui);
                ui.add_space(4.0);
            }
            page_paper(ui, |ui| {
                let gaya = &state.dokumen.gaya;
                let mut contoh = egui::RichText::new(format!(
                    "{} - {} {:.0} pt",
                    gaya.gaya, gaya.font, gaya.ukuran
                ))
                .size(14.0 * state.zoom);
                if gaya.tebal {
                    contoh = contoh.strong();
                }
                if gaya.miring {
                    contoh = contoh.italics();
                }
                if gaya.garis_bawah {
                    contoh = contoh.underline();
                }
                ui.label(contoh.color(ke_egui(gaya.warna)));
                ui.separator();

                let keluar = egui::TextEdit::multiline(&mut state.dokumen.teks)
                    .desired_rows(22)
                    .desired_width(f32::INFINITY)
                    .hint_text("Tulis di sini... bebas pakai, bebas salin, bebas edit.")
                    .frame(egui::Frame::NONE)
                    .show(ui);

                if state.dokumen.kosong() {
                    ui.add_space(8.0);
                    if empty_state(
                        ui,
                        "*",
                        "Belum ada tulisan",
                        "Mulai mengetik di atas. Semua milikmu, tanpa batasan.",
                        "Fokus Menulis",
                    ) {
                        keluar.response.request_focus();
                    }
                }
            });
            ui.add_space(16.0);
        });
    });
}
