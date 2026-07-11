# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-07-11

### Changed
- Migrated the crate to the Rust 2024 edition; minimum supported Rust version is now 1.87.
- Removed the optional `arbitrary` dependency from the main crate. The `Arbitrary` fuzzing
  wrappers (`Data16` / `Data32`) now live in the fuzz targets, so the library no longer pulls in
  `arbitrary` for its own build.
- Added the missing package metadata (`authors`, `keywords`, `categories`, `homepage`,
  `documentation`, `readme`, `rust-version`) and a fuller description to `Cargo.toml`.

### Fixed
- Resolved all Clippy warnings and the `unused_io_amount` errors in `read.rs` / `write.rs`
  (dead stores, index-only loops, `write` → `write_all`, `is_multiple_of`).

### Documentation
- Expanded `README.md` with a description, feature list, install instructions, worked usage
  examples, and an API summary.
- Added `ALGORITHM.md` describing the H-transform, quantization, and quadtree coding stages, the
  compressed stream format, and the constraints on use.
- Added this `CHANGELOG.md`.

## [0.3.0] - 2025-05-03

### Changed
- The encoder now writes into any `std::io::Write` sink. `HCEncoder::new` takes the output writer
  and `write` / `write64` no longer take an output buffer argument. **(Breaking API change.)**

## [0.2.2] - 2025-04-08

### Fixed
- General cleanup and Clippy fixes.

## [0.2.1] - 2024-10-15

### Fixed
- Corrected the 64-bit (`write64` / `read64`) compression and decompression implementations, with
  additional failing test cases captured as regression tests.

## [0.2.0] - 2024-09-29

### Added
- 64-bit (`i64`) code path for 32-bit images.
- Property-based tests using `quickcheck`.
- Documentation comments across the codebase.

### Changed
- Refactored the monolithic implementation into separate `read` and `write` modules.

## [0.1.1] - 2022-03-05

### Added
- Repository metadata, upstream CFITSIO / HCompress licenses, and continuous integration.

## [0.1.0] - 2022-02-26

### Added
- Initial release: a pure-Rust port of the CFITSIO implementation of HCompress, supporting lossless
  and lossy compression of 2-D integer images.
