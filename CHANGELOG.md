# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2026-10-03

### Fixed
- The decoder no longer panics on malformed or truncated input; it returns a `DecodeError`
  instead. A stream whose header gives a zero or negative dimension, or more bit planes than the
  output integer holds (32 for `read`, 64 for `read64`), is rejected, and so is a stream that ends
  early (`BadFileFormat`). Previously these hit a division by zero, an out-of-range shift, or a
  read past the end of the input.
- Arithmetic on coefficients read from the stream (the scale multiply, the inverse H-transform and
  smoothing) now wraps as CFITSIO's C `int` arithmetic does, rather than panicking with overflow
  checks on. Valid streams decode exactly as before.
- Lossy streams (`scale > 1`): the last pixel's coefficient was not multiplied by the scale, so
  the decoded image could differ from CFITSIO's. It now matches.
- Decoding with smoothing (`smooth != 0`) panicked, or read past the coefficients it should
  touch, whenever an expansion level was narrower than two pixels in either direction. It now
  matches CFITSIO.
- A 1 x 1 image decodes to its pixel value. The C original's inverse H-transform has undefined
  behaviour here (`1 << -1`); the port panicked in debug builds and returned 0 in release builds.
- For images one pixel wide, the decoder reads the (empty) fourth quadrant's bit planes as CFITSIO
  does, instead of skipping them.

### Changed
- Dropped the `bytes` dependency, which nothing uses any more.

### Added
- `fuzz_decode` cargo-fuzz target, which feeds arbitrary bytes to `read` and `read64`, and
  property tests that decode random and corrupted streams.

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
