//! Input - field, angka, pilihan, dropdown, warna.

use egui::{ComboBox, Response, Ui};

/// Field teks satu baris berlabel.
pub fn text_field(ui: &mut Ui, label: &str, nilai: &mut String, hint: &str) -> Response {
    ui.vertical(|ui| {
        ui.label(label);
        ui.text_edit_singleline(nilai).on_hover_text(hint)
    })
    .inner
}

/// Field cari dengan hint standar command palette.
pub fn search_field(ui: &mut Ui, nilai: &mut String) -> Response {
    ui.add(
        egui::TextEdit::singleline(nilai)
            .hint_text("Cari perintah... (Ctrl+K)")
            .desired_width(220.0),
    )
}

/// Baris angka geser (drag value).
pub fn number_row(ui: &mut Ui, label: &str, nilai: &mut f32, min: f32, max: f32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::DragValue::new(nilai).range(min..=max).speed(1.0));
    });
}

/// Baris slider.
pub fn slider_row(ui: &mut Ui, label: &str, nilai: &mut f32, min: f32, max: f32, suffix: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(egui::Slider::new(nilai, min..=max).suffix(suffix));
    });
}

/// Baris checkbox.
pub fn checkbox_row(ui: &mut Ui, nilai: &mut bool, label: &str) -> Response {
    ui.checkbox(nilai, label)
}

/// Baris radio generik.
pub fn radio_row<T: PartialEq>(ui: &mut Ui, saat_ini: &mut T, nilai: T, label: &str) -> Response {
    ui.radio_value(saat_ini, nilai, label)
}

/// Dropdown font konkret (hindari generik agar API stabil).
pub fn font_picker(ui: &mut Ui, saat_ini: &mut String, opsi: &[&str]) {
    ComboBox::from_label("Font")
        .selected_text(saat_ini.as_str())
        .show_ui(ui, |ui| {
            for o in opsi {
                ui.selectable_value(saat_ini, (*o).to_owned(), *o);
            }
        });
}

/// Dropdown ukuran font konkret.
pub fn size_picker(ui: &mut Ui, saat_ini: &mut f32, opsi: &[f32]) {
    let teks = format!("{saat_ini:.0} pt");
    ComboBox::from_label("Ukuran")
        .selected_text(teks)
        .show_ui(ui, |ui| {
            for o in opsi {
                ui.selectable_value(saat_ini, *o, format!("{o:.0} pt"));
            }
        });
}

/// Dropdown teks generik untuk opsi statis (margin, orientasi, kolom, bahasa).
pub fn option_picker(ui: &mut Ui, label: &str, saat_ini: &mut String, opsi: &[&str]) {
    ComboBox::from_label(label)
        .selected_text(saat_ini.as_str())
        .show_ui(ui, |ui| {
            for o in opsi {
                ui.selectable_value(saat_ini, (*o).to_owned(), *o);
            }
        });
}

/// Pemilih warna aksen/teks.
pub fn color_picker(ui: &mut Ui, label: &str, warna: &mut egui::Color32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.color_edit_button_srgba(warna);
    });
}
