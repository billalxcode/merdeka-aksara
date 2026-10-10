---
name: software-engineer
description: Expert software engineer discipline for Rust, cargo, egui, refactor, fitur, bug, clippy, fmt, test, arsitektur layer, code review. Use when writing, changing, or reviewing code in this repo.
license: MIT
compatibility: opencode
---

# Software Engineer

Act as an expert software engineer on the Merdeka codebase (Rust workspace:
`apps/*` binaries, `crates/*` libs, `assets/`, `docs/`).

## Aturan keras (selalu berlaku)

1. **ASCII saja.** Tanpa unicode di kode, config, dan dokumen baru. Satu-satunya
   pengecualian: `docs/sign-of-ai.md` (data verbatim, jangan diubah).
2. **Bukti via eksekusi.** Setiap perubahan kode ditutup dengan:
   `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`. Semua harus lolos. Jangan klaim selesai tanpanya.
3. **Baca sebelum ubah.** Pakai Read dulu untuk tiap file yang diedit; jaga
   batas edit sekecil mungkin; cek ulang area edit setelahnya.
4. **Jangan commit/push/PR** kecuali user eksplisit meminta. Jangan ubah
   config git, jangan skip hooks.

## Arsitektur yang dijaga

- App Aksara berlapis: `presentation -> application -> {domain, infrastructure}`.
  Domain murni (tanpa egui). Aturan bisnis di domain/application, bukan di UI.
- Fondasi bersama di `crates/merdeka-ui` (komponen, shell, tema) dan
  `crates/merdeka-icons` (enum Tabler hasil generate `build.rs`).
- Tema terang dikunci; warna primer per aplikasi via preset `MerdekaTheme`
  (`aksara()` biru ocean `#2373F4`, default merah Merdeka). Komponen baca warna
  dari visuals aktif, jangan hardcode.
- Pola dependensi path lokal antar crate; aset di `assets/icons/tabler/`.

## egui 0.36 yang sering menjebak

- Panel: `Panel::{top,bottom,left,right}` + `CentralPanel`, show dari root `Ui`.
- `Rounding` -> `CornerRadius`, `window_rounding` -> `window_corner_radius`,
  `Frame::rounding` -> `corner_radius`.
- `TextEdit::frame` menerima `Frame` (`Frame::NONE`), bukan bool.
  `Button::frame` menerima bool. `RichText::color` (bukan `colored`).
- Trait app: `fn ui(&mut self, ui: &mut Ui, frame: &mut Frame)`.

## Artefak

- Publik dan permanen -> `docs/`. Coretan, plan, todo, laporan sementara ->
  `.caches/` (di-ignore git, jangan commit).
