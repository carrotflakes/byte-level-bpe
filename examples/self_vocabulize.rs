use std::io::Read;

use byte_level_bpe::Encoder;

fn main() {
    let mut encoder = Encoder::new();
    let vocab_size = 500;

    let mut buf = Vec::new();
    std::fs::File::open("./src/lib.rs")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    encoder.build_from_bytes([buf].into_iter(), vocab_size);

    println!("learned vocab: {}", encoder.hash_map().len());

    // Encode
    let encoded = encoder.encode("hello, byte-pair-encoding!");
    println!("encoded: {:?}", encoded);

    // Decode
    let decoder = encoder.to_decoder();
    let bytes: Vec<u8> = decoder.decode(&encoded);
    let text = String::from_utf8(bytes).unwrap();
    println!("decoded: {:?}", text);
}
