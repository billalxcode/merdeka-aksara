//! Tema Merdeka: merah-putih, terang/gelap.

/// Merah Merdeka — aksen utama (seleksi, hyperlink, tombol primer).
pub const MERAH_MERDEKA: egui::Color32 = egui::Color32::from_rgb(200, 16, 46);
/// Merah gelap untuk hover / penekanan.
pub const MERAH_TUA: egui::Color32 = egui::Color32::from_rgb(150, 10, 34);

/// Mode terang / gelap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    Terang,
    #[default]
    Gelap,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Terang => Self::Gelap,
            Self::Gelap => Self::Terang,
        }
    }

    pub fn is_dark(self) -> bool {
        matches!(self, Self::Gelap)
    }
}

/// Tema yang disimpan di state aplikasi.
#[derive(Debug, Clone, Copy, Default)]
pub struct MerdekaTheme {
    pub mode: ThemeMode,
}

impl MerdekaTheme {
    pub fn new(mode: ThemeMode) -> Self {
        Self { mode }
    }

    pub fn toggle(&mut self) {
        self.mode = self.mode.toggle();
    }

    /// Terapkan ke egui Context. Panggil saat init + setiap toggle.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self.mode {
            ThemeMode::Terang => egui::Visuals::light(),
            ThemeMode::Gelap => egui::Visuals::dark(),
        };
        visuals.selection.bg_fill = MERAH_MERDEKA;
        visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
        visuals.hyperlink_color = MERAH_MERDEKA;
        ctx.set_visuals(visuals);
    }
}
