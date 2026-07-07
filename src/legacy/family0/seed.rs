// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Ported from `bonk-dev/HarpoS7` (MIT):
//   HarpoS7.Family0/Transforms/SeedTransform.cs
// See `LICENSE-HarpoS7`.

//! `SeedTransform`: the 60-byte EC-encrypted seed (the RealPlc blob's ECIES core).
//!
//! Output layout: `dst[0x14..0x28]` = affine x of the ephemeral point `k·G`; `dst[0x28..0x3C]`
//! = the ephemeral scalar seed `prng1`; `dst[0x00..0x14]` = the ECDH combine
//! `Monolith11( t1 ‖ Transform13( Monolith8( k·publicKey ) ) )`. The same `(prng1, prng2)`
//! drives both `k·G` and `k·publicKey`.
//!
//! The two ephemeral 20-byte buffers are supplied by the injected `fill_random` closure (a
//! CSPRNG in production; a fixed fill in the golden tests, mirroring HarpoS7's
//! `SpanExtensions.StaticFillSequence`).

use super::data::TRANSFORM7_DATA;
use super::monolith;
use super::transform7::transform7;
use super::transforms::transform13;

/// `SeedTransform.DestinationSize` (used by the seed round-trip test).
#[cfg(test)]
const SEED_LEN: usize = 0x3C;

/// `Monolith1.Loop(buf, buf)` — the aliased (destination == source) normalization used by
/// `SeedTransform`: run `execute` (reading a snapshot, writing `buf`) until it returns nonzero.
fn loop_normalize_in_place(buf: &mut [u8]) {
    loop {
        let src = buf.to_vec();
        if monolith::m1::execute(buf, &src) != 0 {
            break;
        }
    }
}

/// `SeedTransform.Execute(destination[0x3C], publicKey[0x28], t1[0x3C])`.
pub fn seed_transform(
    destination: &mut [u8],
    public_key: &[u8],
    t1: &[u8],
    fill_random: &mut dyn FnMut(&mut [u8]),
) {
    let mut prng1 = [0u8; 20];
    fill_random(&mut prng1);

    let mut prng2 = [0u8; 20];
    let mut t7 = [0u8; 72];
    let base = &TRANSFORM7_DATA[0xD8..0x100]; // base point G (40 bytes)

    // Ephemeral point R = k·G; retry until its affine x is nonzero.
    loop {
        fill_random(&mut prng2);
        transform7(&mut t7, &prng1, &prng2, base);
        loop_normalize_in_place(&mut t7);
        let mut x = [0u8; 20];
        monolith::m2::execute(&mut x, &t7);
        if x.iter().any(|&b| b != 0) {
            destination[0x14..0x28].copy_from_slice(&x);
            break;
        }
    }
    destination[0x28..0x3C].copy_from_slice(&prng1);

    // ECDH point k·publicKey, combined with t1.
    transform7(&mut t7, &prng1, &prng2, public_key);
    loop_normalize_in_place(&mut t7);

    let mut m8 = [0u8; 60];
    monolith::m8::execute(&mut m8, &t7);

    let mut m11src = [0u8; 120];
    m11src[..60].copy_from_slice(&t1[..60]);
    transform13(&mut m11src[60..120], &m8);

    let mut m11 = [0u8; 20];
    monolith::m11::execute(&mut m11, &m11src);
    destination[0..20].copy_from_slice(&m11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_transform_matches_transform6() {
        // transform6: StaticFillSequence = [0x2D] -> every fill is 0x2D.
        let public_key =
            include_bytes!("../../../tests/vectors/family0/transforms/transform6-publicKey.bin");
        let t1 = include_bytes!("../../../tests/vectors/family0/transforms/transform6-t1.bin");
        let expected =
            include_bytes!("../../../tests/vectors/family0/transforms/transform6-dst.bin");

        let mut dst = [0u8; SEED_LEN];
        let mut fill = |b: &mut [u8]| b.fill(0x2D);
        seed_transform(&mut dst, public_key, t1, &mut fill);
        assert_eq!(&dst[..], &expected[..]);
    }
}
