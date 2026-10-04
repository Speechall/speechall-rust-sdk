# Changelog

## 0.2.0 — 2026-10-04

### Added

- Transcription providers `smallestai` and `soniox`, and models `smallestai.pulse`,
  `smallestai.pulse-pro`, and `soniox.stt-async-v5`.

### Removed

- Amazon Transcribe and Google Cloud Speech-to-Text provider and model enum
  variants. Gemini remains supported. This is a breaking API change.

### Changed

- Regeneration now formats generated Rust files for consistent output.
