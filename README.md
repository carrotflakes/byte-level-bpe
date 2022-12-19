# byte-level-bpe

byte-level-bpe provides encoders and decoders based on Byte-level Byte Pair Encoding.

## Usage

``` rust
use byte_level_bpe::{Decoder, Encoder};

// Build vocabulary
let mut encoder = Encoder::new();
let vocab_size = 2usize.pow(10); // desired vocabulary size
encoder.build(["abc", "abcd", "abce"].iter(), vocab_size);

// Encode
let buf: Vec<u16> = encoder.encode("abc")

// Decode
let decoder = encoder.to_decoder();
let text = decoder.decode(&buf);

// Save vocabulary to a file
{
    use std::io::Write;
    let vocab: &Vec<[u16; 2]> = decoder.table();
    let file = std::fs::File::create("vocab").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for x in vocab {
        writer.write(&x[0].to_le_bytes()).unwrap();
        writer.write(&x[1].to_le_bytes()).unwrap();
    }
}

// Load vocabulary
let decoder = {
    use std::io::Read;
    let file = std::fs::File::open("vocab").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut table = Vec::new();
    let mut buf = vec![0u8; 4];
    while let Ok(()) = reader.read_exact(&mut buf) {
        table.push([u16::from_le_bytes(buf[..2].try_into().unwrap()), u16::from_le_bytes(buf[2..].try_into().unwrap())]);
    }
    Decoder::from_table(table)
};
```

## Author

* carrotflakes (carrotflakes@gmail.com)

## Copyright

Copyright (c) 2022 carrotflakes (carrotflakes@gmail.com)

## License

Licensed under the MIT License.
