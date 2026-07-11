#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
extern crate hcompress;

#[derive(Clone, Debug, Arbitrary)]
struct Data16 {
    d: Vec<i16>,
}

fuzz_target!(|data: Data16| {
    let d = data.d.iter().map(|&x| x as i32).collect::<Vec<i32>>();
    let mut input: Vec<i32> = d.clone();

    let y = 1;
    let x = input.len();
    let scale = 0; //lossless

    if x < 10 {
        return;
    }

    let mut compressed: Vec<u8> = Vec::with_capacity(x * y * 100);
    let mut encoder = hcompress::write::HCEncoder::new(&mut compressed);
    let _ = encoder.write(&mut input, y, x, scale);

    let mut uncompressed: Vec<i32> = vec![0; x * y];
    let mut decoder = hcompress::read::HCDecoder::new();
    let _ = decoder.read(&compressed, 0, &mut uncompressed);

    assert_eq!(d, uncompressed);
});
