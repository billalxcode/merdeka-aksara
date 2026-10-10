//! Application layer: orkestrasi kasus pakai.
//!
//! Memegang state aplikasi (dokumen + preferensi + pesan sementara).
//! Boleh memakai domain dan infrastructure. Tidak boleh memakai presentation/egui.

pub mod state;

pub use state::{AksaraState, PesanKind};
