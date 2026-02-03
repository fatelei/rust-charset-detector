# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Dev commands (Rust and Python)

- Rust (pure library, no Python bindings)
  - Build: `cargo build --no-default-features`
  - Test: `cargo test --no-default-features -q`
  - Lint: `cargo clippy --no-default-features -- -D warnings`
  - Format: `cargo fmt` (check: `cargo fmt -- --check`)
  - Benchmarks (as referenced in README): `cargo bench`

- Python extension (build and test via maturin)
  - Build local dev extension: `pip install maturin[patchelf] && maturin develop`
  - Run tests: `pytest tests/ -v`
  - Build wheels (local): `maturin build --release --strip --out dist/`

- Running a single test
  - Rust: `cargo test --no-default-features test_detect_utf8 -q`
  - Python: `pytest tests/test_charset_detector.py::TestDetectFunction::test_detect_gbk -q`

Notes
- Default cargo feature set enables the Python bindings (`[features] default = ["python"]`). Use `--no-default-features` for Rust-only builds/tests (matches CI).
- CI also runs formatting (`cargo fmt -- --check`) and linting (`cargo clippy ...`).

## High-level architecture

This project is a Rust charset detection library with optional Python bindings, packaged as a Python wheel via maturin. The core detection is Rust-first; Python is a thin wrapper exposing the same capabilities.

- Public crate surface (`src/lib.rs`)
  - Re-exports core types and functions for ergonomic use: `detect`, `detect_best`, `detect_encoding` and data types from `models`. The `python` module is compiled only when the `python` feature is enabled.

- Core data types (`src/models.rs`)
  - Encoding enum (Unicode, CJK families, ISO-8859, Windows code pages, etc.).
  - DetectionResult/DetectionResults with confidence, optional language, and DetectionMethod (e.g., Bom, Utf8Validation, FrequencyAnalysis, HardRule, Fallback). Results are sorted by confidence and provide helpers like `best()`/`confident()`.

- Byte ranges and helpers (`src/ranges.rs`)
  - Centralized definitions of valid byte ranges per encoding family (UTF-8 continuation bytes; GBK/GB18030, CP949/EUC-KR, Big5, Shift_JIS; ISO-2022 escape sequences). Utility predicates (e.g., `in_ranges`, `is_utf8_continuation`) are reused by scorers.

- Scorers (`src/scorers/`)
  - `utf8.rs`: fast UTF-8 validation with confidence scoring (BOM-aware, overlong/invalid sequence checks).
  - `cjk.rs`: CJK detection heuristics and discrimination, including:
    - GBK vs CP949 discrimination: hard rule (trail byte 0x40 valid only in GBK) plus frequency-based scoring for Chinese/Korean.
    - Additional detectors: Big5 and Shift_JIS with characteristic byte pattern checks.
  - `mod.rs` re-exports scorer functions for consumption by the detector.

- Orchestration (`src/detector.rs`)
  - `CharsetDetector` coordinates a staged pipeline:
    1) BOM detection (UTF-8/16/32) → early high-confidence return.
    2) UTF-8 validation (fast path; high confidence may short-circuit).
    3) ISO-2022 escape-sequence sniffing (JP/KR candidates).
    4) CJK family detection: GBK/CP949 discrimination, Big5, Shift_JIS.
    5) Confidence filtering: return only confident candidates if any; otherwise include reasonable fallbacks (e.g., UTF-8, ISO-8859-1).
  - Convenience fns: `detect`, `detect_best`, `detect_encoding` delegate to a default `CharsetDetector`.

- Python bindings (`src/python.rs` + `python/charset_detector/__init__.py`)
  - PyO3 module `_detector` exposes:
    - Classes: `CharsetDetector`, `CharsetMatch`, `CharsetMatches`.
    - Functions: `detect_from_bytes`, `detect_best_from_bytes`.
  - Pure-Python `charset_detector` package re-exports a friendly API:
    - Top-level fns: `detect(bytes, min_confidence=0.3)`, `detect_best(bytes, min_confidence=0.3)`.
    - Class wrapper `CharsetDetector(min_confidence=0.3)` with `detect`, `detect_best`, `detect_encoding` methods.

## What CI does (useful parity for local dev)
- Rust: `cargo test --no-default-features`, `cargo clippy --no-default-features -- -D warnings`, and `cargo fmt -- --check`.
- Python wheels: `maturin build --release --strip` across CPython versions and OSes; then `pytest tests/ -v` on the produced wheels.

## Pointers from README
- End-user install: `pip install charset-detector`.
- Quick start examples exist for both Python and Rust; API mirrors the functions/classes exposed above.
