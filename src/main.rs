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
}
