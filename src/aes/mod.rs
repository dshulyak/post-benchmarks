use std::fs::File;
use std::hint::black_box;
use std::io::{BufReader, Read};
use std::thread::scope;

use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::{Aes128, Aes256};

pub fn work_many(workers: usize, size: u64) {
    scope(|s| {
        for _ in 0..workers {
            s.spawn(|| work_parallel(size/workers as u64));
        }
    })
}

pub fn work_parallel(size: u64) {
    let file = File::open("/dev/zero").expect("Could not open file");
    let mut reader = BufReader::with_capacity(1024 * 1024, file);

    const AES_PARALLELISM: usize = 16;
    let challenge = GenericArray::from_slice(b"hello world, challenge me!!!!!!!");
    let mut labels = [GenericArray::from([0u8; 16]); AES_PARALLELISM];
    let mut blocks = [GenericArray::from([0u8; 16]); AES_PARALLELISM];

    // Create AES ciphers for each nonce
    let key_cipher = Aes256::new(challenge);
    let ciphers = (0..6 as u8)
        .map(|i| {
            let mut key = GenericArray::from([0u8; 16]);
            key[0] = i;
            key_cipher.encrypt_block(&mut key);
            Aes128::new(&key)
        })
        .collect::<Vec<Aes128>>();

    for _ in 0..(size / (16 * AES_PARALLELISM as u64)) {
        for label_block in &mut labels {
            reader.read_exact(label_block).unwrap();
        }
        for cipher in &ciphers {
            for label_block in &mut labels {
                cipher.encrypt_block(label_block);
            }
        }
    }
}

#[cfg(test)]
use itertools::enumerate;
#[test]
fn test_aes() {
    const AES_PARALLELISM: usize = 16;

    let mut labels = [GenericArray::from([0u8; 16]); AES_PARALLELISM];
    let mut blocks = [GenericArray::from([0u8; 16]); AES_PARALLELISM];

    let cipher = Aes128::new(&GenericArray::from([0u8; 16]));

    for i in 0..16 as u8 {
        labels[i as usize][0] = i;
    }
    cipher.encrypt_blocks_b2b(&labels, &mut blocks).unwrap();

    for (i, out) in enumerate(blocks) {
        let mut out_single = GenericArray::from([0; 16]);
        cipher.encrypt_block_b2b(&labels[i], &mut out_single);
        assert_eq!(out, out_single);
    }
}
