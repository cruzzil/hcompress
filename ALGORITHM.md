# The HCompress algorithm

This document describes how the HCompress algorithm works, as implemented by this crate, and the
constraints you need to respect when using it. It is a summary intended for users of the library;
the authoritative description is the original paper
([tilecompression2.3.pdf](https://fits.gsfc.nasa.gov/registry/tilecompression/tilecompression2.3.pdf),
also bundled as [`docs/paper.pdf`](docs/paper.pdf)) and the CFITSIO source from which this port is
derived.

## Overview

HCompress compresses a two-dimensional array of integers in three stages. Decompression runs the
same three stages in reverse:

```
 Compression:   H-transform  ->  Quantize (digitize)  ->  Quadtree bit-plane coding  ->  bytes
 Decompression: bytes  ->  Quadtree decoding  ->  De-quantize (undigitize)  ->  Inverse H-transform
```

### 1. The H-transform

The H-transform is a two-dimensional, integer, Haar-like wavelet transform. The image is repeatedly
divided into 2×2 blocks. For each block

```
  a  b
  c  d
```

four coefficients are computed:

- `h0 = a + b + c + d`  — the sum (a low-pass / average term),
- `hx = a - b + c - d`  — the horizontal difference,
- `hy = a + b - c - d`  — the vertical difference,
- `hc = a - b - c + d`  — the diagonal ("curvature") difference.

The `h0` low-pass terms form a half-size image, and the transform is applied to it again. After
`log2(max(nx, ny))` passes only a single `h0` value — the sum of all pixels — remains, together with
a pyramid of difference coefficients. The transform is exactly reversible (the inverse is `hinv`).

Because the sums grow as the transform proceeds, the coefficients need more bits than the input
pixels. This is why the crate provides two code paths (see **Constraints** below).

### 2. Quantization (digitize)

Each transform coefficient is divided by the `scale` factor, rounding towards the nearest multiple
of `scale`. This is the *only* lossy step:

- `scale = 0` or `scale = 1` → no quantization → **lossless**.
- `scale > 1` → coefficients are coarsened → **lossy**, with higher `scale` giving smaller output.

A useful rule of thumb from the literature is to set `scale` proportional to the RMS noise of the
image (e.g. `scale ≈ 2 × sigma`), which discards mostly noise while keeping the signal. The best
value is image-dependent.

### 3. Quadtree bit-plane coding

The quantized coefficients are entropy-coded one bit plane at a time, from the most significant bit
down. For each bit plane the array is coded hierarchically as a quadtree: at each level a few bits
record which quadrants contain any set bits, so large empty regions collapse to almost nothing. The
lowest level uses a fixed Huffman-style code for the 4-bit nibbles. Sign bits are stored separately,
and the three quadrants (produced by the transform) are coded independently.

## Compressed stream format

Every stream produced by this crate begins with the two-byte magic code `0xDD 0x99`, followed by the
image dimensions, the scale factor, the sum of all pixels, the number of bit planes per quadrant,
and finally the quadtree-coded bit planes. The decoder validates the magic code and the dimensions
before allocating anything.

## Constraints on use

Read these before relying on the library.

### Input must be 2-D integer data

- HCompress operates on integer arrays only. Floating-point images must be scaled to integers first.
- The data is treated as a 2-D image of `nx × ny` elements. One-dimensional data is handled by
  setting `nx = 1` (or `ny = 1`).
- **`ny` is the fastest-varying dimension.** This is reversed from the usual FITS convention, where
  `ny` would be the X (display) axis. The element at image row `i`, column `j` lives at index
  `i * ny + j`.

### Pick the right bit width

Because the H-transform accumulates sums, the working integers must be wider than the input:

| Function pair          | Input values          | Working type | Use when |
|------------------------|-----------------------|--------------|----------|
| `write` / `read`       | up to ~16-bit integers, stored in `i32` | `i32` | Typical 16-bit astronomical images. |
| `write64` / `read64`   | up to ~32-bit integers, stored in `i64` | `i64` | Values that would overflow the 32-bit transform. |

Matching the original implementation: the non-`64` functions expect *shorts padded to int*, and the
`64` functions expect *ints padded to long*. Feeding values that are too large for the chosen path
can cause the transform to overflow; the encoder reports this as an error
(`EncodeError::DataCompressionError`) rather than producing a corrupt stream.

### Buffer ownership and sizing

- **The encoder mutates its input in place.** `write`/`write64` perform the H-transform and
  quantization directly on the array you pass in. Clone the data first if you still need the
  original.
- **The decoder needs a pre-allocated output buffer** of at least `nx * ny` elements. If it is too
  small the decoder returns `DecodeError::IncorrectAllocationSize`.
- For `read64`, the working buffer is an `i64` slice, but the reconstructed values are 32-bit. The
  decoder packs the `nx * ny` result values back into the low half of the buffer as `i32`s (use
  `bytemuck::cast_slice_mut` to view them), so the buffer must be allocated with `2 × nx × ny`
  `i32`s of space, as shown in the fuzz targets.

### Lossy decompression and smoothing

- When `scale > 1` was used, decompression cannot recover the exact original image.
- The decoder's `smooth` parameter (0 = off) applies smoothing during the inverse transform to
  reduce the blocky artifacts that lossy quantization can introduce. It has no effect on losslessly
  compressed data.

### Error conditions

Decoding returns a `DecodeError` for malformed or mismatched input:

- `BadFileFormat` — the magic code is missing/incorrect.
- `NumericalOverflow` — the declared dimensions overflow when multiplied.
- `IncorrectAllocationSize` — the output buffer is smaller than `nx * ny`.
- `BadFormatCode` — an unexpected code was found in the quadtree stream.
- `BadBitPlaneValues` / `MemoryAllocationError` — corrupt bit-plane data or allocation failure.

### What HCompress is *not* good for

- Non-integer data, 3-D data, or non-image byte streams.
- Incompressible data (pure noise, already-compressed data): the quadtree coder cannot find
  structure to exploit, so the output can be as large as, or larger than, the input.
