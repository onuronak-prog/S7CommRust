// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from bonk-dev/HarpoS7 `HarpoS7/Keys/KeyUtilities.cs` (MIT, (c) 2024 bonk).

//! Key-derivation functions for the legacy auth handshake.

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use super::digest::SESSION_KEY_LEN;
use super::fingerprint::fingerprint_challenge;

/// `DeriveSessionKey` = `HMAC-SHA256(key1[..24], f(challenge) || challenge[2..18])[..24]`.
///
/// The result keys the per-PDU integrity digest ([`super::digest::packet_digest`]).
pub fn derive_session_key(key1: &[u8], challenge: &[u8]) -> [u8; SESSION_KEY_LEN] {
    let fp = fingerprint_challenge(challenge);
    let mut source = [0u8; 8 + 16];
    source[..8].copy_from_slice(&fp);
    source[8..].copy_from_slice(&challenge[2..18]);

    let mut mac = Hmac::<Sha256>::new_from_slice(&key1[..SESSION_KEY_LEN]).expect("any key length");
    mac.update(&source);
    let out = mac.finalize().into_bytes();

    let mut key = [0u8; SESSION_KEY_LEN];
    key.copy_from_slice(&out[..SESSION_KEY_LEN]);
    key
}

/// `DeriveChallengeEncryptionKey` = `SHA256(random_key[..24] || 01..0f 00)[..16]` — the
/// AES-128 key used to encrypt the challenge and `key1` into the blob (PLCSIM path).
pub fn derive_challenge_encryption_key(random_key: &[u8]) -> [u8; 16] {
    let mut plaintext = [0u8; 24 + 16];
    plaintext[..24].copy_from_slice(&random_key[..24]);
    // 16-byte magic: little-endian dwords 0x04030201, 0x08070605, 0x0C0B0A09, 0x000F0E0D.
    plaintext[24..].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]);

    let digest = Sha256::digest(plaintext);
    let mut key = [0u8; 16];
    key.copy_from_slice(&digest[..16]);
    key
}

/// `DeriveSeedEncryptionKeyAndIv` -> 48 bytes (a 32-byte AES key candidate + a 16-byte IV
/// region): `SHA256(rev(a2[..32]) || a3[..64] || 0x00)[..32]` followed by
/// `SHA256(rev(a2[..32]) || a3[..64] || 0x20)[..16]`. `rev` is a full byte reversal.
pub fn derive_seed_encryption_key_and_iv(a2: &[u8], a3: &[u8]) -> [u8; 48] {
    let mut v1 = [0u8; 32 + 64 + 1];
    // `Span<byte>.ReverseBytes` is a full byte reversal for lengths that are a multiple of 8.
    for (i, dst) in v1[..32].iter_mut().enumerate() {
        *dst = a2[31 - i];
    }
    v1[32..96].copy_from_slice(&a3[..64]);

    let mut dest = [0u8; 48];
    let mut offset = 0u8;
    while (0x30i32 - offset as i32) > 0 {
        v1[96] = offset;
        let digest = Sha256::digest(v1);
        let size = (0x30 - offset as i32).min(0x20) as usize;
        dest[offset as usize..offset as usize + size].copy_from_slice(&digest[..size]);
        offset += 0x20;
    }
    dest
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    // All vectors from HarpoS7 `KeyUtilitiesTests.cs` (MIT).
    #[test]
    fn derive_session_key_vectors() {
        let challenge1 = [
            184u8, 13, 177, 179, 217, 72, 76, 110, 66, 64, 64, 63, 99, 198, 181, 1, 44, 197, 46,
            127,
        ];
        assert_eq!(
            derive_session_key(&[0xDD; 24], &challenge1),
            hex!("a233875e3c7fc059c016128de590ab3c28bc04c277fa7c51"),
        );
        assert_eq!(
            derive_session_key(
                &[0x11; 24],
                &hex!("07d53c9f8f4faa90084fc8b0f06a46591c296584")
            ),
            hex!("13ae7ac7a1b7897619bee7c1537732fdb77db96a1062adfc"),
        );
        assert_eq!(
            derive_session_key(
                &[0x44; 24],
                &hex!("5b8a85c93166a953404a1ccb17814ce9dcc7385c")
            ),
            hex!("9f0d95569f9af3ce1b20cfbe4c0d5611b2ef26d801660bc2"),
        );
    }

    #[test]
    fn derive_challenge_encryption_key_vectors() {
        assert_eq!(
            derive_challenge_encryption_key(&[0xDD; 24]),
            hex!("4e001016db625dcce9105bdcd8a1b42c"),
        );
        assert_eq!(
            derive_challenge_encryption_key(&hex!(
                "d36e04f64f89c24e6cb9276d82409ee0e57b98f815063ef4"
            )),
            hex!("b2b7de6183fc1a97f8636952f1aba0fd"),
        );
    }

    #[test]
    fn derive_seed_encryption_key_and_iv_vector() {
        let a2 = hex!("b3420d0c6242b150d6862a4d61559e78a00da5dc7b68551ad86df007aba5bbd9");
        let a3 = hex!(
            "18f124e0b4a6d964cefc8453ed903d52f1b8c85258fe5b2459776c0630dc02fe
             f6d0b082d6d10b1ea728a50037ae69ebd7cca10b1c73da8ac3c287d2704bf325"
        );
        assert_eq!(
            derive_seed_encryption_key_and_iv(&a2, &a3),
            hex!(
                "43950f7b8b896e30457824dc8a591e328772abb8b3c19371
                 29642275610a4a4532687f19c02ca9ef361388943560918c"
            ),
        );
    }
}
