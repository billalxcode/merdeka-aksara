//! Domain: gaya teks dan paragraf.

/// Perataan paragraf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RataTeks {
    #[default]
    Kiri,
    Tengah,
    Kanan,
    Rata,
}

/// Warna RGB milik domain (bebas egui).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const HITAM: Self = Self { r: 0, g: 0, b: 0 };

    pub fn baru(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Gaya penulisan aktif untuk teks baru.
#[derive(Debug, Clone, PartialEq)]
pub struct GayaDokumen {
    pub font: String,
    pub ukuran: f32,
    pub tebal: bool,
    pub miring: bool,
    pub garis_bawah: bool,
    pub warna: Rgb,
    pub gaya: String,
    pub rata: RataTeks,
}

impl Default for GayaDokumen {
    fn default() -> Self {
        Self {
            font: "Inter".to_owned(),
            ukuran: 12.0,
            tebal: false,
            miring: false,
            garis_bawah: false,
            warna: Rgb::HITAM,
            gaya: "Normal".to_owned(),
            rata: RataTeks::Kiri,
        }
    }
}

/// Pilihan font bawaan.
pub const FONT_OPSI: &[&str] = &["Inter", "Liberation Serif", "Noto Sans", "Ubuntu Mono"];
/// Pilihan ukuran font (pt).
pub const UKURAN_OPSI: &[f32] = &[10.0, 11.0, 12.0, 14.0, 16.0, 18.0, 24.0];
