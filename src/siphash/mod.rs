use siphasher::sip::SipHasher;
use std::{hash::Hasher, hint::black_box};

pub fn work(size: u64) {
    let mut sip = SipHasher::new();
    let challenge = b"hello world, challenge me!!!!!!!";
    let nonce = 78;

    sip.write(challenge);
    sip.write_u16(nonce);
    let key0 = sip.finish();

    for i in 0..size {
        let mut sip = SipHasher::new_with_keys(key0, i);
        sip.write_u8(92);
        let v = sip.finish();
        black_box(v);
    }
}
