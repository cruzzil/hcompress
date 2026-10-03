#![no_main]
//! Feed arbitrary bytes to the decoder: it must return, never panic.
//!
//! The first byte picks the smoothing flag and the output size; the rest is
//! the compressed stream.  Run with overflow checks on (`cargo fuzz` builds
//! with debug assertions by default) so arithmetic overflow is caught too.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some((&control, stream)) = data.split_first() else {
        return;
    };
    let smooth = i32::from(control & 1);
    let len = 1 + usize::from(control >> 1) * 8;

    let _ = hcompress::read::HCDecoder::new().read(stream, smooth, &mut vec![0; len]);
    let _ = hcompress::read::HCDecoder::new().read64(stream, smooth, &mut vec![0; len]);
});
