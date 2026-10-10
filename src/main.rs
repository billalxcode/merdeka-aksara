use merdeka_ui::{MerdekaConfig, MerdekaPage, components, run_page};

#[derive(Default)]
struct AksaraPage {
    teks: String,
    kata: usize,
}

impl MerdekaPage for AksaraPage {
    fn title(&self) -> &str {
        "Aksara"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Aksara — tulis bebas, merdeka.");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{} kata", self.kata));
            });
        });
        ui.separator();

        components::card(ui, "Dokumen", |ui| {
            let out = egui::TextEdit::multiline(&mut self.teks)
                .desired_rows(16)
                .hint_text("Tulis di sini… bebas pakai, bebas salin, bebas edit.")
                .show(ui);
            if out.response.changed() {
                self.kata = self.teks.split_whitespace().count();
            }
        });

        ui.add_space(8.0);
        if self.teks.trim().is_empty() {
            components::empty_state(
                ui,
                "Belum ada tulisan",
                "Mulai mengetik di atas. Semua milikmu, tanpa batasan.",
            );
        }
    }

    fn status(&self) -> String {
        format!("Aksara • {} kata • {} karakter", self.kata, self.teks.len())
    }
}

fn main() -> eframe::Result {
    let config = MerdekaConfig::new("Merdeka Aksara", env!("CARGO_PKG_VERSION"));
    run_page(config, |_cc| AksaraPage::default())
}
