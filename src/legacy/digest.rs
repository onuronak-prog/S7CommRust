// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from bonk-dev/HarpoS7 `HarpoS7/Integrity/HarpoPacketDigest.cs` (MIT, (c) 2024 bonk).

//! The per-PDU integrity digest for the legacy (non-TLS) dialect.
//!
//! Every S7CommPlus PDU after `CreateObject` carries `HMAC-SHA256(session_key[..24], data)`
//! over its data part. The PLC drops the connection on a missing or wrong digest.

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{Error, Result};

/// The derived session key is 24 bytes; those 24 bytes are the HMAC key.
pub const SESSION_KEY_LEN: usize = 24;
/// Length of an integrity digest — the full HMAC-SHA256 output.
pub const DIGEST_LEN: usize = 32;

/// Compute a PDU's integrity digest: `HMAC-SHA256(session_key[..24], data)` (32 bytes).
///
/// `data` is the S7CommPlus data part covered by the digest (i.e. excluding the 4-byte
/// `72 ver len` header, the digest field itself, and the trailer).
pub fn packet_digest(session_key: &[u8], data: &[u8]) -> Result<[u8; DIGEST_LEN]> {
    if session_key.len() < SESSION_KEY_LEN {
        return Err(Error::Crypto(format!(
            "session key must be at least {SESSION_KEY_LEN} bytes (got {})",
            session_key.len()
        )));
    }
    let mut mac = Hmac::<Sha256>::new_from_slice(&session_key[..SESSION_KEY_LEN])
        .expect("HMAC accepts a key of any length");
    mac.update(data);
    let mut digest = [0u8; DIGEST_LEN];
    digest.copy_from_slice(&mac.finalize().into_bytes());
    Ok(digest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    // Golden vectors lifted verbatim from HarpoS7
    // `HarpoS7.Tests/Integrity/HarpoPacketDigestTests.cs` (MIT).
    #[test]
    fn matches_harpos7_vectors() {
        // Vector 1 (PLCSIM session key).
        assert_eq!(
            packet_digest(
                &hex!("a233875e3c7fc059c016128de590ab3c28bc04c277fa7c51"),
                &hex!(
                    "31 00 00 04 d4 00 00 00 03 70 40 00
                     00 34 70 40 00 00 00 00 00 04 e8 89
                     69 00 12 00 00 00 00 89 6a 00 13 00
                     89 6b 00 04 00 00 03 00 00 00 00"
                ),
            )
            .unwrap(),
            hex!("44e681f7e915bafa3b8c9eacb9d1f6bb47ebf1469c49c41efb6fc8156d35426a"),
        );

        // Vector 2.
        assert_eq!(
            packet_digest(
                &hex!("4eaf8d971ffcf45a995947cc06bff85b0a2df1ba6f3ae94d"),
                &hex!(
                    "31 00 00 04 d4 00 00 00 03 70 00 10
                     3d 34 70 00 10 3d 00 00 00 04 e8 89
                     69 00 12 00 00 00 00 89 6a 00 13 00
                     89 6b 00 04 00 00 03 00 00 00 00"
                ),
            )
            .unwrap(),
            hex!("30cb4a65de8a03c3a1a290537c23c6e36db12d9bbf3ccde1212c54ea421c5fc3"),
        );

        // Vector 3 (real-PLC session key — exercises the same HMAC).
        assert_eq!(
            packet_digest(
                &hex!("65c4f179980a43cb60e1194ba500f5b9d04f374b56374866"),
                &hex!(
                    "31 00 00 04 d4 00 00 00 03 70 00 10
                     3d 34 70 00 10 3d 00 00 00 04 e8 89
                     69 00 12 00 00 00 00 89 6a 00 13 00
                     89 6b 00 04 00 00 03 00 00 00 00"
                ),
            )
            .unwrap(),
            hex!("a8bb3f236afad6774f4487136d0055771422def8ea862a8c90d2d0b54b1951f1"),
        );
    }

    #[test]
    fn rejects_short_session_key() {
        assert!(packet_digest(&[0u8; 23], b"data").is_err());
    }
}
