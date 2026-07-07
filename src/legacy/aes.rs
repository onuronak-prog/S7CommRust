// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from bonk-dev/HarpoS7 `HarpoS7/Aes/{HarpoAesCtr,HarpoHash,HarpoAes}.cs`
// (MIT, (c) 2024 bonk).

//! The legacy blob's authenticated AES-CTR mode: a CTR keystream over standard AES-128-ECB,
//! plus a custom tabulation-hash running tag ("checksum"). Used to encrypt the challenge and
//! the random key material into the `SecurityKeyEncryptedKey` blob.

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::Aes128;

mod harpo_hash {
    //! The tabulation hash and its lookup-table generation (`HarpoHash`).

    use super::super::aes_consts::LUT_SEED;

    /// The hash seed = `LUT_SEED` viewed as little-endian `u16`s.
    fn hash_seed(index: usize) -> u32 {
        u16::from_le_bytes([LUT_SEED[2 * index], LUT_SEED[2 * index + 1]]) as u32
    }

    /// A `u16` read from `LUT_SEED` at a byte offset (C# `BitConverter.ToUInt16`).
    fn lut_seed_u16(byte_offset: usize) -> u32 {
        u16::from_le_bytes([LUT_SEED[byte_offset], LUT_SEED[byte_offset + 1]]) as u32
    }

    fn lut_seed_init(val1: u32, t1: u32) -> u32 {
        let m = (val1 >> 0x11) | (t1 << 0xF);
        ((m ^ (t1 >> 1)) & 0x7F7F7F7F) ^ m
    }

    /// `Lut1`: derive `output[0..4]` from `a1[0..4]`.
    fn lut1(output: &mut [u32], a1: &[u32]) {
        // `(a1[3] >> 17)` is at most 15 bits, so `& 0x80808080` is 0 or 0x80 -> index 0 or 256.
        let index = (((a1[3] >> 0x11) & 0x8080_8080) * 2) as usize;
        let t2 = lut_seed_u16(index);

        output[3] = lut_seed_init(a1[2], a1[3]);
        output[2] = lut_seed_init(a1[1], a1[2]);
        output[1] = lut_seed_init(a1[0], a1[1]);

        let t1 = a1[0];
        output[0] = ((((t1 << 0xF) ^ (t1 >> 1)) & 0x7F7F_7F7F) ^ (t1 << 0xF)) ^ t2;
    }

    /// `GenerateLookupTable`: build the 1024-word LUT from a 16-byte seed key.
    pub fn generate_lookup_table(key: &[u8; 16], dest: &mut [u32; 1024]) {
        dest.fill(0);
        for i in 0..4 {
            dest[0x200 + i] =
                u32::from_le_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
        }

        let mut index = 0x40usize;
        loop {
            // `output = dest[index*4..]`, `a1 = dest[index*8..]` (disjoint; copy a1 out first).
            let a1 = [
                dest[index * 8],
                dest[index * 8 + 1],
                dest[index * 8 + 2],
                dest[index * 8 + 3],
            ];
            let mut out = [0u32; 4];
            lut1(&mut out, &a1);
            dest[index * 4..index * 4 + 4].copy_from_slice(&out);
            index >>= 1;
            if index == 0 {
                break;
            }
        }

        let mut index = 2usize;
        while index < 0x100 {
            let j = index << 4;
            let mut k = index - 1;
            let mut dest1 = (0x18 + j) / 4;
            let mut dest2 = 0x18 / 4;
            let l = j / 4;
            loop {
                dest[dest1 - 2] = dest[dest2 - 2] ^ dest[l];
                dest[dest1 - 1] = dest[dest2 - 1] ^ dest[l + 1];
                dest[dest1] = dest[dest2] ^ dest[l + 2];
                dest[dest1 + 1] = dest[dest2 + 1] ^ dest[l + 3];
                dest1 += 4;
                dest2 += 4;
                k -= 1;
                if k == 0 {
                    break;
                }
            }
            index *= 2;
        }
    }

    /// `Hash`: mix the 16-byte `output` block through the LUT, in place.
    pub fn hash(output: &mut [u8; 16], lut: &[u32; 1024]) {
        let (mut t1, mut t2, mut t3, mut t4) = (0u32, 0u32, 0u32, 0u32);
        for i in (0..16).rev() {
            let v1 = t3 >> 0x18;
            let v2 = t2 >> 0x18;
            let v3 = t1 >> 0x18;
            let v4 = output[i] as u32;
            let t5 = ((v4 << 4) / 4) as usize;

            t1 = (t1 << 8) ^ hash_seed((t4 >> 0x18) as usize) ^ lut[t5];
            t2 = ((t2 << 8) | v3) ^ lut[t5 + 1];
            t3 = ((t3 << 8) | v2) ^ lut[t5 + 2];
            t4 = ((t4 << 8) | v1) ^ lut[t5 + 3];
        }
        output[0..4].copy_from_slice(&t1.to_le_bytes());
        output[4..8].copy_from_slice(&t2.to_le_bytes());
        output[8..12].copy_from_slice(&t3.to_le_bytes());
        output[12..16].copy_from_slice(&t4.to_le_bytes());
    }
}

/// Encrypt one block in place with AES-128-ECB.
fn ecb_encrypt(cipher: &Aes128, block: &mut [u8; 16]) {
    cipher.encrypt_block(GenericArray::from_mut_slice(block));
}

/// The authenticated AES-CTR state (`HarpoAesCtr`).
pub struct HarpoAesCtr {
    cipher: Aes128,
    lut: [u32; 1024],
    counter: [u8; 16],
    aes2: [u8; 16],
    aes3: [u8; 16],
    iv_extension: [u8; 16],
    var1: u32,
    var2: u32,
}

impl HarpoAesCtr {
    /// Create with a 16-byte AES key. Call [`HarpoAesCtr::init`] before encrypting.
    pub fn new(key: &[u8]) -> Self {
        let cipher = Aes128::new(GenericArray::from_slice(&key[..16]));
        HarpoAesCtr {
            cipher,
            lut: [0u32; 1024],
            counter: [0u8; 16],
            aes2: [0u8; 16],
            aes3: [0u8; 16],
            iv_extension: [0u8; 16],
            var1: 0,
            var2: 0,
        }
    }

    /// Initialise the keystream/tag from `iv` (length a non-zero multiple of 16).
    pub fn init(&mut self, iv: &[u8]) {
        self.iv_extension = [0u8; 16];

        // 1. AES-encrypt zeros -> the LUT seed.
        let mut seed = [0u8; 16];
        ecb_encrypt(&self.cipher, &mut seed);
        // 2. Build the LUT from the seed.
        harpo_hash::generate_lookup_table(&seed, &mut self.lut);

        // 3. XOR each 16-byte IV block into the extension and hash.
        for i in 0..iv.len() / 16 {
            for k in 0..16 {
                self.iv_extension[k] ^= iv[4 * i + k];
            }
            harpo_hash::hash(&mut self.iv_extension, &self.lut);
        }

        // 4. Fold the IV bit-length into the extension, then hash.
        let iv_bits = (iv.len() as u32) << 3;
        self.iv_extension[0xF] ^= iv_bits as u8;
        self.iv_extension[0xE] ^= (iv_bits >> 8) as u8;
        self.iv_extension[0xD] ^= (iv_bits >> 16) as u8;
        self.iv_extension[0xC] ^= (iv_bits >> 24) as u8;
        self.iv_extension[0xB] ^= ((iv.len() as u32) >> 29) as u8;
        harpo_hash::hash(&mut self.iv_extension, &self.lut);

        // 5. Seed the counter; reset the tag.
        self.counter = self.iv_extension;
        self.aes3 = [0u8; 16];
        self.var1 = 0;
        self.var2 = 0;
    }

    /// Increment the 3-byte big-endian counter at positions 0xF..0xD.
    fn increment_counter(&mut self) {
        let mut v2 = 0x10usize;
        loop {
            if v2 < 0xD {
                break;
            }
            v2 -= 1;
            self.counter[v2] = self.counter[v2].wrapping_add(1);
            if self.counter[v2] != 0 {
                break;
            }
        }
    }

    /// Encrypt `plaintext` into `dest` (CTR), updating the running tag.
    pub fn encrypt_ctr(&mut self, plaintext: &[u8], dest: &mut [u8]) {
        let mut v1 = (self.var2 & 0xF) as usize;
        if self.var2 == 0 && self.var1 != 0 && (self.var1 & 0xF) != 0 {
            harpo_hash::hash(&mut self.aes3, &self.lut);
        }

        let mut v4: usize = 0;
        if v1 != 0 {
            if !plaintext.is_empty() {
                loop {
                    if v1 > 0xF {
                        break;
                    }
                    let v3 = self.aes2[v1] ^ plaintext[v1];
                    dest[v1] = v3;
                    self.aes3[v1] ^= v3;
                    v1 += 1;
                    v4 += 1;
                    if v4 >= plaintext.len() {
                        break;
                    }
                }
            }
            if v1 == 0x10 {
                harpo_hash::hash(&mut self.aes3, &self.lut);
                v1 = 0;
            }
        }

        if v4 + 0x10 <= plaintext.len() {
            let mut out_index = v4;
            loop {
                self.increment_counter();
                self.aes2 = self.counter;
                ecb_encrypt(&self.cipher, &mut self.aes2);
                for j in 0..16 {
                    dest[out_index + j] = plaintext[v4 + j] ^ self.aes2[j];
                    self.aes3[j] ^= dest[out_index + j];
                }
                harpo_hash::hash(&mut self.aes3, &self.lut);
                out_index += 0x10;
                v4 += 0x10;
                if out_index + 0x10 > plaintext.len() {
                    break;
                }
            }
        }

        if plaintext.len() > v4 {
            self.increment_counter();
            self.aes2 = self.counter;
            ecb_encrypt(&self.cipher, &mut self.aes2);
            let mut aes3_index = v1;
            let mut remaining = plaintext.len() - v4;
            let mut out_index = v4;
            v4 += remaining;
            loop {
                let val = self.aes2[aes3_index] ^ plaintext[out_index];
                dest[out_index] = val;
                self.aes3[aes3_index] ^= val;
                out_index += 1;
                aes3_index += 1;
                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }

        self.var2 += v4 as u32;
    }

    /// Finalise and write the authentication tag (≤ 16 bytes) into `dest`.
    pub fn calculate_checksum(&mut self, dest: &mut [u8]) {
        if self.var2 == 0 && self.var1 != 0 && (self.var1 & 0xF) != 0 {
            harpo_hash::hash(&mut self.aes3, &self.lut);
        }
        if self.var2 != 0 && (self.var2 & 0xF) != 0 {
            harpo_hash::hash(&mut self.aes3, &self.lut);
        }

        let v1 = self.var2 << 3;
        for i in (0xC..=0xF).rev() {
            self.aes3[i] ^= (v1 >> ((0xF - i) * 8)) as u8;
        }
        self.aes3[0xB] ^= (self.var2 >> 29) as u8;

        let v2 = self.var1 << 3;
        for i in (4..=7).rev() {
            self.aes3[i] ^= (v2 >> ((7 - i) * 8)) as u8;
        }
        self.aes3[3] ^= (self.var1 >> 29) as u8;

        harpo_hash::hash(&mut self.aes3, &self.lut);

        self.aes2 = self.iv_extension;
        ecb_encrypt(&self.cipher, &mut self.aes2);
        for (j, d) in dest.iter_mut().enumerate() {
            *d = self.aes2[j] ^ self.aes3[j];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn harpo_hash_matches_harpos7() {
        // HarpoHashTests.TbHashTest (MIT).
        let seed = hex!("914fa2b1aea05ba6548d1f242ca60124");
        let mut lut = [0u32; 1024];
        harpo_hash::generate_lookup_table(&seed, &mut lut);
        let mut data = [0xCCu8; 16];
        harpo_hash::hash(&mut data, &lut);
        assert_eq!(data, hex!("8350fa4dee9e240bb13929cb4b020a14"));
    }

    #[test]
    fn ctr_init_counter_matches_harpos7() {
        // HarpoAesCtrTests.TestInit (MIT).
        let mut aes = HarpoAesCtr::new(&hex!("4e001016db625dcce9105bdcd8a1b42c"));
        aes.init(&[0xCCu8; 16]);
        assert_eq!(aes.counter, hex!("d478de8b1a40ed2f89f80166eefcd513"));
    }

    #[test]
    fn ctr_encrypt_matches_harpos7() {
        // HarpoAesCtrTests.TestEncrypt2Times (MIT).
        let mut aes = HarpoAesCtr::new(&hex!("4e001016db625dcce9105bdcd8a1b42c"));
        aes.init(&[0xCCu8; 16]);

        let mut out1 = [0u8; 16];
        aes.encrypt_ctr(&hex!("b1b3d9484c6e4240403f63c6b5012cc5"), &mut out1);
        assert_eq!(out1, hex!("cdf986a8c7975d5390535c1d7954fa7f"));

        let mut out2 = [0u8; 24];
        aes.encrypt_ctr(&[0xDDu8; 24], &mut out2);
        assert_eq!(
            out2,
            hex!("33ff1bd5018448 6b897adb64154b785fab877a290783240a")
        );
    }
}
