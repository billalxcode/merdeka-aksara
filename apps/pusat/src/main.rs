use merdeka_ui::{
    MerdekaConfig, MerdekaPage,
    components::{self, Pita},
};

#[derive(Default)]
struct PusatPage;

impl MerdekaPage for PusatPage {
    fn ribbon(&mut self, ui: &mut egui::Ui, _tab: Pita) {
        components::grup(ui, "Segera", |ui| {
            ui.label("Modul Pusat menyusul.");
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let _ = components::empty_state(
                ui,
                "*",
                "Merdeka Pusat",
                "Pusat kendali bebas. Modul menyusul.",
                "OK",
            );
        });
    }

    fn status_kiri(&self) -> String {
        "Pusat".to_owned()
    }

    fn status_kanan(&self) -> String {
        "Terang".to_owned()
    }
}

fn main() -> eframe::Result {
    merdeka_ui::run_page(
        MerdekaConfig::new("Merdeka Pusat", env!("CARGO_PKG_VERSION")),
        |_cc| PusatPage,
    )
}
