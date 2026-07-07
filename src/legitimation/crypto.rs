// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Legitimation/LegitimationCrypto.cs,
// LGPL-3.0-or-later.

//! Cryptographic primitives for legitimation (auth).
//!
//! The "new" (firmware ≥ V3.1) legitimation path derives an AES key from the RFC 5705
//! exported keying material (`sha256(oms_secret)`), then AES-256-CBC/PKCS7-encrypts the
//! credentials payload using the first 16 bytes of the PLC challenge as the IV. These are
//! faithful ports of `LegitimationCrypto.sha256` and `LegitimationCrypto.EncryptAesCbc`.
//!
//! This module is pure, deterministic crypto with no protocol dependencies, so it is
//! unit-tested independently of the surrounding legitimation flow (which needs the
//! object and value model).

use aes::Aes256;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::error::{Error, Result};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// Length of an AES block / a CBC IV, in bytes.
pub const AES_BLOCK_LEN: usize = 16;

/// Compute the SHA-256 digest of `data` (`LegitimationCrypto.sha256`).
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute the SHA-1 digest of `data`. Used to hash the password for the legacy
/// legitimation payload (`buildLegitimationPayload`, username-less login).
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// AES-256-CBC encrypt `plaintext` with PKCS7 padding (`LegitimationCrypto.EncryptAesCbc`).
///
/// `key` must be 32 bytes (AES-256); `iv` must be 16 bytes.
pub fn encrypt_aes256_cbc_pkcs7(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let enc = Aes256CbcEnc::new_from_slices(key, iv)
        .map_err(|_| Error::Crypto("invalid AES-256 key or IV length".into()))?;
    Ok(enc.encrypt_padded_vec_mut::<Pkcs7>(plaintext))
}

/// AES-256-CBC decrypt `ciphertext`, removing PKCS7 padding. Inverse of
/// [`encrypt_aes256_cbc_pkcs7`]; provided for testing and symmetry.
pub fn decrypt_aes256_cbc_pkcs7(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let dec = Aes256CbcDec::new_from_slices(key, iv)
        .map_err(|_| Error::Crypto("invalid AES-256 key or IV length".into()))?;
    dec.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(|_| Error::Crypto("invalid PKCS7 padding".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn sha256_abc_known_vector() {
        // FIPS 180-2 / well-known: sha256("abc").
        assert_eq!(
            sha256(b"abc"),
            hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        );
    }

    #[test]
    fn sha1_abc_known_vector() {
        assert_eq!(
            sha1(b"abc"),
            hex!("a9993e364706816aba3e25717850c26c9cd0d89d")
        );
    }

    #[test]
    fn aes256_cbc_first_block_matches_nist() {
        // NIST SP 800-38A, F.2.5 CBC-AES256.Encrypt: the first ciphertext block is
        // deterministic regardless of PKCS7 padding (which only appends a trailing block).
        let key = hex!("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4");
        let iv = hex!("000102030405060708090a0b0c0d0e0f");
        let plaintext = hex!("6bc1bee22e409f96e93d7e117393172a");
        let ct = encrypt_aes256_cbc_pkcs7(&key, &iv, &plaintext).unwrap();
        assert_eq!(&ct[..16], &hex!("f58c4c04d6e5f1ba779eabfb5f7bfbd6")[..]);
        // PKCS7 pads a full block to two blocks.
        assert_eq!(ct.len(), 32);
    }

    #[test]
    fn aes256_cbc_roundtrip() {
        let key = sha256(b"oms-secret-stand-in-32-bytes-aaa");
        let iv = [0x11u8; AES_BLOCK_LEN];
        let plaintext = b"username\0password\0extra";
        let ct = encrypt_aes256_cbc_pkcs7(&key, &iv, plaintext).unwrap();
        let pt = decrypt_aes256_cbc_pkcs7(&key, &iv, &ct).unwrap();
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn rejects_bad_lengths() {
        assert!(encrypt_aes256_cbc_pkcs7(&[0u8; 16], &[0u8; 16], b"x").is_err());
        assert!(encrypt_aes256_cbc_pkcs7(&[0u8; 32], &[0u8; 8], b"x").is_err());
    }
}
