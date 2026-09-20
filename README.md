# hcompress

[![Crates.io](https://img.shields.io/crates/v/hcompress.svg)](https://crates.io/crates/hcompress)
[![Actions Status](https://github.com/cruzzil/hcompress/workflows/CI/badge.svg)](https://github.com/cruzzil/hcompress/actions)
[![Documentation](https://docs.rs/hcompress/badge.svg)](https://docs.rs/hcompress/)
[![codecov](https://codecov.io/gh/cruzzil/hcompress/graph/badge.svg?token=YZEX06JT7K)](https://codecov.io/gh/cruzzil/hcompress)
[![Dependency status](https://deps.rs/repo/github/cruzzil/hcompress/status.svg)](https://deps.rs/repo/github/cruzzil/hcompress)

A pure-Rust port of the [CFITSIO](https://heasarc.gsfc.nasa.gov/fitsio/) implementation of the
**HCompress** image compression algorithm.

HCompress is an image compression scheme developed at the Space Telescope Science Institute and
widely used to compress astronomical images (for example, in FITS tile compression). It works
particularly well on the smoothly-varying, noisy two-dimensional integer images that are typical
in astronomy, and supports both lossless and lossy compression.

For a description of how the algorithm works, and the constraints on its use, see
[ALGORITHM.md](ALGORITHM.md).

## Features

- Lossless and lossy compression of 2-D integer images.
- 16-bit images stored in `i32` (`write` / `read`).
- 32-bit images stored in `i64` (`write64` / `read64`).
- Faithful to the reference CFITSIO/HCompress bitstream, including the two-byte magic
  code `0xDD 0x99`.
- No `unsafe`-heavy dependencies; pure safe Rust over `std::io::Write`.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
hcompress = "0.4"
```

## Usage

### 16-bit images (`i32`)

The encoder writes the compressed bytes into any `std::io::Write` sink. The decoder writes the
reconstructed image into a caller-provided buffer.

```rust
use hcompress::read::HCDecoder;
use hcompress::write::HCEncoder;

// A small 4 x 3 image (ny is the fastest-varying axis).
let nx = 4; // slow axis (rows)
let ny = 3; // fast axis (columns)
let original: Vec<i32> = vec![
    10, 11, 12,
    13, 14, 15,
    16, 17, 18,
    19, 20, 21,
];

// --- Compress ---
// NOTE: the encoder H-transforms the input in place, so pass a copy if you
// still need the original data afterwards.
let mut work = original.clone();
let mut compressed: Vec<u8> = Vec::new();
let scale = 0; // 0 = lossless; larger values give more (lossy) compression
let mut encoder = HCEncoder::new(&mut compressed);
encoder.write(&mut work, ny, nx, scale).unwrap();

// --- Decompress ---
let mut restored: Vec<i32> = vec![0; nx * ny];
let mut decoder = HCDecoder::new();
let smooth = 0; // apply smoothing on decode (only relevant for lossy data)
let (out_nx, out_ny, out_scale) = decoder.read(&compressed, smooth, &mut restored).unwrap();

assert_eq!((out_nx, out_ny, out_scale), (nx, ny, 0));
assert_eq!(restored, original); // lossless round-trip
```

### 32-bit images (`i64`)

Use `write64` / `read64` for images whose values exceed the 24-bit range that the 16-bit path can
represent losslessly. The working buffer passed to `read64` is an `i64` slice; the reconstructed
`i32` values are written back into the low half of that buffer (see [ALGORITHM.md](ALGORITHM.md)
for the exact layout).

## API summary

| Function | Input element | Description |
|----------|---------------|-------------|
| `HCEncoder::write(a, ny, nx, scale)`   | `i32` | Compress a 16-bit image (16-bit values padded to `i32`). |
| `HCEncoder::write64(a, ny, nx, scale)` | `i64` | Compress a 32-bit image (32-bit values padded to `i64`). |
| `HCDecoder::read(input, smooth, a)`    | `i32` | Decompress into a caller-allocated `i32` buffer. |
| `HCDecoder::read64(input, smooth, a)`  | `i64` | Decompress into a caller-allocated `i64` buffer. |

The `read`/`read64` functions return `(nx, ny, scale)` on success.

> **NOTE:** the `nx` and `ny` dimensions as used in this library are reversed from the usual FITS
> notation. `ny` is the fastest-varying dimension, which is usually considered the X axis in a FITS
> image display.
>
> **NOTE:** the non-`64` functions support shorts padded to int, and the `64` functions support int
> padded to long, matching the original implementation.

See [ALGORITHM.md](ALGORITHM.md) for the full set of constraints (minimum sizes, in-place mutation,
scale semantics, overflow behaviour).

## References

- Paper detailing the algorithm:
  <https://fits.gsfc.nasa.gov/registry/tilecompression/tilecompression2.3.pdf>
  (also included in [`docs/paper.pdf`](docs/paper.pdf)).
- Original library: <https://www.stsci.edu/software/hcompress.html>

## Changes

See [CHANGELOG.md](CHANGELOG.md).

## License

This crate is distributed under the [MIT License](LICENSE).

It is derived from the CFITSIO / HCompress sources; the upstream licenses are reproduced in the
[`licenses/`](licenses/) directory:

- [`licenses/CFITSIO_LICENSE`](licenses/CFITSIO_LICENSE)
- [`licenses/HCOMPRESS_LICENSE`](licenses/HCOMPRESS_LICENSE)
