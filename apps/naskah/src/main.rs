use merdeka_ui::{
    MerdekaConfig, MerdekaPage,
    components::{self, Pita},
};

#[derive(Default)]
struct NaskahPage;

impl MerdekaPage for NaskahPage {
    fn ribbon(&mut self, ui: &mut egui::Ui, _tab: Pita) {
        components::grup(ui, "Segera", |ui| {
            ui.label("Modul Naskah menyusul.");
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let _ = components::empty_state(
                ui,
                "*",
                "Merdeka Naskah",
                "Ruang naskah bebas. Modul menyusul.",
                "OK",
            );
        });
    }

    fn status_kiri(&self) -> String {
        "Naskah".to_owned()
    }

    fn status_kanan(&self) -> String {
        "Terang".to_owned()
    }
}

fn main() -> eframe::Result {
    merdeka_ui::run_page(
        MerdekaConfig::new("Merdeka Naskah", env!("CARGO_PKG_VERSION")),
        |_cc| NaskahPage,
    )
}
