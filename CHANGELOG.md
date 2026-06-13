# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable no-duplicate-heading -->

## [Unreleased]

## [1.1.1] - 2026-06-13

### Added

- New correctly rounded functions: `compoundf` for `f32`, and `lgamma`,
  `tgamma`, and `sincos` for `f64`.
- Opt-in `f16` feature exposing 43 binary16 functions (`acosf16` …
  `tgammaf16`, including `sqrtf16` and `sincosf16`).  Requires nightly Rust
  for the unstable `f16` primitive, and GCC 12+ or Clang 17+.
- Opt-in `f128` feature exposing 9 binary128 functions (`cbrtq`, `expq`,
  `exp10q`, `exp2q`, `expm1q`, `hypotq`, `logq`, `rsqrtq`, `sqrtq`).
  Requires nightly Rust for the unstable `f128` primitive, and Clang 15+ or
  GCC 14+.

### Changed

- Move to Rust edition 2024 and declare `rust-version = "1.85"`, matching
  `core-math-sys`.  The default build keeps working on stable Rust.

### Dependencies

- Bump `core-math-sys` to 1.1.1, which provides the new functions and the
  feature-gated `f16`/`f128` bindings.

### Tooling

- Benchmarks for the new `f32`/`f64` functions.
- Reorder the `bench!` macro arms so that the `_` shorthand keeps working in
  edition 2024, where `expr` fragments also match `_`.
- CI now also builds with the MSRV and tests the `f16`/`f128` features on
  nightly.

## [1.0.2] - 2026-06-09

### Dependencies

- Bump `core-math-sys` to 1.0.2, which now tracks the official CORE-MATH
  release instead of a vendored fork. Brings the latest upstream accuracy
  and correctness fixes to every wrapped function.

### Tooling

- Flatten the shared benchmark helper from `benches/bench/mod.rs` to
  `benches/bench.rs` and pin explicit benchmark targets with
  `autobenches = false`. No effect on the published library API.

## [1.0.1] - 2026-05-23

### Dependencies

- Bump `core-math-sys` to 1.0.1.

### Tooling

- Add a Criterion benchmark suite comparing every wrapped function against
  `std` and `libm`.

## [1.0.0] - 2025-08-03

### Changed

- Promote the crate to a stable 1.0 release now that the companion
  `core-math-sys` pins a vendored fork of CORE-MATH, guaranteeing the FFI
  surface stays compatible across upstream changes.

## [0.2.0] - 2024-08-27

### Added

- Initial Rusty wrappers over `core-math-sys`, exposing CORE-MATH's
  correctly rounded `f32` and `f64` math functions.
- Linkage sanity test to catch missing symbols at build time.

### Fixed

- Output type of `sincosf`.

### Tooling

- GitHub Actions workflow `rust.yml` for CI.

[1.1.1]: https://github.com/jdh8/core-math/releases/tag/1.1.1
[1.0.2]: https://github.com/jdh8/core-math/releases/tag/1.0.2
[1.0.1]: https://github.com/jdh8/core-math/releases/tag/1.0.1
[1.0.0]: https://github.com/jdh8/core-math/releases/tag/1.0.0
[0.2.0]: https://github.com/jdh8/core-math/releases/tag/0.2.0
