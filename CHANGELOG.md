# Changelog

## 0.4.0 — 2026-10-07

### Added

- The `openai.gpt-transcribe` transcription model enum variant.

## 0.3.0 — 2026-10-04

### Removed

- The `togetherai.thinkingmachines-inkling-small` model enum variant. The
  `togetherai.thinkingmachines-inkling` model remains supported. This is a
  breaking API change.

## 0.2.0 — 2026-10-04

### Added

- Transcription providers `smallestai` and `soniox`, and models `smallestai.pulse`,
  `smallestai.pulse-pro`, and `soniox.stt-async-v5`.

### Removed

- Amazon Transcribe and Google Cloud Speech-to-Text provider and model enum
  variants. Gemini remains supported. This is a breaking API change.

### Changed

- Regeneration now formats generated Rust files for consistent output.
