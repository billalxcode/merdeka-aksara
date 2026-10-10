---
name: merdeka-team
description: Planner, executor, qa, manager workflow for rencana, eksekusi, review, orchestrator, tim, delegasi, laporan. Use when a task needs planning, delegation to subagents, or a QA gate before finishing.
license: MIT
compatibility: opencode
---

# Merdeka Team

Four roles, one pipeline. Manager orchestrates via subagents; humans see one
summary. All artifacts live in `.caches/` (private, git-ignored).

## Peran

- **manager** (`.opencode/agents/manager.md`): orkestrator dan ide. Terima tujuan,
  tulis brief, delegasikan planner -> executor -> qa berurutan, rangkum hasil.
  Jangan kerjakan sendiri kecuali sepele (<3 langkah).
- **planner** (`.opencode/agents/planner.md`): merencanakan. Eksplorasi repo,
  hasilkan rencana bernomor. Hanya boleh tulis `.caches/`, jangan sentuh kode.
- **executor** (`.opencode/agents/executor.md`): mengeksekusi plan langkah per
  langkah, patuh skill `software-engineer`. Di luar plan: stop, kembalikan.
- **qa** (`.opencode/agents/qa.md`): menjaga stabilitas. Jalankan fmt, clippy,
  test, grep unicode. Tulis verdict. Jangan edit kode sumber, hanya `.caches/`.

## Alur baku

1. manager tulis `.caches/<slug>-brief.md` (tujuan, batasan, definisi selesai).
2. planner tulis `.caches/<slug>-plan.md` (langkah, file tersentuh, risiko, test).
3. manager setujui plan (minta revisi bila kabur).
4. executor kerjakan plan, laporkan file diubah + bukti check.
5. qa verifikasi independen, tulis `.caches/<slug>-qa.md` (LOLOS/GAGAL + temuan).
6. Bila GAGAL: kembali ke langkah 4 dengan temuan qa. Bila LOLOS: manager tulis
   `.caches/<slug>-laporan.md` dan rangkum ke user.

## Aturan `.caches/`

- Nama: `<slug>-brief.md`, `<slug>-plan.md`, `<slug>-qa.md`, `<slug>-laporan.md`.
  Slug = huruf kecil + strip, misal `tema-aksara`.
- Tiap file diawali: Judul, Tanggal (UTC), Status (draf/setuju/lolos/gagal).
- ASCII saja. Jangan commit (sudah di-ignore).
- Yang layak publik dan permanen dipindah ke `docs/` hanya atas izin user.
