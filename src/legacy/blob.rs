// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from bonk-dev/HarpoS7 `HarpoS7.Utilities/{Auth/BlobMetadataWriter,Extensions/KeyExtensions}.cs`
// (MIT, (c) 2024 bonk). Public key from `HarpoS7.PublicKeys/Keys/03/B07654AC9CAA4ACA.bin`.

//! The `SecurityKeyEncryptedKey` blob metadata header and key-id derivation (PLCSIM family).

use sha2::{Digest, Sha256};

/// Length of the full PLCSIM-family encrypted-key blob.
pub const PLCSIM_BLOB_LEN: usize = 216;

const SYMMETRIC_KEY_FLAGS: u32 = 0x0301; // KeyTypeSymmetricSessionKey | KeyFamilyVPlc
const PUBLIC_KEY_FLAGS: u32 = 0x0310; // KeyTypeCommPublicKey | KeyFamilyVPlc

/// Public key of the PLCSIM Advanced instance used for development — family `03`, fingerprint
/// `03:B07654AC9CAA4ACA` (from HarpoS7's bundled key set). A general client would select the
/// key by the fingerprint the PLC reports in its `CreateObject` response.
pub const PLCSIM_PUBLIC_KEY: [u8; 64] = [
    0x47, 0x00, 0xdb, 0x8f, 0xa2, 0x5d, 0x79, 0x1c, 0x2a, 0x77, 0xee, 0xc9, 0x79, 0x5d, 0x66, 0xe3,
    0xb5, 0xf2, 0xba, 0x9a, 0x59, 0x50, 0x8a, 0xdd, 0x51, 0x0c, 0xa9, 0xfe, 0x87, 0x62, 0xaa, 0x08,
    0x1d, 0xff, 0x80, 0xea, 0x8f, 0x73, 0x0a, 0xd4, 0xca, 0xa0, 0xbc, 0xa7, 0xba, 0x92, 0x89, 0x2c,
    0x69, 0x19, 0x84, 0x33, 0x8e, 0xec, 0x20, 0x47, 0x68, 0x1d, 0x95, 0x8d, 0xc5, 0xc5, 0x08, 0x6a,
];

/// `DeriveKeyId` = `SHA256(key[..24] || "DERIVE")[..8]` — the 8-byte key identifier the PLC
/// uses to recognise which key a blob references.
pub fn derive_key_id(key: &[u8]) -> [u8; 8] {
    let mut buf = [0u8; 24 + 6];
    buf[..24].copy_from_slice(&key[..24]);
    buf[24..].copy_from_slice(b"DERIVE");
    let digest = Sha256::digest(buf);
    let mut id = [0u8; 8];
    id.copy_from_slice(&digest[..8]);
    id
}

/// Write the 48-byte blob metadata header (PLCSIM family) into `dest`. Returns the next
/// writable offset (always 48).
pub fn write_metadata(dest: &mut [u8], public_key: &[u8], key1: &[u8]) -> usize {
    dest[0..4].copy_from_slice(&0xFEE1_DEADu32.to_le_bytes());
    dest[4..8].copy_from_slice(&(PLCSIM_BLOB_LEN as u32).to_le_bytes());
    dest[8..12].copy_from_slice(&1u32.to_le_bytes()); // security key version
    dest[12..16].copy_from_slice(&1u32.to_le_bytes()); // legacy CSI security level
    dest[16..24].copy_from_slice(&derive_key_id(key1));
    dest[24..28].copy_from_slice(&SYMMETRIC_KEY_FLAGS.to_le_bytes());
    dest[28..32].copy_from_slice(&0u32.to_le_bytes());
    dest[32..40].copy_from_slice(&derive_key_id(public_key));
    dest[40..44].copy_from_slice(&PUBLIC_KEY_FLAGS.to_le_bytes());
    dest[44..48].copy_from_slice(&0u32.to_le_bytes());
    48
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    // KeyExtensionsTests.DeriveKeyId (MIT).
    #[test]
    fn derive_key_id_matches_harpos7() {
        assert_eq!(
            derive_key_id(&hex!(
                "eca6d799ddf03eaadd16b5d7245331e426c9e6ba8997877a7394f3286532a6b0"
                "53e4229818085223432483fba4d5c43bd6c354c10febc903908ed271697f39e9"
            )),
            hex!("84d2485f016b9b5a"),
        );
        assert_eq!(derive_key_id(&[0x11; 24]), hex!("06ddcee4adaec77a"));
        assert_eq!(derive_key_id(&[0x44; 24]), hex!("06d0ef4b10626822"));
    }

    // BlobMetadataWriterTests.WriteMetadataWritesExpectedHeaderAndFlags (PlcSim case, MIT).
    #[test]
    fn write_metadata_plcsim_header() {
        let public_key = [0u8; 64];
        let key1 = [0u8; 24];
        let mut dest = [0u8; PLCSIM_BLOB_LEN];

        let offset = write_metadata(&mut dest, &public_key, &key1);
        assert_eq!(offset, 48);

        let dw = |i: usize| u32::from_le_bytes(dest[i..i + 4].try_into().unwrap());
        assert_eq!(dw(0), 0xFEE1_DEAD);
        assert_eq!(dw(4), 216);
        assert_eq!(dw(8), 1);
        assert_eq!(dw(12), 1);
        assert_eq!(&dest[16..24], &derive_key_id(&key1));
        assert_eq!(dw(24), 0x0301);
        assert_eq!(dw(28), 0);
        assert_eq!(&dest[32..40], &derive_key_id(&public_key));
        assert_eq!(dw(40), 0x0310);
        assert_eq!(dw(44), 0);
    }
}
