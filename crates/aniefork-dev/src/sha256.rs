//! SHA-256 (FIPS 180-4), for module fingerprints. Dependency-free and checked against the
//! standard's own test vectors. A fingerprint has to be collision-resistant: trust is given to
//! it, so two different modules must not share one.

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const INITIAL: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

/// A SHA-256 computed over data fed in pieces.
#[derive(Clone)]
pub struct Sha256 {
    state: [u32; 8],
    /// Bytes waiting for a full 64-byte block.
    block: [u8; 64],
    filled: usize,
    /// Bytes fed so far.
    length: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Sha256 { state: INITIAL, block: [0; 64], filled: 0, length: 0 }
    }
}

impl Sha256 {
    pub fn new() -> Self {
        Sha256::default()
    }

    pub fn update(&mut self, mut data: &[u8]) {
        self.length = self.length.wrapping_add(data.len() as u64);
        if self.filled > 0 {
            let take = (64 - self.filled).min(data.len());
            let (head, rest) = data.split_at(take);
            if let Some(slot) = self.block.get_mut(self.filled..self.filled + take) {
                slot.copy_from_slice(head);
            }
            self.filled += take;
            data = rest;
            if self.filled < 64 {
                return;
            }
            let block = self.block;
            compress(&mut self.state, &block);
            self.filled = 0;
        }
        let (blocks, rest) = data.as_chunks::<64>();
        for block in blocks {
            compress(&mut self.state, block);
        }
        if let Some(slot) = self.block.get_mut(..rest.len()) {
            slot.copy_from_slice(rest);
        }
        self.filled = rest.len();
    }

    /// The digest: 32 bytes.
    pub fn finish(mut self) -> [u8; 32] {
        let bits = self.length.wrapping_mul(8);
        // A 1 bit, zeros up to 8 bytes short of a block, then the length in bits.
        let mut padding = [0u8; 64];
        padding[0] = 0x80;
        let pad = if self.filled < 56 { 56 - self.filled } else { 120 - self.filled };
        self.update(padding.get(..pad).unwrap_or(&[]));
        self.update(&bits.to_be_bytes());
        let mut out = [0u8; 32];
        for (word, slot) in self.state.iter().zip(out.as_chunks_mut::<4>().0) {
            *slot = word.to_be_bytes();
        }
        out
    }

    /// The digest as 64 lowercase hexadecimal digits.
    pub fn hex(self) -> String {
        self.finish().iter().map(|b| format!("{b:02x}")).collect()
    }
}

fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for (slot, bytes) in w.iter_mut().zip(block.as_chunks::<4>().0) {
        *slot = u32::from_be_bytes(*bytes);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for (k, word) in K.iter().zip(w.iter()) {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(*k).wrapping_add(*word);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (slot, v) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *slot = slot.wrapping_add(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(data: &[u8]) -> String {
        let mut h = Sha256::new();
        h.update(data);
        h.hex()
    }

    #[test]
    fn matches_the_standards_test_vectors() {
        assert_eq!(hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(
            hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
        assert_eq!(hex(&vec![b'a'; 1_000_000]), "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
    }

    #[test]
    fn feeding_in_pieces_gives_the_same_digest() {
        let data: Vec<u8> = (0..1000u32).map(|i| (i.wrapping_mul(31) % 251) as u8).collect();
        let whole = hex(&data);
        // Every way of cutting the data, including pieces that end exactly on a block.
        for piece in [1, 3, 55, 56, 63, 64, 65, 119, 120, 128, 999] {
            let mut h = Sha256::new();
            for chunk in data.chunks(piece) {
                h.update(chunk);
            }
            assert_eq!(h.hex(), whole, "pieces of {piece}");
        }
        // Lengths around the padding boundary.
        for n in [55, 56, 57, 63, 64, 65, 119, 120, 121] {
            let mut a = Sha256::new();
            a.update(&data[..n]);
            let mut b = Sha256::new();
            b.update(&data[..n / 2]);
            b.update(&[]);
            b.update(&data[n / 2..n]);
            assert_eq!(a.hex(), b.hex(), "length {n}");
        }
    }
}
