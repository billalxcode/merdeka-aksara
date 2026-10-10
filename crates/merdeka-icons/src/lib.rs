//! merdeka-icons - seluruh ikon Tabler, tinggal panggil.
//!
//! Enum `Outline` (5000+) dan `Filled` (1000+) dibuat otomatis oleh `build.rs`
//! dari nama file di `assets/icons/tabler/`. Tidak perlu load satu-satu:
//! panggil varian atau metodenya, file SVG dibaca otomatis saat dipakai
//! (dan di-cache oleh egui).
//!
//! Contoh:
//! ```ignore
//! use merdeka_icons::{Filled, Icon, Outline};
//!
//! ui.add(Outline::Bold.image(16.0));            // enum langsung
//! ui.add(Icon::from_name("file-text").image(16.0)); // dari string
//! ui.add(merdeka_icons::icon_named("filled/star", 16.0)); // string + filled
//! ```

use std::path::PathBuf;

include!(concat!(env!("OUT_DIR"), "/merdeka_icons_generated.rs"));

/// Akar aset tabler (berisi `outline/`, `filled/`, `LICENSE`).
fn assets_root() -> PathBuf {
    const REL: &str = "assets/icons/tabler";
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join(REL);
        if p.join("LICENSE").is_file() {
            return p;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        for d in exe.ancestors().skip(1).take(6) {
            let p = d.join(REL);
            if p.join("LICENSE").is_file() {
                return p;
            }
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/icons/tabler")
}

/// Path file SVG untuk gaya (`outline`/`filled`) + nama kebab.
fn svg_path_for(gaya: &str, nama: &str) -> PathBuf {
    assets_root().join(format!("{gaya}/{nama}.svg"))
}

/// Path file SVG dari string: `"bold"` atau `"filled/star"`.
pub fn svg_path(nama: &str) -> PathBuf {
    match nama.strip_prefix("filled/") {
        Some(sisa) => svg_path_for("filled", sisa),
        None => svg_path_for("outline", nama),
    }
}

/// Gambar egui dari gaya + nama + ukuran kotak px.
pub fn image_for(gaya: &str, nama: &str, ukuran: f32) -> egui::Image<'static> {
    let p = svg_path_for(gaya, nama);
    let abs = std::path::absolute(&p).unwrap_or(p);
    egui::Image::from_uri(format!("file://{}", abs.display()))
        .fit_to_exact_size(egui::vec2(ukuran, ukuran))
}

/// Gambar egui dari enum + ukuran kotak px. Ini cara utama memakai ikon.
pub fn icon(ikon: Icon, ukuran: f32) -> egui::Image<'static> {
    ikon.image(ukuran)
}

/// Gambar egui dari string nama (`"bold"`, `"filled/star"`) + ukuran px.
pub fn icon_named(nama: &str, ukuran: f32) -> egui::Image<'static> {
    match nama.strip_prefix("filled/") {
        Some(sisa) => image_for("filled", sisa, ukuran),
        None => image_for("outline", nama, ukuran),
    }
}

impl Icon {
    /// Gambar egui dari enum ini. Sama dengan [`icon`].
    pub fn gambar(&self, ukuran: f32) -> egui::Image<'static> {
        icon(*self, ukuran)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumlah_masuk_akal() {
        assert!(Outline::COUNT > 5000, "outline {}", Outline::COUNT);
        assert!(Filled::COUNT > 1000, "filled {}", Filled::COUNT);
    }

    #[test]
    fn varian_dasar_benar() {
        assert_eq!(Outline::Bold.name(), "bold");
        assert_eq!(Outline::FileText.name(), "file-text");
        assert_eq!(Outline::from_name("folder-open"), Some(Outline::FolderOpen));
        assert_eq!(Outline::from_name("tidak-ada"), None);
        assert!(!Icon::Outline(Outline::Bold).is_filled());
    }

    #[test]
    fn semua_file_punya_varian() {
        let root = assets_root();
        for gaya_dir in ["outline", "filled"] {
            let mut n = 0;
            for entri in std::fs::read_dir(root.join(gaya_dir)).unwrap() {
                let p = entri.unwrap().path();
                if p.extension().and_then(|e| e.to_str()) != Some("svg") {
                    continue;
                }
                let nama = p.file_stem().unwrap().to_str().unwrap();
                let ada = if gaya_dir == "outline" {
                    Outline::from_name(nama).is_some()
                } else {
                    Filled::from_name(nama).is_some()
                };
                assert!(ada, "tanpa varian: {gaya_dir}/{nama}");
                n += 1;
            }
            assert!(n > 0, "kosong: {gaya_dir}");
        }
        assert_eq!(Outline::all().count(), Outline::COUNT);
        assert_eq!(Filled::all().count(), Filled::COUNT);
    }

    #[test]
    fn icon_dan_path_bisa_dipakai() {
        assert!(svg_path("bold").is_file());
        assert!(svg_path("filled/star").is_file());
        assert_eq!(Icon::from_name("bold"), Some(Icon::Outline(Outline::Bold)));
        let isi = std::fs::read_to_string(svg_path("bold")).unwrap();
        assert!(isi.contains("<svg"));
        let _ = icon(Icon::Outline(Outline::Bold), 16.0);
        let _ = Outline::Search.image(14.0);
        let _ = icon_named("printer", 16.0);
    }
}
