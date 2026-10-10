---
name: ai-writing-signs
description: Deteksi dan perbaiki tanda tulisan AI untuk dokumen Merdeka Aksara. Audit gaya LLM (puffery, AI vocab, paralelisme negatif, markdown bocor, sitasi halusinasi) berdasar docs/sign-of-ai.md, lalu humanisasi jadi tulisan spesifik, netral, dan alami.
license: MIT
compatibility: opencode
metadata:
  audience: merdeka-aksara-contributors
  source: docs/sign-of-ai.md
  language: id-en
---

## Apa yang saya lakukan

- Audit teks dokumen (Indonesia / Inggris) terhadap pola khas chatbot LLM: ChatGPT, Claude, Gemini, Grok, DeepSeek, Perplexity.
- Klasifikasi temuan jadi `kuat`, `sedang`, `lemah` - bukan vonis final.
- Perbaiki teks agar spesifik, netral, dan manusiawi tanpa sekadar menyamarkan tanda (obati masalah dalamnya, bukan poles luarnya).
- Bantu implementasi fitur pemeriksa ini di Merdeka Aksara (Rust).

Sumber kebenaran: `docs/sign-of-ai.md` (Wikipedia: Signs of AI writing, 1969 baris). Skill ini adalah distilasi operasionalnya. Bila ragu, baca file sumber langsung.

## Kapan gunakan saya

Gunakan saat:
- mereview / mengedit dokumen di Merdeka Aksara,
- menulis dokumentasi, artikel, draft, komentar, atau commit message,
- user berkata "cek AI", "humanize", "tanda AI", "apakah ini AI?", "rapikan gaya LLM",
- mengimplementasikan linter / pemeriksa gaya di codebase Rust `merdeka-aksara`.

Jangan gunakan untuk: refactoring logika bisnis umum, debugging Rust yang tak terkait gaya tulisan.

## Prinsip penting (wajib patuhi)

1. **Deskriptif, bukan preskriptif.** Daftar ini adalah observasi, bukan aturan tata bahasa. Banyak pola juga muncul di tulisan manusia (editorial, blog, fan-fiction).
2. **Jangan tuduh hanya dari gaya.** Kemampuan manusia membedakan AI vs manusia ~= acak (57-64% di studi 2025). Detektor GPTZero/Pangram punya error rate non-trivial, sensitif pada parafrase dan model yang tak dikenal.
3. **Jangan poles tanda saja.** Contoh: menghapus bold berlebihan tapi membiarkan klaim halusinasi = membuat deteksi lebih sulit. Perbaiki masalah dalam: spesifisitas, netralitas, verifikasi sumber.
4. **Konteks penting.** `underscore` figuratif = curiga; `underscore` sebagai garis bawah literal = abaikan. `landscape` abstrak = curiga; `landscape` foto bentang alam = abaikan.
5. **Bahasa manusia ikut berubah.** Sejak 2024 tulisan manusia memang terpengaruh LLM. Satu kata bukan bukti.

## Sinyal KUAT (hampir pasti tempelan chatbot)

Langsung flag sebagai `kuat` bila ditemukan:

- **Sisa markup internal model:**
  - ChatGPT: `:contentReference[oaicite:0]{index=0}`, `oai_citation`, `[oai_citation:0#domain]`, `turn0search0` / `citeturn0search0`, `0`, `({"attribution":{"attributableIndex":"..."}})`, `Example+1`, `Wikipedia+1`, `Ignyte+3Microsoft Learn+3`
  - Gemini: `[cite: 1]`, `[cite: 3, 12, 13]`, `[span_1](start_span)` / `[span_1](end_span)`
  - Grok: `<grok-card data-id=...>`, `[](grok_render_citation_card_json={"cardIds":[...]})`
  - DeepSeek: `[85+L261-269]`, `[854140639155648+L119-L123]` (kurung lentikular + dagger)
  - Perplexity: `[attached_file:1]`, `[web:1]`, URL `ppl-ai-file-upload`, bucket S3
  - Tak dikenal: `:::writing{variant="document" id="12345"}` / `:::`
- **Markdown bocor di konteks non-Markdown:** `**bold**` untuk bold, `## Heading`, `[teks](https://...)`, `---` sebagai pembatas seksi tiap bagian, ``` ```wikitext ``` blok pagar tiga.
- **UTM chatbot di URL:** `?utm_source=chatgpt.com`, `utm_source=openai`, `utm_source=copilot.com`, `referrer=grok.com`.
- **Komunikasi kolaboratif nyasar ke isi:** `Certainly!`, `Of course!`, `I hope this helps`, `Would you like...`, `is there anything else`, `let me know`, `Here is a...`, `As an AI language model`, `as of my last knowledge update`, instruksi `<!-- SUBMISSION NOTES -->`, `[Your Name]`, `[Describe the specific section...]`, `PASTE_SPOTIFY_TRACK_URL_HERE`, `2025-XX-XX`, `INSERT_SOURCE_URL_30`.
- **Kategori/template halusinasi:** `[[Category:American hip hop musicians]]` (seharusnya `hip-hop`), `{{Infobox ancient population}}`, `{{AfC submission|d}}` yang pre-decline tanpa alasan, maintenance tag tak masuk akal.
- **Sitasi rusak massal:** banyak link 404 yang tak ada di Wayback Machine, DOI valid tapi mengarah ke artikel tak terkait, ISBN checksum invalid, buku tanpa halaman/URL yang tak verifikabel, `<references>` berisi named-ref tak dipakai di badan teks (`Cite error: list-defined reference ... is not used`).

## Sinyal SEDANG (kumpulkan 3+ baru curiga)

### 1. Content - regresi ke rata-rata (generik + melebihkan)

- **Peninggian signifikansi/legacy:** `stands/serves as`, `is a testament/reminder`, `crucial/pivotal/vital/significant/key role/moment`, `underscores/highlights its importance`, `reflects broader`, `symbolizing its enduring/lasting`, `contributing to`, `setting the stage`, `marking/shaping`, `represents/marks a shift`, `key turning point`, `evolving landscape`, `focal point`, `indelible mark`, `deeply rooted`. Contoh: etimologi desa kecil pun diklaim `highlights the enduring legacy... transformative power`.
- **Penekanan notabilitas kalengan:** `independent coverage`, `featured in`, `profiled in multiple high-quality outlets`, `significant sustained coverage`, `trade publications`, `regional/national media outlets`, `maintains an active/strong social media presence`. LLM meniru bunyi pedoman WP:N (`independent coverage`) seolah itu isi artikel.
- **Analisis superfisial partisipel:** kalimat diakhiri frasa `-ing` (`highlighting...`, `underscoring...`, `reflecting...`, `fostering...`, `enhancing...`, `cultivating...`, `valuable insights`, `align/resonate with`) tanpa sumber. Sering + atribusi samar.
- **Puffery promosi:** `boasts a`, `vibrant`, `rich`, `profound`, `showcasing`, `exemplifies`, `commitment to`, `nestled`, `breathtaking`, `in the heart of`, `groundbreaking`, `renowned`, `featuring`, `diverse array`, `gateway to`, `tapestry of`.
- **Koneksi samar:** `in connection with/to`, `connected with/to`, `in association with`, `associated with`, `became associated with` - padahal bisa ditulis langsung (`was CEO of`, `taught at`).
- **Atribusi samar + generalisasi:** `Industry reports`, `Observers have cited`, `Experts argue`, `Some critics argue`, `several sources` padahal hanya 1 sitasi, `such as` sebelum daftar yang diklaim tak lengkap.
- **Outline kaku Challenges/Future:** `Despite its [positif], X faces several challenges... Despite these challenges... positions them as critical... Future Outlook`. Formula akhir artikel generik.
- **Seksi `Awards and recognition` / `Recognition`:** judul `X and Y` + fokus legacy. Sangat umum di artikel AI.

### 2. Language - kosakata & sintaksis AI

Daftar kata pantau (kasus sensitif konteks). Ko-okurensi = makin kuat:

- **2023-mid-2024 (GPT-4):** `Additionally` (awal kalimat), `boasts`, `bolstered`, `crucial`, `delve`, `emphasizing`, `enduring`, `garner`, `intricate/intricacies`, `interplay`, `key` (adjektiva), `landscape` (abstrak), `meticulously`, `pivotal`, `underscore` (verba), `tapestry` (abstrak), `testament`, `valuable`, `vibrant`
- **mid-2024-mid-2025 (GPT-4o):** `align with`, `bolstered`, `crucial`, `emphasizing`, `enhance`, `enduring`, `fostering`, `highlighting`, `pivotal`, `showcasing`, `underscore`, `vibrant`
- **mid-2025-mid-2026 (GPT-5):** `emphasizing`, `enhance`, `highlighting`, `showcasing` (+ kosakata notabilitas di atas)
- **mid-2026+ (GPT-6):** `challenge`, `clearer`, `dependable`, `echoed`, `foster`, `leverage`, `matters`, `multifaceted`, `practical`, `prioritize`, `quietly`, `steady`, `universally` + kalimat pendek 8-20 kata, koma/titik dominan, em-dash dihindari total
- **Grok:** `causal`, `empirical`, `correlate`, `underscore` persisten

Pola sintaksis:

- **Hindari kopula:** `serves/stands/marks/functions/operates/represents as [a]` ganti `is/are`; `features/offers/maintains/boasts [a]` ganti `has`; `refers to` ganti definisi langsung. Contoh buruk: `Gallery 825 serves as LAAA's exhibition space` -> baik: `Gallery 825 is LAAA's exhibition arm`.
- **Paralelisme negatif:** `Not only X but also Y`, `It's not X, it's Y`, `no X, no Y, just Z`, `doesn't just X; it Y`, `rather than X`, `X rather than Y`. Sekali = gaya; bertubi-tubi = khas LLM.
- **Rule of three:** `adjective, adjective, adjective` atau `frasa, frasa, and frasa` (`mythological image, magical weapons, and spirit of resistance`). Kuat bila muncul di tempat yang tak perlu gaya.
- **Lead perlakukan judul generik sebagai proper noun:** `The "List of songs about Mexico" is a curated compilation...`, `Catchment area (health) refers to...`.

### 3. Style - format mekanis

- **Title Case di heading:** `Impact of Technology and Digitalization` (seharusnya sentence case).
- **Bold berlebihan:** tiap istilah kunci di-bold ala slide/dek/listikel/key-takeaways.
- **Inline-header vertical list:** `- **Route Details**: ...`, `1. Historical Context Post-WWII Era: ...`, `Conflict of Interest (COI)/Autobiography: ...`. Sering pakai `- - - # emoji 1.` bukan markup asli.
- **Heading error:** heading judul artikel di badan, lompat level (`===` tanpa `==`), `= Level 1 =` berlebihan, heading isi hanya sub-heading.
- **Emoji sebagai format:** `       ` di heading/bullet. Dulu umum, kini lebih jarang tapi tetap flag bila di artikel formal.
- **Tabel mini tak perlu:** tabel 2 kolom yang lebih baik jadi prosa/infobox.
- **Curly quotes/apostrof saja = lemah:** `"..." '...' '` dipakai ChatGPT/DeepSeek, tapi Word/macOS smart-quotes, LanguageTool, koran besar juga pakai. Jangan jadikan bukti tunggal. Gemini/Claude biasanya straight quotes.

### 4. Komunikasi & hedging

- **Disclaimer keterbatasan sumber (RAG gagal):** `While specific details are limited/scarce...`, `not widely available/documented/disclosed`, `in the provided/available sources`, `based on available information`, `should be treated as X rather than Y`, `does not by itself establish...`, `maintains a low profile / keeps personal details private` (spekulatif soal privat).
- **Didaktik lama (2022-2024):** `it's important/crucial/critical to note/remember`, `worth noting`.
- **Ringkasan tempelan:** seksi `Conclusion`, `In summary`, `In conclusion`, `Overall` di tiap akhir - khas model lama.
- **Edit summary AI:** template kaku `Revised X for clarity and compliance...`, `ensured neutrality, improved flow`, `while preserving original meaning`, `added sourced content with independent/secondary sources`, menyebut nama parameter/template (`infobox parameters (image_size)`, `inline citations`), `Addressing reviewer feedback...`.

## Workflow audit (jalankan berurutan)

1. **Pindai cepat (regex literal):** cari sinyal KUAT dulu. Satu temuan kuat -> langsung `flag: kuat, perlu verifikasi sumber`.
   - Pola: `contentReference|oaicite|oai_citation|turn0search|turn0image|attributableIndex|start_span|cite: \d|grok_card|grok_render_citation|[\d++|attached_file|ppl-ai-file-upload|utm_source=(openai|chatgpt|copilot)|referrer=grok|:::writing|```wikitext|^\s*##\s|^\s*---\s*$|\[.*\]\(https?://`
2. **Hitung kepadatan SEDANG:** hitung kemunculan AI-vocab + frasa legacy + paralelisme negatif + rule-of-three per 500 kata. 1-2 = abaikan; 3-5 beda kategori = `sedang`; 6+ / ko-okurensi (`showcasing + fostering + underscored` dalam 1 paragraf) = `kuat`.
3. **Cek sitasi & fakta:** buka tiap link, validasi DOI/ISBN checksum, pastikan DOI mengarah ke artikel yang diklaim, buku ada nomor halaman + kutipan verifikabel. Tandai halusinasi.
4. **Cek histori & penjelasan:** bila ada riwayat (git/wiki), bandingkan gaya lama vs baru (pergeseran tiba-tiba pasca Nov 2022 curiga; gaya konsisten sejak lama = manusia). Minta penulis menjelaskan pilihan editorialnya - manusia bisa menjelaskan, tempelan LLM biasanya tidak.
5. **Putuskan:** `bersih` / `perlu humanisasi` / `perlu verifikasi sumber` / `kuat-indikasi-AI-mentah`. Jangan tulis "terbukti AI 100%". Jelaskan bukti + lokasi baris.
6. **Perbaiki (humanize):** lihat bagian bawah. Setelah perbaiki, ulangi langkah 1-2 untuk memastikan skor turun tanpa menghilangkan makna.

Format laporan (Indonesia, ringkas):

```text
Hasil: [bersih / sedang / kuat] - skor <n> temuan / 500 kata
Kuat:
- baris 12: `turn0search0` sisa sitasi ChatGPT
Sedang:
- baris 8,21,30: `showcasing, fostering, underscored` ko-okurensi
- baris 15: `serves as` -> ganti `is`
Saran perbaikan: [2-3 bullet konkret]
```

## Cara memperbaiki (jangan sekadar sembunyikan tanda)

- **Spesifikkan, jangan agungkan:** hapus `pivotal moment / testament / vibrant tapestry`. Ganti dengan fakta: siapa, kapan, berapa. `marking a pivotal moment in regional statistics` -> `established in 1989; by 1992 it published X survey covering Y municipalities`.
- **Kembalikan kopula sederhana:** `serves as -> is`, `features four spaces -> has four spaces`, `boasts a collection -> has a collection`, `authored -> wrote`, `utilized -> used`, `relocated -> moved`.
- **Pangkas analisis -ing:** hapus `offering a diverse range of experiences against...` bila tak ada sumber yang berkata begitu. Bila perlu, atribusi eksplisit: `Menurut [sumber], ...`.
- **Tegaskan koneksi:** `was associated with leadership of X` -> `was CEO of X (2017-2019, per [sumber])`.
- **Pecah paralelisme & triplets:** ubah `Not only beautiful but sustainable, innovative, and inclusive` -> satu klaim per kalimat + sumber.
- **Netralkan nada:** `nestled in breathtaking... offers fascinating glimpse` -> hapus adjective promosi, sisakan fakta geografis.
- **Perbaiki format:** Title Case -> sentence case; bold massal -> polos; list `**Header**: ...` -> prosa atau tabel asli bila data tabular; `##` -> heading level benar; `**bold**` -> bold native; `[x](y)` -> link native.
- **Verifikasi sitasi:** ganti link mati/DOI salah/ISBN invalid dengan sumber nyata yang dibaca. Tambah halaman buku. Hapus named-ref tak terpakai. Buang UTM chatbot.
- **Hapus sisa instruksi:** hapus `Certainly, Here's..., Let me know..., In conclusion...`, placeholder `[Your Name]`, submission statement untuk reviewer.
- **Pertahankan suara manusia:** boleh ada `there is a`, `it has a`, `very`, `perhaps`, `tends to`, `one of the best / is the only / was the first` bila didukung sumber, `wrote/moved/used/tried/died` - ini justru lebih manusiawi menurut data 25 tahun Wikipedia.

Contoh:

```text
Buruk: Douera enjoys close proximity to Algiers, further enhancing its significance as a dynamic hub, offering diverse experiences against Algeria's stunning natural beauty.
Baik: Douera terletak ~X km dari Algiers (sumber: ...). Sensus 2008 mencatat 56.998 penduduk (sumber: ...).
```

## Untuk Merdeka Aksara (Rust) - implementasi linter

Bila diminta implementasi fitur, buat modulator kecil, bukan monolit:

- `src/aksara_lint/patterns.rs`: tabel regex `KUAT` (literal di atas) + daftar `AI_VOCAB` per era + frasa legacy. Gunakan `regex` crate, case-insensitive, word-boundary.
- `src/aksara_lint/scorer.rs`: `fn score(text: &str) -> {kuat: Vec<Span>, sedang: Vec<Span>, density_per_500: f32}`. Ko-okurensi dalam 1 paragraf bobot x2.
- `src/aksara_lint/citations.rs`: validasi URL (reqwest HEAD), DOI resolve via `https://doi.org/`, ISBN-10/13 checksum, deteksi `utm_source`, `access-date` basi.
- `src/aksara_lint/suggest.rs`: aturan rewrite di atas sebagai `Diagnostic { span, reason_id, replacement }`. Jangan auto-replace agresif; tampilkan diff untuk persetujuan user.
- Uji: `cargo test` dengan contoh positif dari `docs/sign-of-ai.md` dan contoh negatif tulisan manusia pra-2022.

Jangan simpan kunci API / data pribadi di repo. Hormati semangat Merdeka: bebas pakai, salin, edit - cantumkan lisensi MIT.

## Yang BUKAN bukti (jangan flag)

- Gramatika sempurna saja; penulis profesional memang begitu.
- Campuran register kasual-formal / nada klinis-emosional (bisa jadi penulis teknis muda, neurodivergen, atau multi-editor).
- Prosa `bland/robotic` atau `fancy/akademik` secara umum - yang khas AI adalah kata spesifik di atas, bukan formalitas umum.
- Satu transition word (`Additionally`, `Consequently`, `Notably`) berdiri sendiri; manusia esai juga pakai.
- Konten tak bersumber saja (570rb+ artikel pra-LLM juga begitu) atau wikitext aneh acak (bisa bug VisualEditor / extension browser). Wikitext benar pun normal bila pakai preview.
- Perbedaan varietas English (US vs UK) ringan pada non-native - hanya curiga bila pergeseran dramatis tak terjelaskan.
