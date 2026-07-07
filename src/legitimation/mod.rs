// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Legitimation/*, LGPL-3.0-or-later.

//! Legitimation (authentication).
//!
//! The "new" (firmware ≥ V3.1) path: fetch a challenge from `ServerSessionRequest` via
//! `GetVarSubstreamed`, build a credentials [`PValue::Struct`], AES-256-CBC/PKCS7-encrypt
//! it with `key = sha256(oms_secret)` and `iv = challenge[..16]`, then submit the
//! ciphertext as a `Blob` to `Legitimate` via `SetVariable`. The flow is driven by
//! [`crate::Connection::legitimate`]; this module provides the payload builder and crypto.

pub mod crypto;

use crate::value::PValue;
use crate::wire::pdu::ids;

/// Legitimation method discriminator (`LegitimationType`).
pub mod legitimation_type {
    /// Legacy login (selected by an empty username).
    pub const LEGACY: u32 = 1;
    /// New (username + password) login.
    pub const NEW: u32 = 2;
}

/// Build the legitimation credentials payload (`buildLegitimationPayload`): a struct with
/// the legitimation type and username/password blobs.
///
/// Mirrors the reference exactly: an **empty username** selects the *legacy* login (type
/// `LEGACY`, password stored as its **SHA-1 hash** — the common "password for full access"
/// case); a **non-empty username** selects the *new* login (type `NEW`, plaintext UTF-8
/// username and password). Both forms are AES-encrypted by the caller for fw ≥ V3.1.
pub fn build_legitimation_payload(username: &str, password: &str) -> PValue {
    let (type_value, password_data) = if username.is_empty() {
        (
            legitimation_type::LEGACY,
            crypto::sha1(password.as_bytes()).to_vec(),
        )
    } else {
        (legitimation_type::NEW, password.as_bytes().to_vec())
    };

    PValue::Struct {
        id: ids::LID_LEGITIMATION_PAYLOAD_STRUCT,
        elements: vec![
            (
                ids::LID_LEGITIMATION_PAYLOAD_TYPE,
                PValue::UDInt(type_value),
            ),
            (
                ids::LID_LEGITIMATION_PAYLOAD_USERNAME,
                PValue::Blob {
                    root_id: 0,
                    data: username.as_bytes().to_vec(),
                },
            ),
            (
                ids::LID_LEGITIMATION_PAYLOAD_PASSWORD,
                PValue::Blob {
                    root_id: 0,
                    data: password_data,
                },
            ),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_serializes_to_struct() {
        let payload = build_legitimation_payload("user", "pw");
        let mut out = Vec::new();
        payload.serialize(&mut out).unwrap();
        // flags 00, datatype 0x17 (struct), struct id 40400 = 0x00009dd0 fixed.
        assert_eq!(&out[0..6], &[0x00, 0x17, 0x00, 0x00, 0x9d, 0xd0]);
        // ends with the struct terminator 0x00.
        assert_eq!(*out.last().unwrap(), 0x00);
    }

    #[test]
    fn empty_username_uses_legacy_type_and_sha1_password() {
        let payload = build_legitimation_payload("", "secret");
        match payload {
            PValue::Struct { id, elements } => {
                assert_eq!(id, ids::LID_LEGITIMATION_PAYLOAD_STRUCT);
                assert_eq!(elements[0].1, PValue::UDInt(legitimation_type::LEGACY));
                // Password element is the SHA-1 of the password (20 bytes), not plaintext.
                match &elements[2].1 {
                    PValue::Blob { data, .. } => {
                        assert_eq!(data.as_slice(), &crypto::sha1(b"secret")[..]);
                        assert_eq!(data.len(), 20);
                    }
                    other => panic!("expected Blob, got {other:?}"),
                }
            }
            other => panic!("expected Struct, got {other:?}"),
        }
    }

    #[test]
    fn nonempty_username_uses_new_type_and_plaintext_password() {
        let payload = build_legitimation_payload("admin", "pw");
        if let PValue::Struct { elements, .. } = payload {
            assert_eq!(elements[0].1, PValue::UDInt(legitimation_type::NEW));
            assert_eq!(
                elements[2].1,
                PValue::Blob {
                    root_id: 0,
                    data: b"pw".to_vec()
                }
            );
        } else {
            panic!("expected Struct");
        }
    }
}
