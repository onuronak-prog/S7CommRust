// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Ported from `bonk-dev/HarpoS7` (MIT):
//   HarpoS7.Family0/Transforms/PreSeedTransform.cs
// See `LICENSE-HarpoS7`.

//! The Family-0 keyed transforms built on the [`super::monolith`] circuits.
//!
//! [`pre_seed`] (`PreSeedTransform`) maps the 24-byte `key1` to the 60-byte `t1` that seeds
//! both `SeedTransform`'s final combine and `KeyDerivationTransform`'s AES keys. `KeyDerivation`
//! and `Transform13` (also Monolith-driven) land next.

use super::data::{BIGINT_DATA, MAGIC_POSTFIX, SHARED_DATA, TRANSFORM12_METADATA, TRANSFORM1_DATA};
use super::field;
use super::monolith;

/// Fetch a 24-byte packed field element `idx` from the `context` (idx < 0x100) or the
/// `BigIntData` constant pool (idx >= 0x100).
fn t12_gather(context: &[u8], idx: usize) -> [u8; 24] {
    let src: &[u8] = if idx < 0x100 {
        &context[idx * 24..]
    } else {
        &BIGINT_DATA[(idx - 0x100) * 24..]
    };
    src[..24].try_into().unwrap()
}

/// `Transform12.Execute`: run `count` field ops (encoded in the metadata table starting at
/// `index`) over the packed-cell `context`. Each op is `dst_cell = src1 OP src2` with OP in
/// {mul, square, add, sub} over `GF(2^160-47)`.
pub fn transform12(context: &mut [u8], index: usize, count: usize) {
    for i in 0..count {
        let off = (index + i) * 4;
        let md = u32::from_le_bytes(TRANSFORM12_METADATA[off..off + 4].try_into().unwrap());
        let dst_index = ((md >> 0x16) & 0xFF) as usize * 24;
        let src1 = t12_gather(context, ((md >> 0xB) & 0x3FF) as usize);
        let src2 = t12_gather(context, (md & 0x3FF) as usize);
        let dst = &mut context[dst_index..dst_index + 24];
        match md >> 0x1E {
            0 => field::bigint_mul(dst, &src1, &src2),
            1 => field::bigint_square(dst, &src1),
            2 => field::bigint_add(dst, &src1, &src2),
            _ => field::bigint_sub(dst, &src1, &src2),
        }
    }
}

const M9_SRC_DWORDS: usize = 0xC5; // 197 dwords = 788 bytes = Monolith9 source

fn u32s_to_bytes(a: &[u32], out: &mut [u8]) {
    for (j, &w) in a.iter().enumerate() {
        out[j * 4..j * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
}

fn rd_u32(buf: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(buf[i * 4..i * 4 + 4].try_into().unwrap())
}

/// Run `Monolith10` over `src_dwords` (its 18-dword source), writing the 192-dword result
/// into `buffer2[0..192]` (leaving the injected tail `buffer2[0xC0..0xC5]` untouched).
fn run_m10(src_dwords: &[u32], buffer2: &mut [u32; M9_SRC_DWORDS]) {
    let mut src_bytes = vec![0u8; src_dwords.len() * 4];
    u32s_to_bytes(src_dwords, &mut src_bytes);
    let mut out = [0u8; 192 * 4];
    monolith::ten::execute(&mut out, &src_bytes);
    for (j, slot) in buffer2.iter_mut().enumerate().take(192) {
        *slot = u32::from_le_bytes(out[j * 4..j * 4 + 4].try_into().unwrap());
    }
}

/// Run `Monolith9` over the 197-dword `buffer2`, returning its 24-byte output.
fn run_m9(buffer2: &[u32; M9_SRC_DWORDS]) -> [u8; 24] {
    let mut b = [0u8; M9_SRC_DWORDS * 4];
    u32s_to_bytes(buffer2, &mut b);
    let mut m9 = [0u8; 24];
    monolith::nine::execute(&mut m9, &b);
    m9
}

/// `PreSeedTransform.Execute`: `key1` (24 bytes) → `t1` (60 bytes), via three Monolith9
/// passes over `[Transform1Data(192) | key1-pair(2) | MagicPostfix(3)]`.
pub fn pre_seed(dst: &mut [u8], key1: &[u8]) {
    assert!(dst.len() >= 60, "pre_seed dst too small");
    assert!(key1.len() >= 24, "pre_seed key1 too small");

    let mut wb = [0u32; M9_SRC_DWORDS];
    wb[..192].copy_from_slice(&TRANSFORM1_DATA);
    wb[0xC2..0xC5].copy_from_slice(&MAGIC_POSTFIX);

    let mut wb_bytes = [0u8; M9_SRC_DWORDS * 4];
    for i in 0..3 {
        wb[0xC0] = u32::from_le_bytes(key1[i * 8..i * 8 + 4].try_into().unwrap());
        wb[0xC1] = u32::from_le_bytes(key1[i * 8 + 4..i * 8 + 8].try_into().unwrap());
        for (j, &w) in wb.iter().enumerate() {
            wb_bytes[j * 4..j * 4 + 4].copy_from_slice(&w.to_le_bytes());
        }

        let mut m9 = [0u8; 24];
        monolith::nine::execute(&mut m9, &wb_bytes);

        let copy = if i < 2 { 24 } else { 12 }; // 6 or 3 dwords
        dst[i * 24..i * 24 + copy].copy_from_slice(&m9[..copy]);
    }
}

/// `KeyDerivationTransform.Execute`: `t1` (60 bytes) → 48 bytes = challengeKey(16) ‖
/// checksumKey(16) ‖ lutSeed(16), via `Monolith10` then six `Monolith9` passes with
/// `SharedData` injected. Feeds `RealPlcAuthenticator.DeriveKeysAndLookupTable`.
pub fn key_derivation(dst: &mut [u8], t1: &[u8]) {
    assert!(dst.len() >= 48, "key_derivation dst too small");
    assert!(t1.len() >= 60, "key_derivation t1 too small");

    // buffer1 = [0xFFFFFFFF, 0xFFFFFFFF, 0x0000FFFF, t1(15 dwords)] (Monolith10 source).
    let mut buffer1 = [0u32; 0x18];
    buffer1[0] = 0xFFFF_FFFF;
    buffer1[1] = 0xFFFF_FFFF;
    buffer1[2] = 0x0000_FFFF;
    for j in 0..15 {
        buffer1[3 + j] = rd_u32(t1, j);
    }

    let mut buffer2 = [0u32; M9_SRC_DWORDS];
    run_m10(&buffer1, &mut buffer2);

    for i in 0..6 {
        buffer2[0xC0] = SHARED_DATA[i * 2];
        buffer2[0xC1] = SHARED_DATA[i * 2 + 1];
        buffer2[0xC2] = SHARED_DATA[0x12 + i * 3];
        buffer2[0xC3] = SHARED_DATA[0x12 + i * 3 + 1];
        buffer2[0xC4] = SHARED_DATA[0x12 + i * 3 + 2];

        let m9 = run_m9(&buffer2);
        dst[i * 8..i * 8 + 8].copy_from_slice(&m9[..8]);

        if i == 2 {
            buffer1[0] = 0;
            buffer1[1] = 0;
            buffer1[2] = 0;
            run_m10(&buffer1, &mut buffer2);
        }
    }
}

/// `Transform13.Execute`: 60 bytes → 60 bytes, via `Monolith10` then three `Monolith9`
/// passes with `SharedData` + `MagicPostfix` injected. Used in `SeedTransform`'s combine.
pub fn transform13(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= 60, "transform13 dst too small");
    assert!(src.len() >= 60, "transform13 src too small");

    // mask = [0xFFFFFFFF, 0xFFFFFFFF, 0x0000FFFF, src(15 dwords)] (Monolith10 source).
    let mut mask = [0u32; 18];
    mask[0] = 0xFFFF_FFFF;
    mask[1] = 0xFFFF_FFFF;
    mask[2] = 0x0000_FFFF;
    for j in 0..15 {
        mask[3 + j] = rd_u32(src, j);
    }

    let mut buffer2 = [0u32; M9_SRC_DWORDS];
    run_m10(&mask, &mut buffer2);

    for i in 6..9 {
        buffer2[0xC0] = SHARED_DATA[i * 2];
        buffer2[0xC1] = SHARED_DATA[i * 2 + 1];
        buffer2[0xC2] = MAGIC_POSTFIX[0];
        buffer2[0xC3] = MAGIC_POSTFIX[1];
        buffer2[0xC4] = MAGIC_POSTFIX[2];

        let m9 = run_m9(&buffer2);
        let dest_index = 0x18 * (i - 6);
        let copy = if i < 8 { 24 } else { 12 };
        dst[dest_index..dest_index + copy].copy_from_slice(&m9[..copy]);

        if i == 7 {
            mask[0] = 0;
            mask[1] = 0;
            mask[2] = 0;
            run_m10(&mask, &mut buffer2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn pre_seed_golden() {
        let src = include_bytes!("../../../tests/vectors/family0/transforms/transform1-src.bin");
        let expected =
            include_bytes!("../../../tests/vectors/family0/transforms/transform1-dst.bin");
        let mut dst = [0u8; 60];
        pre_seed(&mut dst, src);
        assert_eq!(&dst[..], &expected[..]);
    }

    // t1 = PreSeed(key1 = 0x35×24) dumped from the .NET oracle for the S71500 auth case;
    // matches the transform6-t1 (SeedTransform input) block repeated.
    #[test]
    fn pre_seed_s71500_key1() {
        let mut dst = [0u8; 60];
        pre_seed(&mut dst, &[0x35u8; 24]);
        assert_eq!(
            dst,
            hex!(
                "27bdbbd62dcc78a45425958de6e7194259edb80b5aa0b978"
                "27bdbbd62dcc78a45425958de6e7194259edb80b5aa0b978"
                "27bdbbd62dcc78a45425958d"
            ),
        );
    }

    #[test]
    fn key_derivation_golden() {
        let src = include_bytes!("../../../tests/vectors/family0/transforms/transform2-src.bin");
        let expected =
            include_bytes!("../../../tests/vectors/family0/transforms/transform2-dst.bin");
        let mut dst = [0u8; 48];
        key_derivation(&mut dst, src);
        assert_eq!(&dst[..], &expected[..]);
    }

    // KeyDerivation(PreSeed(key1=0x35×24)) must yield the exact challenge/checksum/lut keys
    // the .NET oracle produced for the S71500 blob (see cipher.rs fixtures).
    #[test]
    fn key_derivation_s71500_keys() {
        let mut t1 = [0u8; 60];
        pre_seed(&mut t1, &[0x35u8; 24]);
        let mut dst = [0u8; 48];
        key_derivation(&mut dst, &t1);
        assert_eq!(
            dst,
            hex!(
                "df85fc2e31ed70c4672f07350a02b768" // challengeKey
                "410755938896af2583a90fdf0f9e1bee" // checksumKey
                "0ddf4f10a200874232887bf3225736ce" // lutSeed
            ),
        );
    }

    #[test]
    fn transform13_golden() {
        let src = include_bytes!("../../../tests/vectors/family0/transforms/transform13-src.bin");
        let expected =
            include_bytes!("../../../tests/vectors/family0/transforms/transform13-dst.bin");
        let mut dst = [0u8; 60];
        transform13(&mut dst, src);
        assert_eq!(&dst[..], &expected[..]);
    }

    #[test]
    fn transform12_golden() {
        // TransformTests.ExecuteTransform12 cases: (index, count).
        for (tag, index, count) in [("0x0", 0x772A, 0x7E), ("0x29", 0xC2B0, 0x67)] {
            let mut ctx = match tag {
                "0x0" => include_bytes!(
                    "../../../tests/vectors/family0/transforms/transform12_0x0-ctx.bin"
                )
                .to_vec(),
                _ => include_bytes!(
                    "../../../tests/vectors/family0/transforms/transform12_0x29-ctx.bin"
                )
                .to_vec(),
            };
            let expected: &[u8] = match tag {
                "0x0" => include_bytes!(
                    "../../../tests/vectors/family0/transforms/transform12_0x0-ctx-expected.bin"
                ),
                _ => include_bytes!(
                    "../../../tests/vectors/family0/transforms/transform12_0x29-ctx-expected.bin"
                ),
            };
            transform12(&mut ctx, index, count);
            assert_eq!(&ctx[..], expected, "transform12 case {tag}");
        }
    }
}
