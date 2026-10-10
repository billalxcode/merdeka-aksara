//! Presentation: halaman Aksara sebagai `MerdekaPage`.
//!
//! Adaptor tipis: teruskan ke ribbon + editor dengan state aplikasi.

use super::{editor, ribbon};
use crate::application::AksaraState;
use merdeka_ui::{MerdekaPage, components::Pita};

pub struct AksaraPage {
    state: AksaraState,
}

impl AksaraPage {
    pub fn baru() -> Self {
        Self {
            state: AksaraState::baru(),
        }
    }
}

impl Default for AksaraPage {
    fn default() -> Self {
        Self::baru()
    }
}

impl MerdekaPage for AksaraPage {
    fn ribbon(&mut self, ui: &mut egui::Ui, tab: Pita) {
        ribbon::tampilkan(&mut self.state, ui, tab);
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        editor::tampilkan(&mut self.state, ui);
    }

    fn status_kiri(&self) -> String {
        let s = &self.state;
        format!(
            "Halaman 1 | {} | {} | {} | {} kolom",
            s.bahasa, s.margin, s.orientasi, s.kolom,
        )
    }

    fn status_kanan(&self) -> String {
        let s = &self.state;
        format!(
            "{} kata | {} karakter | {:.0}% | Terang{}",
            s.dokumen.kata(),
            s.dokumen.karakter(),
            s.zoom_persen(),
            if s.lacak { " | Lacak" } else { "" },
        )
    }
}
