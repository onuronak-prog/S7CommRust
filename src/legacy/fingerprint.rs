// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from bonk-dev/HarpoS7 `HarpoS7/Fingerprint/HarpoFingerprint.cs` (MIT, (c) 2024 bonk).
// The constant tables and per-round mutations live in the generated `fingerprint_consts`.

//! The `f()` challenge fingerprint: a custom 8-byte tabulation/permutation hash of the
//! per-session challenge, used as one input to the session-key derivation
//! ([`super::keys`]). It is not a standard hash; the logic and its constant tables are a
//! faithful port of HarpoS7.

use super::fingerprint_consts::{
    mutate, BIG_CONTEXT_INIT, BIG_CONTEXT_LEN, DATA1_COLLECTION, DATA2_COLLECTION, FINGERPRINT_LEN,
    NUM_MUTATIONS, SMALL_CONTEXT_LEN, XOR_MAGIC,
};

/// Compute `f(challenge)` — 8 bytes derived from `challenge[2..18]`. Returns `None` if the
/// challenge is not fingerprintable (a HarpoS7 algorithm limitation; see [`sub_procedure`]).
///
/// `challenge` must be at least 18 bytes (pass the original 20-byte server challenge).
fn try_fingerprint(challenge: &[u8]) -> Option<[u8; FINGERPRINT_LEN]> {
    assert!(challenge.len() >= 18, "challenge must be at least 18 bytes");

    let mut small = [0u8; SMALL_CONTEXT_LEN];
    let mut big = [0u32; BIG_CONTEXT_LEN];

    // InitializeContexts: seed the big context with its initial value and the small context
    // with the 16 middle bytes of the challenge.
    big.copy_from_slice(&BIG_CONTEXT_INIT);
    small[..16].copy_from_slice(&challenge[2..18]);

    for round in 0..NUM_MUTATIONS {
        if !sub_procedure(
            DATA1_COLLECTION[round],
            XOR_MAGIC[round],
            DATA2_COLLECTION[round],
            &mut small,
            &big,
        ) {
            return None;
        }
        mutate(&mut big, round);
    }

    Some(final_fingerprint(&small))
}

/// Compute `f(challenge)`. Panics if the challenge is not fingerprintable — callers on the live
/// path must first gate the challenge with [`is_challenge_fingerprintable`] (and reconnect for a
/// fresh challenge otherwise), so this only sees fingerprintable challenges.
pub fn fingerprint_challenge(challenge: &[u8]) -> [u8; FINGERPRINT_LEN] {
    try_fingerprint(challenge)
        .expect("challenge is fingerprintable — gate with is_challenge_fingerprintable() first")
}

/// Whether `f(challenge)` can be computed. About 1 in 6 random challenges cannot be (a HarpoS7
/// limitation), and the derived session key is unobtainable for those — reconnect for a new one.
pub fn is_challenge_fingerprintable(challenge: &[u8]) -> bool {
    challenge.len() >= 18 && try_fingerprint(challenge).is_some()
}

/// Shift amount selector (`PwVarMask`): 4 for an even value, 0 for an odd one.
fn pw_var_mask(value: u8) -> u32 {
    ((((value & 1) as i32) * -4 + 4) & 0x1F) as u32
}

/// Read a nibble/byte from the small context (`PwVarRead`).
fn pw_var_read(value: u16, small: &[u8]) -> u8 {
    small[(value >> 1) as usize] >> pw_var_mask((value & 0xff) as u8)
}

/// One byte of `data2` viewed little-endian (C# `Span<ushort>.AsBytes()`). Returns `None` when
/// the byte index is past the end of the table — see [`sub_procedure`].
fn data2_byte(data2: &[u16], k: usize) -> Option<u32> {
    data2
        .get(k >> 1)
        .map(|&w| ((w >> (8 * (k & 1) as u16)) & 0xff) as u32)
}

/// `FingerprintSubProcedure`: walks `data1` in triplets, mixing the small and big contexts.
///
/// Returns `false` if a `data2` access runs past the end of the round's table. This is a genuine
/// limitation of the reverse-engineered HarpoS7 algorithm: for round 1 the `data2` table is one
/// `u16` short of the worst-case index, so ~1/6 of (random) challenges cannot be fingerprinted.
/// Callers treat that challenge as unusable and retry with a fresh one rather than panic.
#[must_use]
fn sub_procedure(
    data1: &[u16],
    xor_magic: u32,
    data2: &[u16],
    small: &mut [u8],
    big: &[u32],
) -> bool {
    let mut index = 0usize;
    let mut ctx_offset: u32 = 0;
    loop {
        let pw0 = pw_var_read(data1[index], small);
        let pw1 = pw_var_read(data1[index + 1], small);
        let pw2 = data1[index + 2];
        index += 3;

        let static3 = ((pw1 as u32 & 0xF) | ((pw0 as u32) << 4)) & 0xFF;
        let t1 = (static3 >> 3) + (ctx_offset >> 2);
        let big_index = (t1 % 0x2F) as usize;
        let static6 = big[big_index] ^ xor_magic;

        let t4 = ((7u32 - (pw1 as u32 & 0x7)) * 0x04) & 0x1F;
        let static5 = (static6 >> t4) as u8;

        let ctx_buffer_index = (pw2 >> 1) as usize;
        let t5 = pw_var_mask((pw2 & 0xff) as u8);
        let data2_index = ((static3 >> 1) + ctx_offset) as usize;

        let Some(d2) = data2_byte(data2, data2_index) else {
            return false; // unfingerprintable challenge — caller retries with a fresh one
        };
        let f_val = ((((d2 >> pw_var_mask(pw1)) ^ static5 as u32) & 0xF) << t5)
            | ((0xF0u32 >> t5) & small[ctx_buffer_index] as u32);
        small[ctx_buffer_index] = f_val as u8;

        ctx_offset = ctx_offset.wrapping_add(0x80);
        if index >= data1.len() {
            break;
        }
    }
    true
}

/// `FinalFingerprint`: extract the 8 output bytes from fixed small-context positions. The
/// expressions mirror HarpoS7 exactly; Rust shares C's `<< >> & ^ |` precedence so the
/// (already parenthesised) source ports verbatim. `fp[k]` starts at 0.
fn final_fingerprint(small: &[u8]) -> [u8; FINGERPRINT_LEN] {
    let s = |i: usize| small[i] as u32;
    let mut fp = [0u32; FINGERPRINT_LEN];

    fp[0] = (s(93) << 4) | (s(224) >> 4);
    fp[1] = ((((fp[1] ^ s(189)) & 0xf ^ s(189)) ^ s(53)) & 0xF) ^ ((fp[1] ^ s(189)) & 0xf ^ s(189));
    fp[2] = ((fp[2] & 0xf | s(119) << 4) ^ s(86)) & 0xF ^ (fp[2] & 0xf | s(119) << 4);
    fp[3] = ((fp[3] ^ s(83)) & 0xf ^ s(83)) & 0xF0 | (s(33) >> 4);
    fp[4] = ((((fp[4] ^ s(229)) & 0xf ^ s(229)) ^ s(58)) & 0xF) ^ ((fp[4] ^ s(229)) & 0xf ^ s(229));
    fp[5] = ((((fp[5] ^ s(69)) & 0xf ^ s(69)) ^ s(165)) & 0xF) ^ ((fp[5] ^ s(69)) & 0xf ^ s(69));
    fp[6] = ((fp[6] ^ s(63)) & 0xf ^ s(63)) & 0xF0 | (s(89) >> 4);
    fp[7] =
        ((((fp[7] ^ s(172)) & 0xf ^ s(172)) ^ s(247)) & 0xF) ^ ((fp[7] ^ s(172)) & 0xf ^ s(172));

    let mut out = [0u8; FINGERPRINT_LEN];
    for (o, v) in out.iter_mut().zip(fp) {
        *o = v as u8;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden vector from HarpoS7 `HarpoFingerprintTests.FingerprintChallenge` (MIT).
    #[test]
    fn fingerprint_challenge_matches_harpos7() {
        let challenge: [u8; 20] = [
            184, 13, 177, 179, 217, 72, 76, 110, 66, 64, 64, 63, 99, 198, 181, 1, 44, 197, 46, 127,
        ];
        assert_eq!(
            fingerprint_challenge(&challenge),
            [0xe2, 0x87, 0xc1, 0xcb, 0x65, 0x9b, 0x9e, 0xdf],
        );
    }

    // The HarpoS7 algorithm can't fingerprint ~1/6 of challenges (its round-1 `data2` table is one
    // `u16` short of the worst-case index). `is_challenge_fingerprintable` must catch those, and
    // `fingerprint_challenge` must never panic on a challenge it reports as fingerprintable.
    #[test]
    fn detects_unfingerprintable_challenges() {
        let golden = [
            184u8, 13, 177, 179, 217, 72, 76, 110, 66, 64, 64, 63, 99, 198, 181, 1, 44, 197, 46,
            127,
        ];
        assert!(is_challenge_fingerprintable(&golden));

        let (mut ok, mut bad) = (0u32, 0u32);
        for seed in 0u32..3000 {
            let mut c = [0u8; 20];
            for (i, b) in c.iter_mut().enumerate() {
                *b = (seed
                    .wrapping_mul(2_654_435_761)
                    .wrapping_add(i as u32 * 40_503)
                    >> 5) as u8;
            }
            if is_challenge_fingerprintable(&c) {
                ok += 1;
                let _ = fingerprint_challenge(&c); // must not panic
            } else {
                bad += 1;
            }
        }
        // Both cases must occur (the split is roughly 5:1, but assert only that each is non-empty).
        assert!(ok > 0 && bad > 0, "expected a mix: ok={ok} bad={bad}");
    }
}
