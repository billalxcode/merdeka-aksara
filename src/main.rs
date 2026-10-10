use merdeka_ui::{
    MerdekaConfig, MerdekaPage, MerdekaTheme,
    components::{
        self, BannerKind, Konfirmasi, Pita, alat_button, alat_toggle, empty_state, grup, notice,
        page_paper, ruler_stub,
    },
};

const FONT_OPSI: &[&str] = &["Inter", "Liberation Serif", "Noto Sans", "Ubuntu Mono"];
const UKURAN_OPSI: &[f32] = &[10.0, 11.0, 12.0, 14.0, 16.0, 18.0, 24.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum RataTeks {
    #[default]
    Kiri,
    Tengah,
    Kanan,
    Rata,
}

impl RataTeks {
    fn label(self) -> &'static str {
        match self {
            RataTeks::Kiri => "< Rata Kiri",
            RataTeks::Tengah => "<> Tengah",
            RataTeks::Kanan => "> Rata Kanan",
            RataTeks::Rata => "= Justify",
        }
    }
}

#[derive(Default)]
struct AksaraPage {
    teks: String,
    kata: usize,
    tebal: bool,
    miring: bool,
    garis_bawah: bool,
    font: String,
    ukuran: f32,
    warna: egui::Color32,
    rata: RataTeks,
    gaya: String,
    zoom: f32,
    tampil_ruler: bool,
    tampil_sidebar: bool,
    lacak: bool,
    komentar: u32,
    margin: String,
    orientasi: String,
    kolom: String,
    bahasa: String,
    pesan: Option<(BannerKind, String)>,
    konfirm_baru: bool,
    tabel_dialog: bool,
    tabel_baris: u32,
    tabel_kolom: u32,
}

impl AksaraPage {
    fn new() -> Self {
        Self {
            font: "Inter".to_owned(),
            ukuran: 12.0,
            warna: egui::Color32::BLACK,
            gaya: "Normal".to_owned(),
            zoom: 1.0,
            tampil_ruler: true,
            tampil_sidebar: true,
            margin: "Normal".to_owned(),
            orientasi: "Potret".to_owned(),
            kolom: "1".to_owned(),
            bahasa: "Indonesia".to_owned(),
            tabel_baris: 3,
            tabel_kolom: 3,
            ..Default::default()
        }
    }

    fn hitung(&mut self) {
        self.kata = self.teks.split_whitespace().count();
    }

    fn info(&mut self, jenis: BannerKind, teks: impl Into<String>) {
        self.pesan = Some((jenis, teks.into()));
    }
}

impl MerdekaPage for AksaraPage {
    fn ribbon(&mut self, ui: &mut egui::Ui, tab: Pita) {
        match tab {
            Pita::Berkas => {
                grup(ui, "Dokumen", |ui| {
                    if alat_button(ui, "+", "Baru", "Dokumen kosong").clicked() {
                        self.konfirm_baru = true;
                    }
                    if alat_button(ui, "O", "Buka", "Buka DOCX/ODT/Markdown (segera hadir)")
                        .clicked()
                    {
                        self.info(BannerKind::Info, "Buka berkas: segera hadir di P1 IO.");
                    }
                    if alat_button(ui, "S", "Simpan", "Simpan dokumen").clicked() {
                        self.info(
                            BannerKind::Sukses,
                            format!(
                                "Draf tersimpan ({} kata) - tulis ke disk menyusul.",
                                self.kata
                            ),
                        );
                    }
                });
                ui.separator();
                grup(ui, "Ekspor", |ui| {
                    if alat_button(ui, "P", "PDF", "Ekspor PDF (segera hadir)").clicked() {
                        self.info(BannerKind::Info, "Ekspor PDF: segera hadir di P1 IO.");
                    }
                    if alat_button(ui, "E", "EPUB", "Ekspor EPUB (segera hadir)").clicked() {
                        self.info(BannerKind::Info, "Ekspor EPUB: segera hadir di P4.");
                    }
                });
            }
            Pita::Beranda => {
                grup(ui, "Font", |ui| {
                    components::font_picker(ui, &mut self.font, FONT_OPSI);
                    components::size_picker(ui, &mut self.ukuran, UKURAN_OPSI);
                    alat_toggle(ui, &mut self.tebal, "B", "Tebal", "Tebal (Ctrl+B)");
                    alat_toggle(ui, &mut self.miring, "I", "Miring", "Miring (Ctrl+I)");
                    alat_toggle(
                        ui,
                        &mut self.garis_bawah,
                        "U",
                        "Garis",
                        "Garis bawah (Ctrl+U)",
                    );
                    components::color_picker(ui, "Warna", &mut self.warna);
                });
                ui.separator();
                grup(ui, "Paragraf", |ui| {
                    for r in [
                        RataTeks::Kiri,
                        RataTeks::Tengah,
                        RataTeks::Kanan,
                        RataTeks::Rata,
                    ] {
                        let mut aktif = self.rata == r;
                        if components::toggle_chip(ui, &mut aktif, r.label()).clicked() && aktif {
                            self.rata = r;
                        }
                    }
                    if alat_button(ui, "-", "Poin", "Daftar poin").clicked() {
                        self.teks.push_str("\n- ");
                        self.hitung();
                    }
                    if alat_button(ui, "1.", "Nomor", "Daftar bernomor").clicked() {
                        self.teks.push_str("\n1. ");
                        self.hitung();
                    }
                });
                ui.separator();
                grup(ui, "Gaya", |ui| {
                    for g in ["Normal", "Judul 1", "Judul 2", "Judul"] {
                        let mut aktif = self.gaya == g;
                        if components::toggle_chip(ui, &mut aktif, g).clicked() && aktif {
                            self.gaya = g.to_owned();
                        }
                    }
                });
            }
            Pita::Sisip => {
                grup(ui, "Tabel & Media", |ui| {
                    if alat_button(ui, "T", "Tabel", "Sisip tabel").clicked() {
                        self.tabel_dialog = true;
                    }
                    if alat_button(ui, "G", "Gambar", "Sisip gambar (segera hadir)").clicked() {
                        self.info(BannerKind::Info, "Sisip gambar: segera hadir di P1.");
                    }
                    if alat_button(ui, "H", "Header", "Header (segera hadir)").clicked() {
                        self.info(BannerKind::Info, "Header/footer: segera hadir di P1.");
                    }
                    if alat_button(ui, "#", "Nomor", "Nomor halaman (segera hadir)").clicked() {
                        self.info(BannerKind::Info, "Nomor halaman: segera hadir di P1.");
                    }
                });
                ui.separator();
                grup(ui, "Tautan", |ui| {
                    if alat_button(ui, "@", "Link", "Sisip hyperlink").clicked() {
                        self.teks.push_str(" [teks](https://...)");
                        self.hitung();
                    }
                    if alat_button(ui, "B", "Bookmark", "Bookmark (segera hadir)").clicked() {
                        self.info(BannerKind::Info, "Bookmark: segera hadir di P2.");
                    }
                });
            }
            Pita::TataLetak => {
                grup(ui, "Halaman", |ui| {
                    components::option_picker(
                        ui,
                        "Margin",
                        &mut self.margin,
                        &["Normal", "Sempit", "Lebar"],
                    );
                    components::option_picker(
                        ui,
                        "Orientasi",
                        &mut self.orientasi,
                        &["Potret", "Lanskap"],
                    );
                    components::option_picker(ui, "Kolom", &mut self.kolom, &["1", "2", "3"]);
                });
                ui.separator();
                grup(ui, "Tampil", |ui| {
                    components::checkbox_row(ui, &mut self.tampil_ruler, "Penggaris");
                    components::checkbox_row(ui, &mut self.tampil_sidebar, "Sidebar");
                });
            }
            Pita::Referensi => {
                grup(ui, "Daftar & Catatan", |ui| {
                    if alat_button(ui, "=", "Daftar Isi", "Daftar isi otomatis (P2)").clicked() {
                        self.info(BannerKind::Info, "Daftar isi otomatis: segera hadir di P2.");
                    }
                    if alat_button(ui, "1", "Catatan", "Sisip catatan kaki").clicked() {
                        self.teks.push_str("[^1]");
                        self.hitung();
                    }
                    if alat_button(ui, "K", "Keterangan", "Keterangan gambar/tabel (P2)").clicked()
                    {
                        self.info(
                            BannerKind::Info,
                            "Keterangan + cross-ref: segera hadir di P2.",
                        );
                    }
                });
            }
            Pita::Tinjau => {
                grup(ui, "Review", |ui| {
                    if alat_button(ui, "C", "Komentar", "Tambah komentar").clicked() {
                        self.komentar += 1;
                        self.info(
                            BannerKind::Sukses,
                            format!("Komentar #{} ditambahkan.", self.komentar),
                        );
                    }
                    components::checkbox_row(ui, &mut self.lacak, "Lacak perubahan");
                    if alat_button(ui, "ABC", "Ejaan", "Periksa ejaan (P3)").clicked() {
                        self.info(BannerKind::Info, "Ejaan hunspell: segera hadir di P3.");
                    }
                });
            }
            Pita::Tampil => {
                grup(ui, "Zoom", |ui| {
                    components::zoom_slider(ui, &mut self.zoom);
                });
                ui.separator();
                grup(ui, "Panel", |ui| {
                    components::checkbox_row(ui, &mut self.tampil_ruler, "Penggaris");
                    components::checkbox_row(ui, &mut self.tampil_sidebar, "Sidebar");
                });
                ui.separator();
                grup(ui, "Tema", |ui| {
                    ui.label("Terang (dikunci)");
                });
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        // Konfirmasi dokumen baru (pakai dialog reusable).
        if self.konfirm_baru
            && components::confirm(
                ui.ctx(),
                &mut self.konfirm_baru,
                "Dokumen Baru",
                "Buang isi dokumen saat ini dan mulai kosong?",
            ) == Konfirmasi::Ya
        {
            self.teks.clear();
            self.hitung();
            self.info(BannerKind::Sukses, "Dokumen baru dibuat.");
        }

        // Dialog sisip tabel (modal tengah).
        if self.tabel_dialog {
            let mut baris = self.tabel_baris;
            let mut kolom = self.tabel_kolom;
            let mut sisip = false;
            let mut batal = false;
            components::modal(
                ui.ctx(),
                &mut self.tabel_dialog,
                "Sisip Tabel",
                |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Baris");
                        ui.add(egui::DragValue::new(&mut baris).range(1..=10));
                        ui.label("Kolom");
                        ui.add(egui::DragValue::new(&mut kolom).range(1..=8));
                    });
                },
                |ui| {
                    if components::primary_button(ui, "Sisipkan").clicked() {
                        sisip = true;
                    }
                    if components::secondary_button(ui, "Batal").clicked() {
                        batal = true;
                    }
                },
            );
            self.tabel_baris = baris;
            self.tabel_kolom = kolom;
            if batal {
                self.tabel_dialog = false;
            }
            if sisip {
                let (b, k) = (baris as usize, kolom as usize);
                let mut t = String::from("\n");
                for _ in 0..b {
                    t.push('|');
                    for _ in 0..k {
                        t.push_str(" sel |");
                    }
                    t.push('\n');
                }
                self.teks.push_str(&t);
                self.hitung();
                self.tabel_dialog = false;
                self.info(BannerKind::Sukses, format!("Tabel {b}x{k} disisipkan."));
            }
        }

        // Sidebar navigasi (opsional).
        if self.tampil_sidebar {
            egui::Panel::left("aksara_navigasi")
                .default_size(200.0)
                .show(ui, |ui| {
                    components::sidebar_section(ui, "Navigasi", true, |ui| {
                        ui.label("P Halaman 1");
                        ui.label(" Cari di dokumen...");
                    });
                    components::sidebar_section(ui, "Gaya", true, |ui| {
                        ui.label(format!("Aktif: {}", self.gaya));
                        ui.label(format!("Font: {} {:.0} pt", self.font, self.ukuran));
                    });
                });
        }

        // Kanvas dokumen.
        egui::CentralPanel::default().show(ui, |ui| {
            if let Some((jenis, teks)) = self.pesan.clone() {
                ui.horizontal(|ui| {
                    notice(ui, jenis, &teks);
                    if components::ghost_button(ui, "Tutup").clicked() {
                        self.pesan = None;
                    }
                });
                ui.add_space(4.0);
            }

            let lebar = MerdekaTheme::default().lebar_dokumen * self.zoom.clamp(0.5, 2.0);
            components::document_scroll(ui, lebar, |ui| {
                if self.tampil_ruler {
                    ruler_stub(ui);
                    ui.add_space(4.0);
                }
                page_paper(ui, |ui| {
                    let mut contoh = egui::RichText::new(format!(
                        "{} - {} {:.0} pt",
                        self.gaya, self.font, self.ukuran
                    ))
                    .size(14.0 * self.zoom);
                    if self.tebal {
                        contoh = contoh.strong();
                    }
                    if self.miring {
                        contoh = contoh.italics();
                    }
                    if self.garis_bawah {
                        contoh = contoh.underline();
                    }
                    ui.label(contoh.color(self.warna));
                    ui.separator();

                    let keluar = egui::TextEdit::multiline(&mut self.teks)
                        .desired_rows(22)
                        .desired_width(f32::INFINITY)
                        .hint_text("Tulis di sini... bebas pakai, bebas salin, bebas edit.")
                        .frame(egui::Frame::NONE)
                        .show(ui);
                    if keluar.response.changed() {
                        self.hitung();
                    }

                    if self.teks.trim().is_empty() {
                        ui.add_space(8.0);
                        if empty_state(
                            ui,
                            "*",
                            "Belum ada tulisan",
                            "Mulai mengetik di atas. Semua milikmu, tanpa batasan.",
                            "Fokus Menulis",
                        ) {
                            keluar.response.request_focus();
                        }
                    }
                });
                ui.add_space(16.0);
            });
        });
    }

    fn status_kiri(&self) -> String {
        format!(
            "Halaman 1 - {} - {} - {} - {} kolom",
            self.bahasa, self.margin, self.orientasi, self.kolom,
        )
    }

    fn status_kanan(&self) -> String {
        format!(
            "{} kata - {} karakter - {:.0}% - Terang{}",
            self.kata,
            self.teks.len(),
            self.zoom * 100.0,
            if self.lacak { " - Lacak" } else { "" },
        )
    }
}

fn main() -> eframe::Result {
    let config = MerdekaConfig::new("Merdeka Aksara", env!("CARGO_PKG_VERSION"));
    merdeka_ui::run_page(config, |_cc| AksaraPage::new())
}
