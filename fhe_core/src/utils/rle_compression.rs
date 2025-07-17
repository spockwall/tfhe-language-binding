use std::fmt;

pub struct RunLengthEncoded {
    pub value: u8,
    pub count: u32,
}

// Implementing the Display trait
impl fmt::Display for RunLengthEncoded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.value, self.count)
    }
}

// Implementing the Debug trait (explicitly, although derive can be used as well)
impl fmt::Debug for RunLengthEncoded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.value, self.count)
    }
}
pub fn rle_compress(data: &[u8]) -> Vec<RunLengthEncoded> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut compressed = Vec::new();
    let mut count = 1;
    let mut prev = data[0];

    for &byte in &data[1..] {
        if byte == prev {
            count += 1;
        } else {
            compressed.push(RunLengthEncoded { value: prev, count });
            prev = byte;
            count = 1;
        }
    }

    compressed.push(RunLengthEncoded { value: prev, count });
    compressed
}

pub fn rle_decompress(compressed: &[RunLengthEncoded]) -> Vec<u8> {
    let mut decompressed = Vec::new();

    for &RunLengthEncoded { value, count } in compressed {
        for _ in 0..count {
            decompressed.push(value);
        }
    }

    decompressed
}

pub fn rle_to_bytes(rle: &Vec<RunLengthEncoded>) -> Vec<u8> {
    rle.iter()
        .map(|x| vec![x.value; x.count as usize])
        .flatten()
        .collect()
}

pub fn bytes_to_rle(bytes: &Vec<u8>) -> Vec<RunLengthEncoded> {
    bytes
        .iter()
        .map(|x| RunLengthEncoded {
            value: *x,
            count: 1,
        })
        .collect()
}
