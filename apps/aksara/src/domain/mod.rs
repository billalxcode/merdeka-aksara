//! Domain layer: aturan bisnis inti Aksara.
//!
//! Murni logika dokumen - tidak tahu menahu soal egui, file, atau UI.
//! Tidak boleh depend ke layer lain.

pub mod dokumen;
pub mod gaya;

pub use dokumen::Dokumen;
pub use gaya::{RataTeks, Rgb};
