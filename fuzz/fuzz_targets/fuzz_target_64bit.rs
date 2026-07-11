#![no_main]
use arbitrary::Arbitrary;
use bytemuck::cast_slice_mut;
use libfuzzer_sys::fuzz_target;
extern crate hcompress;

#[derive(Clone, Debug, Arbitrary)]
struct Data32 {
    d: Vec<i32>,
}

fuzz_target!(|data: Data32| {
    let mut input: Vec<i64> = data.d.iter().map(|&x| x as i64).collect::<Vec<i64>>();

    let y = 1;
    let x = input.len();
    let scale = 0; //lossless

    if x < 10 {
        return;
    }

    let mut compressed: Vec<u8> = Vec::with_capacity(x * y * 100);
    let mut encoder = hcompress::write::HCEncoder::new(&mut compressed);
    let _ = encoder.write64(&mut input, y, x, scale);

    let mut uncompressed: Vec<i32> = vec![0; x * y * 2];
    let mut decoder = hcompress::read::HCDecoder::new();
    let _ = decoder.read64(&compressed, 0, cast_slice_mut(&mut uncompressed));

    assert_eq!(data.d, uncompressed[..(x * y)]);
});
