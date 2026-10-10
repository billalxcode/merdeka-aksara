//! Domain: dokumen + operasi teks.

use super::gaya::GayaDokumen;

/// Dokumen Aksara: isi teks + gaya aktif.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dokumen {
    pub teks: String,
    pub gaya: GayaDokumen,
}

impl Dokumen {
    pub fn baru() -> Self {
        Self::default()
    }

    /// Jumlah kata (pisah spasi).
    pub fn kata(&self) -> usize {
        self.teks.split_whitespace().count()
    }

    /// Jumlah karakter (byte).
    pub fn karakter(&self) -> usize {
        self.teks.len()
    }

    pub fn kosong(&self) -> bool {
        self.teks.trim().is_empty()
    }

    pub fn bersihkan(&mut self) {
        self.teks.clear();
    }

    pub fn sisip_poin(&mut self) {
        self.teks.push_str("\n- ");
    }

    pub fn sisip_nomor(&mut self) {
        self.teks.push_str("\n1. ");
    }

    pub fn sisip_hyperlink(&mut self) {
        self.teks.push_str(" [teks](https://...)");
    }

    pub fn sisip_catatan(&mut self) {
        self.teks.push_str("[^1]");
    }

    /// Bangun teks tabel polos. Baris/kolom dijepit ke 1..=10 / 1..=8.
    pub fn tabel_teks(baris: usize, kolom: usize) -> String {
        let b = baris.clamp(1, 10);
        let k = kolom.clamp(1, 8);
        let mut t = String::from("\n");
        for _ in 0..b {
            t.push('|');
            for _ in 0..k {
                t.push_str(" sel |");
            }
            t.push('\n');
        }
        t
    }

    pub fn sisip_tabel(&mut self, baris: usize, kolom: usize) {
        self.teks.push_str(&Self::tabel_teks(baris, kolom));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hitung_kata_dan_karakter() {
        let dok = Dokumen {
            teks: "Merdeka menulis bebas".to_owned(),
            ..Default::default()
        };
        assert_eq!(dok.kata(), 3);
        assert_eq!(dok.karakter(), "Merdeka menulis bebas".len());
        assert!(!dok.kosong());
    }

    #[test]
    fn dokumen_kosong_dan_bersih() {
        let mut dok = Dokumen {
            teks: "  \n ".to_owned(),
            ..Default::default()
        };
        assert!(dok.kosong());
        dok.teks.push_str("isi");
        assert!(!dok.kosong());
        dok.bersihkan();
        assert_eq!(dok.teks, "");
        assert_eq!(dok.kata(), 0);
    }

    #[test]
    fn sisipan_teks() {
        let mut dok = Dokumen::baru();
        dok.sisip_poin();
        dok.sisip_nomor();
        dok.sisip_hyperlink();
        dok.sisip_catatan();
        assert!(dok.teks.contains("- "));
        assert!(dok.teks.contains("1. "));
        assert!(dok.teks.contains("[teks](https://...)"));
        assert!(dok.teks.contains("[^1]"));
    }

    #[test]
    fn tabel_dijepit_dan_terbentuk() {
        let t = Dokumen::tabel_teks(2, 3);
        assert_eq!(t.matches('|').count(), 2 * (3 + 1));
        let t0 = Dokumen::tabel_teks(0, 99);
        assert_eq!(t0.matches('|').count(), 9);
    }
}
