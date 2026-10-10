//! Shell aplikasi: config + trait halaman + runner eframe 0.36.

use crate::components;
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
            initial_size: [960.0, 640.0],
        }
    }
}

impl MerdekaConfig {
    pub fn new(app_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
            version: version.into(),
            initial_size: [960.0, 640.0],
        }
    }
}

/// Satu layar/halaman aplikasi.
///
/// Tiap aplikasi Merdeka (Aksara, Layar, ...) implement trait ini,
/// `MerdekaApp` yang mengurus bar atas / tengah / status + tema.
pub trait MerdekaPage {
    fn title(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui);
    fn status(&self) -> String {
        String::new()
    }
}

/// Wrapper eframe yang memberi chrome Merdeka di sekeliling `P`.
pub struct MerdekaApp<P: MerdekaPage> {
    config: MerdekaConfig,
    theme: MerdekaTheme,
    page: P,
}

impl<P: MerdekaPage> MerdekaApp<P> {
    pub fn new(config: MerdekaConfig, page: P) -> Self {
        Self {
            config,
            theme: MerdekaTheme::default(),
            page,
        }
    }
}

impl<P: MerdekaPage> eframe::App for MerdekaApp<P> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let status = self.page.status();
        components::top_bar(ui, &self.config.app_name.clone(), &self.config.version.clone(), &mut self.theme);
        if !status.is_empty() {
            components::status_bar(ui, &status);
        }
        components::central(ui, |ui| {
            self.page.ui(ui);
        });
    }
}

/// Jalankan halaman sebagai aplikasi desktop native.
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
            let page = creator(cc);
            let app = MerdekaApp::new(config, page);
            app.theme.apply(&cc.egui_ctx);
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
}
