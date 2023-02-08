use siphasher::sip::SipHasher;
use std::hash::Hasher;

pub fn work(size: u64) -> u64 {
    let mut sip = SipHasher::new();
    let challenge = b"hello world, challenge me!!!!!!!";
    let nonce = 78;

    sip.write(challenge);
    sip.write_u16(nonce);
    let key0 = sip.finish();

    let mut v = 0;
    for i in 0..size {
        let mut sip = SipHasher::new_with_keys(key0, i);
        sip.write_u8(v as u8);
        v = sip.finish();
    }
    v
}
