//! Application: state + perintah kasus pakai.

use crate::domain::Dokumen;
use crate::infrastructure::berkas;
use std::path::Path;

/// Nama berkas default untuk simpan/buka cepat (P0).
const BERKAS_DEFAULT: &str = "dokumen-aksara.txt";

/// Jenis pesan banner (dipetakan ke BannerKind egui di presentation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PesanKind {
    Info,
    Sukses,
    Galat,
}

/// Seluruh state aplikasi Aksara.
pub struct AksaraState {
    pub dokumen: Dokumen,
    pub zoom: f32,
    pub tampil_ruler: bool,
    pub tampil_sidebar: bool,
    pub lacak: bool,
    pub komentar: u32,
    pub margin: String,
    pub orientasi: String,
    pub kolom: String,
    pub bahasa: String,
    pub pesan: Option<(PesanKind, String)>,
    pub konfirm_baru: bool,
    pub tabel_dialog: bool,
    pub tabel_baris: u32,
    pub tabel_kolom: u32,
}

impl AksaraState {
    pub fn baru() -> Self {
        Self {
            dokumen: Dokumen::baru(),
            zoom: 1.0,
            tampil_ruler: true,
            tampil_sidebar: true,
            lacak: false,
            komentar: 0,
            margin: "Normal".to_owned(),
            orientasi: "Potret".to_owned(),
            kolom: "1".to_owned(),
            bahasa: "Indonesia".to_owned(),
            pesan: None,
            konfirm_baru: false,
            tabel_dialog: false,
            tabel_baris: 3,
            tabel_kolom: 3,
        }
    }

    pub fn info(&mut self, jenis: PesanKind, teks: impl Into<String>) {
        self.pesan = Some((jenis, teks.into()));
    }

    pub fn tutup_pesan(&mut self) {
        self.pesan = None;
    }

    /// Kasus pakai: dokumen baru (dipanggil setelah konfirmasi YA).
    pub fn jalankan_baru(&mut self) {
        self.dokumen.bersihkan();
        self.info(PesanKind::Sukses, "Dokumen baru dibuat.");
    }

    /// Kasus pakai: tambah komentar.
    pub fn tambah_komentar(&mut self) {
        self.komentar += 1;
        self.info(
            PesanKind::Sukses,
            format!("Komentar #{} ditambahkan.", self.komentar),
        );
    }

    /// Kasus pakai: sisip tabel dari dialog.
    pub fn sisip_tabel(&mut self) {
        let b = self.tabel_baris as usize;
        let k = self.tabel_kolom as usize;
        self.dokumen.sisip_tabel(b, k);
        self.tabel_dialog = false;
        self.info(PesanKind::Sukses, format!("Tabel {b}x{k} disisipkan."));
    }

    /// Kasus pakai: simpan ke berkas default via infrastructure.
    pub fn simpan(&mut self) {
        match berkas::simpan_teks(Path::new(BERKAS_DEFAULT), &self.dokumen) {
            Ok(()) => self.info(
                PesanKind::Sukses,
                format!(
                    "Tersimpan ke {} ({} kata).",
                    BERKAS_DEFAULT,
                    self.dokumen.kata()
                ),
            ),
            Err(e) => self.info(PesanKind::Galat, format!("Gagal menyimpan: {e}")),
        }
    }

    /// Kasus pakai: buka dari berkas default via infrastructure.
    pub fn buka(&mut self) {
        match berkas::buka_teks(Path::new(BERKAS_DEFAULT)) {
            Ok(dok) => {
                let kata = dok.kata();
                self.dokumen = dok;
                self.info(
                    PesanKind::Sukses,
                    format!("Dibuka dari {BERKAS_DEFAULT} ({kata} kata)."),
                );
            }
            Err(e) => self.info(PesanKind::Galat, format!("Gagal membuka: {e}")),
        }
    }

    pub fn zoom_persen(&self) -> f32 {
        self.zoom * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dokumen_baru_membersihkan() {
        let mut s = AksaraState::baru();
        s.dokumen.teks.push_str("halo dunia");
        s.jalankan_baru();
        assert!(s.dokumen.kosong());
        assert!(matches!(s.pesan, Some((PesanKind::Sukses, _))));
        s.tutup_pesan();
        assert!(s.pesan.is_none());
    }

    #[test]
    fn semua_jenis_pesan_tersimpan() {
        let mut s = AksaraState::baru();
        for jenis in [PesanKind::Info, PesanKind::Sukses, PesanKind::Galat] {
            s.info(jenis, "uji");
            assert!(matches!(s.pesan, Some((j, _)) if j == jenis));
        }
    }

    #[test]
    fn komentar_bertambah_dan_tabel_masuk() {
        let mut s = AksaraState::baru();
        s.tambah_komentar();
        s.tambah_komentar();
        assert_eq!(s.komentar, 2);
        s.tabel_baris = 2;
        s.tabel_kolom = 2;
        s.sisip_tabel();
        assert!(!s.tabel_dialog);
        assert_eq!(s.dokumen.kata(), 10);
    }
}
