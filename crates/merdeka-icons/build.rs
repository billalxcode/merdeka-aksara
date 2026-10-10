//! Build script: hasilkan enum ikon dari nama file SVG.
//!
//! Membaca `../../assets/icons/tabler/{outline,filled}/*.svg` lalu menulis
//! `merdeka_icons_generated.rs` ke OUT_DIR berisi:
//! - `pub enum Outline` + `pub enum Filled` (satu varian per ikon)
//! - `name()`, `from_name()`, `all()`, `COUNT`, `image()`
//! - `pub enum Icon` gabungan + `name()`, `is_filled()`, `from_name()`

use std::collections::HashSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let tabler = crate_dir.join("../../assets/icons/tabler");
    let outline_dir = tabler.join("outline");
    let filled_dir = tabler.join("filled");
    println!("cargo::rerun-if-changed={}", outline_dir.display());
    println!("cargo::rerun-if-changed={}", filled_dir.display());

    let outline = baca(&outline_dir);
    let filled = baca(&filled_dir);
    assert!(!outline.is_empty(), "tidak ada SVG outline");
    assert!(!filled.is_empty(), "tidak ada SVG filled");

    let mut out = String::new();
    out.push_str("// Dibuat otomatis oleh build.rs - jangan edit manual.\n");
    tulis_enum(&mut out, "Outline", "outline", &outline);
    tulis_enum(&mut out, "Filled", "filled", &filled);
    tulis_icon(&mut out);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("merdeka_icons_generated.rs"), out).unwrap();
}

/// Baca direktori SVG -> daftar (nama_kebab, Variant) terurut.
fn baca(dir: &Path) -> Vec<(String, String)> {
    let mut entri: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|_| panic!("tidak bisa baca {}", dir.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("svg"))
        .collect();
    entri.sort();

    let mut dipakai = HashSet::new();
    let mut hasil = Vec::new();
    for p in entri {
        let nama = p.file_stem().unwrap().to_str().unwrap().to_owned();
        assert!(nama.is_ascii(), "nama ikon harus ASCII: {nama}");
        let dasar = varian(&nama);
        let mut var = dasar.clone();
        let mut i = 2;
        while !dipakai.insert(var.clone()) {
            var = format!("{dasar}{i}");
            i += 1;
        }
        hasil.push((nama, var));
    }
    hasil
}

/// "file-text" -> "FileText", "2fa" -> "N2fa", "a-b-2" -> "AB2".
fn varian(nama: &str) -> String {
    let mut s = String::new();
    for bagian in nama.split('-') {
        if bagian.is_empty() {
            continue;
        }
        let mut cs = bagian.chars();
        if let Some(pertama) = cs.next() {
            s.extend(pertama.to_uppercase());
            for c in cs {
                if c.is_ascii_alphanumeric() {
                    s.push(c.to_ascii_lowercase());
                }
            }
        }
    }
    if s.is_empty() {
        s.push('N');
    } else if s.chars().next().unwrap().is_ascii_digit() {
        s.insert(0, 'N');
    }
    s
}

fn tulis_enum(out: &mut String, enum_nama: &str, gaya: &str, ikon: &[(String, String)]) {
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(out, "pub enum {enum_nama} {{").unwrap();
    for (_, v) in ikon {
        writeln!(out, "    {v},").unwrap();
    }
    writeln!(out, "}}").unwrap();

    writeln!(out, "impl {enum_nama} {{").unwrap();
    writeln!(
        out,
        "    pub const COUNT: usize = {}; // jumlah ikon",
        ikon.len()
    )
    .unwrap();

    writeln!(out, "    pub fn all() -> impl Iterator<Item = Self> {{").unwrap();
    writeln!(out, "        [").unwrap();
    for (_, v) in ikon {
        writeln!(out, "            Self::{v},").unwrap();
    }
    writeln!(out, "        ].into_iter()").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "    pub fn name(&self) -> &'static str {{").unwrap();
    writeln!(out, "        match self {{").unwrap();
    for (n, v) in ikon {
        writeln!(out, "            Self::{v} => \"{n}\",").unwrap();
    }
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "    pub fn from_name(nama: &str) -> Option<Self> {{").unwrap();
    writeln!(out, "        match nama {{").unwrap();
    for (n, v) in ikon {
        writeln!(out, "            \"{n}\" => Some(Self::{v}),").unwrap();
    }
    writeln!(out, "            _ => None,").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(
        out,
        "    pub fn image(&self, ukuran: f32) -> egui::Image<'static> {{"
    )
    .unwrap();
    writeln!(
        out,
        "        crate::image_for(\"{gaya}\", self.name(), ukuran)"
    )
    .unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
}

fn tulis_icon(out: &mut String) {
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(out, "pub enum Icon {{").unwrap();
    writeln!(out, "    Outline(Outline),").unwrap();
    writeln!(out, "    Filled(Filled),").unwrap();
    writeln!(out, "}}").unwrap();

    writeln!(out, "impl Icon {{").unwrap();
    writeln!(out, "    pub fn is_filled(&self) -> bool {{").unwrap();
    writeln!(out, "        matches!(self, Self::Filled(_))").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "    pub fn name(&self) -> &'static str {{").unwrap();
    writeln!(out, "        match self {{").unwrap();
    writeln!(out, "            Self::Outline(o) => o.name(),").unwrap();
    writeln!(out, "            Self::Filled(f) => f.name(),").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "    pub fn from_name(nama: &str) -> Option<Self> {{").unwrap();
    writeln!(
        out,
        "        if let Some(sisa) = nama.strip_prefix(\"filled/\") {{"
    )
    .unwrap();
    writeln!(
        out,
        "            return Filled::from_name(sisa).map(Self::Filled);"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "        Outline::from_name(nama).map(Self::Outline)").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(
        out,
        "    pub fn image(&self, ukuran: f32) -> egui::Image<'static> {{"
    )
    .unwrap();
    writeln!(out, "        match self {{").unwrap();
    writeln!(out, "            Self::Outline(o) => o.image(ukuran),").unwrap();
    writeln!(out, "            Self::Filled(f) => f.image(ukuran),").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
}
