use std::collections::HashMap;

pub struct Decoder {
    table: Vec<[u16; 2]>,
}

impl Decoder {
    pub fn from_table(table: Vec<[u16; 2]>) -> Self {
        Decoder { table }
    }

    pub fn decode(&self, codes: &[u16]) -> Vec<u8> {
        let mut result = Vec::new();
        for &code in codes {
            let mut codes = vec![code];
            while !codes.is_empty() {
                let code = codes.pop().unwrap();
                if code <= 0xff {
                    result.push(code as u8);
                } else {
                    let [a, b] = self.table[(code - 0x100) as usize];
                    codes.extend(&[b, a]);
                }
            }
        }
        result
    }

    pub fn as_encoder(&self) -> Encoder {
        let table = self
            .table
            .iter()
            .enumerate()
            .map(|(i, &[a, b])| ((a as u32) << 16 | b as u32, i as u16 + 0x100))
            .collect();
        Encoder { table }
    }

    pub fn table(&self) -> &Vec<[u16; 2]> {
        &self.table
    }
}

pub struct Encoder {
    table: HashMap<u32, u16>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            table: HashMap::new(),
        }
    }

    pub fn build<T: ToString, I: Iterator<Item = T>>(&mut self, texts: I, vocab_size: usize) {
        let mut texts: Vec<_> = texts
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .map(Text::String)
            .collect();

        // let mut new_pairs = HashMap::new();

        loop {
            let mut counter = Counter::default();
            for text in &mut texts {
                match text {
                    Text::String(s) => {
                        let encoded = self.encode(s);
                        counter.add_codes(&encoded);
                        if s.bytes().count() != encoded.len() {
                            *text = Text::Pairs(encoded); // TODO
                        }
                    }
                    Text::Pairs(pairs) => {
                        self.encode_ex(pairs);
                        counter.add_codes(&pairs);
                    }
                }
            }

            let mut pairs: Vec<(u16, u32)> = counter.finish();

            let offset = 2u16.pow(8) + self.table.len() as u16;

            pairs.truncate(vocab_size.saturating_sub(offset as usize));

            if pairs.is_empty() {
                break;
            }

            // new_pairs = pairs
            //     .iter()
            //     .enumerate()
            //     .map(|(i, (_, pair))| (*pair, offset + i as u16))
            //     .collect();
            self.table.extend(
                pairs
                    .iter()
                    .enumerate()
                    .map(|(i, (_, pair))| (*pair, offset + i as u16)),
            );
        }
    }

    pub fn encode(&self, str: &str) -> Vec<u16> {
        let mut result = str.bytes().map(|c| c as u16).collect::<Vec<_>>();
        self.encode_ex(&mut result);
        result
    }

    fn encode_ex(&self, codes: &mut Vec<u16>) {
        let mut i = 0;
        while i + 1 < codes.len() {
            if let Some(nc) = self
                .table
                .get(&((codes[i] as u32) << 16 | codes[i + 1] as u32))
            {
                codes[i] = *nc;
                codes.remove(i + 1);
                i = i.saturating_sub(1);
            } else {
                i += 1;
            }
        }
    }

    pub fn as_decoder(&self) -> Decoder {
        let mut table = vec![[0; 2]; self.table.len()];
        for (&k, &v) in self.table.iter() {
            table[v as usize - 0x100] = [(k >> 16) as u16, k as u16];
        }
        Decoder { table }
    }

    pub fn hash_map(&self) -> &HashMap<u32, u16> {
        &self.table
    }
}

enum Text {
    String(String),
    Pairs(Vec<u16>),
}

#[derive(Default)]
struct Counter(HashMap<u32, u16>);

impl Counter {
    fn add(&mut self, pair: u32) {
        *self.0.entry(pair).or_insert(0) += 1;
    }

    fn add_codes(&mut self, codes: &[u16]) {
        for i in 0..codes.len() - 1 {
            self.add((codes[i] as u32) << 16 | codes[i + 1] as u32);
        }
    }

    fn finish(self) -> Vec<(u16, u32)> {
        let mut pairs: Vec<(u16, u32)> = self
            .0
            .into_iter()
            .filter(|(_, v)| 1 < *v)
            .map(|(k, v)| (v, k))
            .collect();
        pairs.sort_unstable();
        pairs.reverse();

        let mut closed = Vec::new();
        for (i, (count, pair)) in pairs.iter().copied().enumerate() {
            let a = (pair >> 16) as u16;
            let b = pair as u16;
            if closed.contains(&a) || closed.contains(&b) {
                let mut new_pairs = Vec::new();
                for (count_, pair) in pairs.iter().skip(i + 1).copied() {
                    if count_ < count {
                        break;
                    }
                    let a = (pair >> 16) as u16;
                    let b = pair as u16;
                    if !closed.contains(&a) && !closed.contains(&b) {
                        new_pairs.push((count_, pair));
                        closed.push(a);
                        closed.push(b);
                    }
                }
                pairs.truncate(i);
                pairs.extend(new_pairs);
                break;
            }
            closed.push(a);
            closed.push(b);
        }
        pairs
    }
}
