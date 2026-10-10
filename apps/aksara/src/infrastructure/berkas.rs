//! Infrastructure: simpan/buka dokumen teks polos.
//!
//! Format kaya (DOCX/ODT/PDF/EPUB) menyusul di P1/P4.

use crate::domain::Dokumen;
use std::io;
use std::path::Path;

/// Simpan isi dokumen sebagai teks polos.
pub fn simpan_teks(path: &Path, dok: &Dokumen) -> io::Result<()> {
    std::fs::write(path, &dok.teks)
}

/// Buka teks polos menjadi dokumen baru (gaya default).
pub fn buka_teks(path: &Path) -> io::Result<Dokumen> {
    Ok(Dokumen {
        teks: std::fs::read_to_string(path)?,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn berkas_sementara(nama: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("aksara-test-{}-{}", std::process::id(), nama))
    }

    #[test]
    fn roundtrip_simpan_buka() {
        let path = berkas_sementara("roundtrip.txt");
        let mut dok = Dokumen::baru();
        dok.teks.push_str("Merdeka menulis bebas");
        simpan_teks(&path, &dok).unwrap();
        let baca = buka_teks(&path).unwrap();
        assert_eq!(baca.teks, dok.teks);
        assert_eq!(baca.kata(), 3);
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn buka_gagal_untuk_hilang() {
        let path = berkas_sementara("tidak-ada.txt");
        assert!(buka_teks(&path).is_err());
    }
}
