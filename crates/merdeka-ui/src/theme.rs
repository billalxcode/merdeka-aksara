//! Tema Merdeka - **terang, dikunci**.
//!
//! Seluruh aplikasi Merdeka memakai satu tema terang yang sama agar konsisten
//! dan mudah dikenali. Tidak ada mode gelap / toggle.

/// Merah Merdeka - aksen utama (seleksi, hyperlink, tombol primer, tab aktif).
pub const MERAH: egui::Color32 = egui::Color32::from_rgb(200, 16, 46);
/// Merah tua - hover / tekan / teks di atas tint.
pub const MERAH_TUA: egui::Color32 = egui::Color32::from_rgb(150, 10, 34);
/// Tint merah muda - latar tombol aktif / hover.
pub const MERAH_MUDA: egui::Color32 = egui::Color32::from_rgb(251, 231, 235);
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

/// Tema terang Merdeka. Tidak ada varian lain.
#[derive(Debug, Clone, Copy)]
pub struct MerdekaTheme {
    /// Lebar maksimum kolom dokumen (pt).
    pub lebar_dokumen: f32,
}

impl Default for MerdekaTheme {
    fn default() -> Self {
        Self {
            lebar_dokumen: 720.0,
        }
    }
}

impl MerdekaTheme {
    /// Terapkan tema terang ke egui Context. Panggil sekali saat init.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut v = egui::Visuals::light();
        v.selection.bg_fill = MERAH;
        v.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
        v.hyperlink_color = MERAH;
        v.window_fill = LATAR;
        v.panel_fill = LATAR;
        v.faint_bg_color = MERAH_MUDA;
        v.extreme_bg_color = KERTAS;
        v.window_corner_radius = egui::CornerRadius::same(10);
        v.widgets.hovered.bg_fill = MERAH_MUDA;
        v.widgets.active.bg_fill = MERAH_MUDA;
        v.widgets.open.bg_fill = MERAH_MUDA;
        ctx.set_visuals(v);
    }
}
