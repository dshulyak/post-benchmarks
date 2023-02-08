use std::{
    fs::File,
    hint::black_box,
    io::{BufReader, Read},
};

pub fn work(size: u64) {
    const B: usize = 16;
    let challenge = b"hello world, challenge me!!!!!!!";

    let mut hasher = blake3::Hasher::new();

    let mut out = [0u8; 640];

    let file = File::open("/dev/zero").expect("Could not open file");
    let mut reader = BufReader::with_capacity(16 * 1024 * 1024, file);
    let mut buffer = [0u8; B];

    for _ in 0..(size / 16) {
        reader.read_exact(&mut buffer[..]).unwrap();
        hasher.reset();
        hasher.update(challenge);
        hasher.update(&buffer);
        let mut out_reader = hasher.finalize_xof();
        out_reader.fill(black_box(&mut out));
    }
}
