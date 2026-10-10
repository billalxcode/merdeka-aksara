//! Presentation: pita (ribbon) per tab.

use crate::application::{AksaraState, PesanKind};
use crate::domain::gaya::{FONT_OPSI, UKURAN_OPSI};
use crate::domain::{RataTeks, Rgb};
use merdeka_ui::{
    components::{self, Pita, alat_toggle, grup, tool_button, tool_toggle, zoom_slider},
    icons::{Icon, Outline},
};

/// (label, ikon, tooltip) tiap perataan.
fn rata_meta(r: RataTeks) -> (&'static str, Outline, &'static str) {
    match r {
        RataTeks::Kiri => ("Kiri", Outline::AlignLeft, "Rata kiri"),
        RataTeks::Tengah => ("Tengah", Outline::AlignCenter, "Rata tengah"),
        RataTeks::Kanan => ("Kanan", Outline::AlignRight, "Rata kanan"),
        RataTeks::Rata => ("Rata", Outline::AlignJustified, "Justify"),
    }
}

/// Konversi warna domain ke egui (satu-satunya tempat konversi ini).
pub fn ke_egui(w: Rgb) -> egui::Color32 {
    egui::Color32::from_rgb(w.r, w.g, w.b)
}

fn dari_egui(c: egui::Color32) -> Rgb {
    let [r, g, b, _] = c.to_array();
    Rgb::baru(r, g, b)
}

/// Gambar isi pita untuk tab aktif.
pub fn tampilkan(state: &mut AksaraState, ui: &mut egui::Ui, tab: Pita) {
    match tab {
        Pita::Berkas => {
            grup(ui, "Dokumen", |ui| {
                if tool_button(
                    ui,
                    Icon::Outline(Outline::FilePlus),
                    "Baru",
                    "Dokumen kosong",
                )
                .clicked()
                {
                    state.konfirm_baru = true;
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::FolderOpen),
                    "Buka",
                    "Buka dokumen-aksara.txt",
                )
                .clicked()
                {
                    state.buka();
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::DeviceFloppy),
                    "Simpan",
                    "Simpan ke dokumen-aksara.txt",
                )
                .clicked()
                {
                    state.simpan();
                }
            });
            ui.separator();
            grup(ui, "Ekspor", |ui| {
                if tool_button(
                    ui,
                    Icon::Outline(Outline::FileTypePdf),
                    "PDF",
                    "Ekspor PDF (segera hadir)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Ekspor PDF: segera hadir di P1 IO.");
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Book),
                    "EPUB",
                    "Ekspor EPUB (segera hadir)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Ekspor EPUB: segera hadir di P4.");
                }
            });
        }
        Pita::Beranda => {
            grup(ui, "Font", |ui| {
                let gaya = &mut state.dokumen.gaya;
                components::font_picker(ui, &mut gaya.font, FONT_OPSI);
                components::size_picker(ui, &mut gaya.ukuran, UKURAN_OPSI);
                alat_toggle(ui, &mut gaya.tebal, "B", "Tebal", "Tebal (Ctrl+B)");
                alat_toggle(ui, &mut gaya.miring, "I", "Miring", "Miring (Ctrl+I)");
                alat_toggle(
                    ui,
                    &mut gaya.garis_bawah,
                    "U",
                    "Garis",
                    "Garis bawah (Ctrl+U)",
                );
                let mut warna_egui = ke_egui(gaya.warna);
                components::color_picker(ui, "Warna", &mut warna_egui);
                gaya.warna = dari_egui(warna_egui);
            });
            ui.separator();
            grup(ui, "Paragraf", |ui| {
                for r in [
                    RataTeks::Kiri,
                    RataTeks::Tengah,
                    RataTeks::Kanan,
                    RataTeks::Rata,
                ] {
                    let (label, ikon, tips) = rata_meta(r);
                    let mut aktif = state.dokumen.gaya.rata == r;
                    tool_toggle(ui, &mut aktif, Icon::Outline(ikon), label, tips);
                    if aktif {
                        state.dokumen.gaya.rata = r;
                    }
                }
                if tool_button(ui, Icon::Outline(Outline::List), "Poin", "Daftar poin").clicked() {
                    state.dokumen.sisip_poin();
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::ListNumbers),
                    "Nomor",
                    "Daftar bernomor",
                )
                .clicked()
                {
                    state.dokumen.sisip_nomor();
                }
            });
            ui.separator();
            grup(ui, "Gaya", |ui| {
                for g in ["Normal", "Judul 1", "Judul 2", "Judul"] {
                    let mut aktif = state.dokumen.gaya.gaya == g;
                    if components::toggle_chip(ui, &mut aktif, g).clicked() && aktif {
                        state.dokumen.gaya.gaya = g.to_owned();
                    }
                }
            });
        }
        Pita::Sisip => {
            grup(ui, "Tabel & Media", |ui| {
                if tool_button(ui, Icon::Outline(Outline::Table), "Tabel", "Sisip tabel").clicked()
                {
                    state.tabel_dialog = true;
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Photo),
                    "Gambar",
                    "Sisip gambar (segera hadir)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Sisip gambar: segera hadir di P1.");
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Heading),
                    "Header",
                    "Header (segera hadir)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Header/footer: segera hadir di P1.");
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Hash),
                    "Nomor",
                    "Nomor halaman (segera hadir)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Nomor halaman: segera hadir di P1.");
                }
            });
            ui.separator();
            grup(ui, "Tautan", |ui| {
                if tool_button(ui, Icon::Outline(Outline::Link), "Link", "Sisip hyperlink")
                    .clicked()
                {
                    state.dokumen.sisip_hyperlink();
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Bookmark),
                    "Bookmark",
                    "Bookmark (segera hadir)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Bookmark: segera hadir di P2.");
                }
            });
        }
        Pita::TataLetak => {
            grup(ui, "Halaman", |ui| {
                components::option_picker(
                    ui,
                    "Margin",
                    &mut state.margin,
                    &["Normal", "Sempit", "Lebar"],
                );
                components::option_picker(
                    ui,
                    "Orientasi",
                    &mut state.orientasi,
                    &["Potret", "Lanskap"],
                );
                components::option_picker(ui, "Kolom", &mut state.kolom, &["1", "2", "3"]);
            });
            ui.separator();
            grup(ui, "Tampil", |ui| {
                components::checkbox_row(ui, &mut state.tampil_ruler, "Penggaris");
                components::checkbox_row(ui, &mut state.tampil_sidebar, "Sidebar");
            });
        }
        Pita::Referensi => {
            grup(ui, "Daftar & Catatan", |ui| {
                if tool_button(
                    ui,
                    Icon::Outline(Outline::List),
                    "Daftar Isi",
                    "Daftar isi otomatis (P2)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Daftar isi otomatis: segera hadir di P2.");
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Note),
                    "Catatan",
                    "Sisip catatan kaki",
                )
                .clicked()
                {
                    state.dokumen.sisip_catatan();
                }
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Tag),
                    "Keterangan",
                    "Keterangan gambar/tabel (P2)",
                )
                .clicked()
                {
                    state.info(
                        PesanKind::Info,
                        "Keterangan + cross-ref: segera hadir di P2.",
                    );
                }
            });
        }
        Pita::Tinjau => {
            grup(ui, "Review", |ui| {
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Message),
                    "Komentar",
                    "Tambah komentar",
                )
                .clicked()
                {
                    state.tambah_komentar();
                }
                components::checkbox_row(ui, &mut state.lacak, "Lacak perubahan");
                if tool_button(
                    ui,
                    Icon::Outline(Outline::Abc),
                    "Ejaan",
                    "Periksa ejaan (P3)",
                )
                .clicked()
                {
                    state.info(PesanKind::Info, "Ejaan hunspell: segera hadir di P3.");
                }
            });
        }
        Pita::Tampil => {
            grup(ui, "Zoom", |ui| {
                zoom_slider(ui, &mut state.zoom);
            });
            ui.separator();
            grup(ui, "Panel", |ui| {
                components::checkbox_row(ui, &mut state.tampil_ruler, "Penggaris");
                components::checkbox_row(ui, &mut state.tampil_sidebar, "Sidebar");
            });
            ui.separator();
            grup(ui, "Tema", |ui| {
                ui.label("Terang (dikunci)");
            });
        }
    }
}
