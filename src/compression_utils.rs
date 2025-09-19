use std::io::Write;

use flate2::{write, Compression};

pub fn gz_encoding(input: &Vec<u8>) -> Vec<u8> {
    let mut encoder = write::GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input).unwrap();
    encoder.finish().unwrap()
}
