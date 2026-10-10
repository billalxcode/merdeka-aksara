//! Ikon Tabler (SVG) untuk semua aplikasi Merdeka.
//!
//! Aset: `assets/icons/tabler/{outline,filled}/*.svg` + `LICENSE` (MIT, Tabler Icons).
//! Loader image dipasang di `run_page`, sehingga `egui::Image::from_uri("file://...")`
//! bisa menampilkan SVG langsung.
//!
//! Contoh:
//! ```ignore
//! ui.add(merdeka_ui::icons::icon("bold", 16.0));
//! ui.add(merdeka_ui::icons::icon("filled/star", 16.0));
//! ```

use std::path::PathBuf;

/// Cari file SVG ikon tabler.
///
/// `nama` tanpa `.svg`, misal `"bold"`. Awalan `"filled/"` memilih gaya filled,
/// default outline. Mengembalikan path absolut bila ketemu, relatif bila tidak.
pub fn svg_path(nama: &str) -> PathBuf {
    let (gaya, file) = match nama.strip_prefix("filled/") {
        Some(sisa) => ("filled", sisa),
        None => ("outline", nama),
    };
    let rel = format!("assets/icons/tabler/{gaya}/{file}.svg");
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join(&rel);
        if p.is_file() {
            return p;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        for d in exe.ancestors().skip(1).take(5) {
            let q = d.join(&rel);
            if q.is_file() {
                return q;
            }
        }
    }
    PathBuf::from(rel)
}

/// Gambar ikon tabler siap `ui.add(...)`, ukuran kotak `ukuran` px.
pub fn icon(nama: &str, ukuran: f32) -> egui::Image<'static> {
    let abs = std::path::absolute(svg_path(nama)).unwrap_or_else(|_| svg_path(nama));
    egui::Image::from_uri(format!("file://{}", abs.display()))
        .fit_to_exact_size(egui::vec2(ukuran, ukuran))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/icons/tabler")
    }

    #[test]
    fn lisensi_ikut() {
        assert!(root().join("LICENSE").is_file());
    }

    #[test]
    fn ikon_wajib_ada() {
        for n in [
            "bold",
            "italic",
            "underline",
            "list",
            "table",
            "photo",
            "file-text",
            "folder-open",
            "printer",
            "search",
        ] {
            assert!(
                root().join("outline").join(format!("{n}.svg")).is_file(),
                "ikon hilang: {n}"
            );
        }
    }

    #[test]
    fn svg_valid_dan_bisa_dibaca() {
        let isi = std::fs::read_to_string(root().join("outline/bold.svg")).unwrap();
        assert!(isi.contains("<svg"));
        assert!(svg_path("bold").is_file());
    }
}
