# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable no-duplicate-heading -->

## [Unreleased]

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

[1.0.1]: https://github.com/jdh8/core-math/releases/tag/1.0.1
[1.0.0]: https://github.com/jdh8/core-math/releases/tag/1.0.0
[0.2.0]: https://github.com/jdh8/core-math/releases/tag/0.2.0
