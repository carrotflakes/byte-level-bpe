use blbpe::Encoder;

fn main() {
    let mut encoder = Encoder::new();
    encoder.build(["abc", "abcd", "abce"].iter(), 300);
    dbg!(encoder.encode("abc"));

    let mut encoder = Encoder::new();
    encoder.build(["ABCDCDABCDCDE"].iter(), 300);
    dbg!(encoder.hash_map());
    let encoded = dbg!(encoder.encode("ABCDCDABCDCDE"));
    let decoder = encoder.as_decoder();
    dbg!(String::from_utf8(decoder.decode(&encoded)));

    let mut encoder = Encoder::new();
    encoder.build([
        "Byte pair encoding or digram coding is a simple form of data compression in which the most common pair of consecutive bytes of data is replaced with a byte that does not occur within that data.",
        "A table of the replacements is required to rebuild the original data.",
        "これは日本語の文章です。",
    ].iter(), 1000);
    dbg!(encoder.hash_map().len());
    let decoder = encoder.as_decoder();
    let encoded = dbg!(encoder.encode("Byte pair encoding is good!"));
    dbg!(String::from_utf8(decoder.decode(&encoded)));

    let encoded = dbg!(encoder.encode("日本語もいけるよ"));
    dbg!(String::from_utf8(decoder.decode(&encoded)));

    assert_eq!(encoder.hash_map(), decoder.as_encoder().hash_map());
}
