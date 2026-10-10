//! Shell aplikasi Merdeka: bar judul + pita + kanvas + status (terang).
//!
//! Alur lukis tiap frame (root `Ui` dari eframe 0.36):
//! 1. `Panel::top("judul")` - logo, nama, cari
//! 2. `Panel::top("pita-tab")` - Berkas...Tampil
//! 3. `Panel::top("pita-isi")` - grup perintah tab aktif (dari `MerdekaPage::ribbon`)
//! 4. `Panel::bottom("status")` - info dokumen
//! 5. Sisa ruang - `MerdekaPage::ui` (kanvas dokumen + sidebar opsional)

use crate::components::{self, Pita};
use crate::theme::MerdekaTheme;

/// Konfigurasi tiap aplikasi Merdeka.
#[derive(Debug, Clone)]
pub struct MerdekaConfig {
    pub app_name: String,
    pub version: String,
    pub initial_size: [f32; 2],
}

impl Default for MerdekaConfig {
    fn default() -> Self {
        Self {
            app_name: "Merdeka".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            initial_size: [1100.0, 700.0],
        }
    }
}

impl MerdekaConfig {
    pub fn new(app_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
            version: version.into(),
            initial_size: [1100.0, 700.0],
        }
    }
}

/// Satu layar/halaman aplikasi.
///
/// Tiap aplikasi Merdeka (Aksara, Layar, ...) implement trait ini;
/// `MerdekaApp` mengurus chrome (judul, pita, status) + tema terang.
pub trait MerdekaPage {
    /// Grup perintah untuk tab pita aktif. Pakai `components::grup` + `alat_*`.
    fn ribbon(&mut self, ui: &mut egui::Ui, tab: Pita);
    /// Kanvas utama (dokumen + sidebar). Dipanggil dengan sisa ruang layar.
    fn ui(&mut self, ui: &mut egui::Ui);
    fn status_kiri(&self) -> String {
        String::new()
    }
    fn status_kanan(&self) -> String {
        String::new()
    }
}

/// Wrapper eframe yang memberi chrome Merdeka di sekeliling `P`.
pub struct MerdekaApp<P: MerdekaPage> {
    config: MerdekaConfig,
    theme: MerdekaTheme,
    tab: Pita,
    cari: String,
    page: P,
}

impl<P: MerdekaPage> MerdekaApp<P> {
    pub fn new(config: MerdekaConfig, page: P) -> Self {
        Self {
            config,
            theme: MerdekaTheme::default(),
            tab: Pita::Beranda,
            cari: String::new(),
            page,
        }
    }
}

impl<P: MerdekaPage> eframe::App for MerdekaApp<P> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("merdeka_judul").show(ui, |ui| {
            components::title_bar(
                ui,
                &self.config.app_name,
                &self.config.version,
                &mut self.cari,
            );
        });

        egui::Panel::top("merdeka_pita_tab").show(ui, |ui| {
            if let Some(t) = components::tab_bar(ui, self.tab) {
                self.tab = t;
            }
        });

        egui::Panel::top("merdeka_pita_isi").show(ui, |ui| {
            egui::ScrollArea::horizontal()
                .auto_shrink(false)
                .show(ui, |ui| {
                    self.page.ribbon(ui, self.tab);
                });
        });

        let kiri = self.page.status_kiri();
        let kanan = self.page.status_kanan();
        if !kiri.is_empty() || !kanan.is_empty() {
            components::status_bar(ui, &kiri, &kanan);
        }

        self.page.ui(ui);
    }
}

/// Jalankan halaman sebagai aplikasi desktop native (tema terang dikunci).
pub fn run_page<P, F>(config: MerdekaConfig, creator: F) -> eframe::Result
where
    P: MerdekaPage + 'static,
    F: FnOnce(&eframe::CreationContext<'_>) -> P + 'static,
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(config.initial_size),
        ..Default::default()
    };
    let app_name = config.app_name.clone();
    eframe::run_native(
        &app_name,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let page = creator(cc);
            let app = MerdekaApp::new(config, page);
            app.theme.apply(&cc.egui_ctx);
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
}
