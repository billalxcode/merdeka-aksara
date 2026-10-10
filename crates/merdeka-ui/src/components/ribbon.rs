//! Pita (ribbon) ala Word - tab + grup + tombol alat.
//!
//! Tab disamakan dengan Microsoft Word agar migrasi mudah:
//! Berkas - Beranda - Sisip - Tata Letak - Referensi - Tinjau - Tampil.

use egui::{Response, RichText, Ui};

/// Tab pita standar semua aplikasi Merdeka.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pita {
    Berkas,
    #[default]
    Beranda,
    Sisip,
    TataLetak,
    Referensi,
    Tinjau,
    Tampil,
}

impl Pita {
    pub const SEMUA: &'static [Pita] = &[
        Pita::Berkas,
        Pita::Beranda,
        Pita::Sisip,
        Pita::TataLetak,
        Pita::Referensi,
        Pita::Tinjau,
        Pita::Tampil,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Pita::Berkas => "Berkas",
            Pita::Beranda => "Beranda",
            Pita::Sisip => "Sisip",
            Pita::TataLetak => "Tata Letak",
            Pita::Referensi => "Referensi",
            Pita::Tinjau => "Tinjau",
            Pita::Tampil => "Tampil",
        }
    }
}

/// Baris tab pita. Mengembalikan tab baru bila pengguna pindah.
pub fn tab_bar(ui: &mut Ui, aktif: Pita) -> Option<Pita> {
    let mut pindah = None;
    ui.horizontal(|ui| {
        for t in Pita::SEMUA {
            if ui.selectable_label(aktif == *t, t.label()).clicked() {
                pindah = Some(*t);
            }
        }
    });
    pindah
}

/// Grup perintah dalam pita: isi + judul kecil di bawah.
pub fn grup(ui: &mut Ui, judul: &str, isi: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        ui.horizontal(isi);
        ui.label(RichText::new(judul).small().weak());
    });
}

/// Tombol aksi pita: ikon + label, satu klik.
pub fn alat_button(ui: &mut Ui, ikon: &str, label: &str, tooltip: &str) -> Response {
    ui.button(format!("{ikon} {label}")).on_hover_text(tooltip)
}

/// Tombol toggle pita: B/I/U, rata teks, dsb.
pub fn alat_toggle(ui: &mut Ui, aktif: &mut bool, ikon: &str, label: &str, tooltip: &str) -> Response {
    ui.toggle_value(aktif, format!("{ikon} {label}"))
        .on_hover_text(tooltip)
}
