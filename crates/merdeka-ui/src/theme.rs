//! Tema Merdeka - terang, dikunci.
//!
//! Satu tampilan terang untuk semua aplikasi agar konsisten. Warna primer
//! boleh beda per aplikasi lewat preset (Aksara biru ocean, default merah).
//! Tidak ada mode gelap / toggle.

/// Latar aplikasi.
pub const LATAR: egui::Color32 = egui::Color32::from_rgb(247, 246, 243);
/// Kertas dokumen.
pub const KERTAS: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);
/// Garis pembatas / bingkai.
pub const GARIS: egui::Color32 = egui::Color32::from_rgb(229, 225, 218);
/// Teks utama.
pub const TEKS: egui::Color32 = egui::Color32::from_rgb(28, 27, 26);
/// Teks sekunder.
pub const TEKS_REDAH: egui::Color32 = egui::Color32::from_rgb(107, 103, 95);
/// Sukses / info / peringatan / galat.
pub const SUKSES: egui::Color32 = egui::Color32::from_rgb(31, 122, 61);
pub const INFO: egui::Color32 = egui::Color32::from_rgb(29, 90, 160);
pub const PERINGATAN: egui::Color32 = egui::Color32::from_rgb(183, 121, 31);
pub const GALAT: egui::Color32 = egui::Color32::from_rgb(180, 35, 24);

/// Radius sudut standar.
pub const RADIUS: f32 = 8.0;

/// Tema terang. Pilih preset per aplikasi, lalu `apply` sekali saat init.
#[derive(Debug, Clone, Copy)]
pub struct MerdekaTheme {
    /// Warna primer (seleksi, hyperlink, tombol primer, tab aktif, logo).
    pub primer: egui::Color32,
    /// Primer gelap (tekan / penekanan).
    pub primer_tua: egui::Color32,
    /// Tint primer (latar hover / aktif).
    pub primer_muda: egui::Color32,
    /// Lebar maksimum kolom dokumen (pt).
    pub lebar_dokumen: f32,
}

impl MerdekaTheme {
    /// Merah Merdeka (default semua aplikasi).
    pub fn merdeka() -> Self {
        Self {
            primer: egui::Color32::from_rgb(200, 16, 46),
            primer_tua: egui::Color32::from_rgb(150, 10, 34),
            primer_muda: egui::Color32::from_rgb(251, 231, 235),
            lebar_dokumen: 720.0,
        }
    }

    /// Biru ocean Aksara (#2373F4).
    pub fn aksara() -> Self {
        Self {
            primer: egui::Color32::from_rgb(0x23, 0x73, 0xF4),
            primer_tua: egui::Color32::from_rgb(0x1A, 0x59, 0xBF),
            primer_muda: egui::Color32::from_rgb(0xE5, 0xEE, 0xFE),
            lebar_dokumen: 720.0,
        }
    }

    /// Terapkan tema ke egui Context. Panggil sekali saat init.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut v = egui::Visuals::light();
        v.selection.bg_fill = self.primer;
        v.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
        v.hyperlink_color = self.primer;
        v.window_fill = LATAR;
        v.panel_fill = LATAR;
        v.faint_bg_color = self.primer_muda;
        v.extreme_bg_color = KERTAS;
        v.window_corner_radius = egui::CornerRadius::same(10);
        v.widgets.hovered.bg_fill = self.primer_muda;
        v.widgets.active.bg_fill = self.primer_muda;
        v.widgets.open.bg_fill = self.primer_muda;
        ctx.set_visuals(v);
    }
}

impl Default for MerdekaTheme {
    fn default() -> Self {
        Self::merdeka()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primer_aksara_biru_ocean() {
        assert_eq!(
            MerdekaTheme::aksara().primer,
            egui::Color32::from_rgb(0x23, 0x73, 0xF4)
        );
    }

    #[test]
    fn default_tetap_merah_merdeka() {
        assert_eq!(
            MerdekaTheme::default().primer,
            egui::Color32::from_rgb(200, 16, 46)
        );
    }
}
