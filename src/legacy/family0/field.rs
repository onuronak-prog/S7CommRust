// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Ported from `bonk-dev/HarpoS7` (MIT):
//   HarpoS7.Family0/BitOperations/BigIntOperations.cs
// See `LICENSE-HarpoS7`.

//! The Family-0 field-representation codec: `BigIntOperations`.
//!
//! The Monoliths operate on field elements of `GF(p)`, `p = 2^160 - 47`, held in a
//! **packed radix-2^30** form: six little-endian `u32` limbs, each carrying a 30-bit
//! digit shifted up by 2 (i.e. the digit lives in bits `[2..31]`, low two bits zero).
//! The full value is `sum(digit_i * 2^(30*i))` for `i in 0..6`.
//!
//! [`prepare`] converts that packed form into the canonical little-endian **160-bit**
//! form (five `u32` limbs), folding any excess `>= 2^160` back with the Solinas factor
//! `47` (since `2^160 ≡ 47 (mod p)`). [`finalize`] is the inverse repack. These two are
//! the boundary between the packed Monolith world and ordinary integer arithmetic.
//!
//! This is a faithful transliteration of the decompiled C#: every `+`/`*` is unchecked
//! `u32` wrapping, and the odd-looking carry/shift expressions are reproduced verbatim
//! (they are validated byte-for-byte against HarpoS7's `BitOperations` golden vectors).

/// `PrepareSourceSize` — packed input to [`prepare`] (6 × u32).
pub const PREPARE_SRC_LEN: usize = 6 * 4;
/// `PrepareDestinationSize` — canonical 160-bit output of [`prepare`] (5 × u32).
pub const PREPARE_DST_LEN: usize = 5 * 4;
/// `FinalizeSourceSize` — canonical 160-bit input to [`finalize`] (5 × u32).
pub const FINALIZE_SRC_LEN: usize = 5 * 4;
/// `FinalizeDestinationSize` — packed output of [`finalize`] (6 × u32).
pub const FINALIZE_DST_LEN: usize = 6 * 4;

#[inline]
fn rd(buf: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([buf[i * 4], buf[i * 4 + 1], buf[i * 4 + 2], buf[i * 4 + 3]])
}

#[inline]
fn wr(buf: &mut [u8], i: usize, v: u32) {
    buf[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
}

/// `CarryHelper(a, b)` — 1 if `a < b` else 0.
#[inline]
fn carry(a: u32, b: u32) -> u32 {
    (a < b) as u32
}

/// `BigIntOperations.Prepare`: packed radix-2^30 (6 limbs) → canonical 160-bit (5 limbs).
///
/// Panics if `dst < 20` or `src < 24` bytes.
pub fn prepare(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= PREPARE_DST_LEN, "prepare dst too small");
    assert!(src.len() >= PREPARE_SRC_LEN, "prepare src too small");
    let s = |i: usize| rd(src, i);

    let mut d = [0u32; 5];

    let mut temp0 = s(0);
    let mut temp1 = (s(1) << 0x1A).wrapping_add(temp0 >> 2);
    d[0] = temp1;

    temp1 = (carry(temp1, temp0) >> 2).wrapping_add(s(1) >> 6);
    temp0 = s(2).wrapping_mul(0x40_0000).wrapping_add(temp1);
    d[1] = temp0;

    temp1 = carry(temp0, temp1).wrapping_add(s(2) >> 10);
    temp0 = s(3).wrapping_mul(0x4_0000).wrapping_add(temp1);
    d[2] = temp0;

    temp1 = carry(temp0, temp1).wrapping_add(s(3) >> 0xe);
    temp0 = s(4).wrapping_mul(0x4000).wrapping_add(temp1);
    d[3] = temp0;

    temp1 = carry(temp0, temp1).wrapping_add(s(4) >> 0x12);
    temp0 = s(5).wrapping_mul(0x400).wrapping_add(temp1);
    d[4] = temp0;

    temp0 = carry(temp0, temp1)
        .wrapping_add(s(5) >> 0x16)
        .wrapping_mul(0x2f);

    if temp0 != 0 {
        d[0] = d[0].wrapping_add(temp0);
        d[1] = d[1].wrapping_add(carry(d[0], temp0));

        temp0 = carry(d[1], carry(d[0], temp0));
        d[2] = d[2].wrapping_add(temp0);

        temp0 = carry(d[2], temp0);
        d[3] = d[3].wrapping_add(temp0);

        temp0 = carry(d[3], temp0);
        d[4] = d[4].wrapping_add(temp0);

        d[0] = d[0].wrapping_add(carry(d[4], temp0).wrapping_mul(0x2f));
    }

    for (i, &v) in d.iter().enumerate() {
        wr(dst, i, v);
    }
}

/// `BigIntOperations.Finalize`: canonical 160-bit (up to 5 limbs) → packed radix-2^30 (6 limbs).
///
/// `src` may be shorter than 20 bytes; it is zero-padded (matching the C# `realSource`).
/// Panics if `dst < 24` bytes.
pub fn finalize(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= FINALIZE_DST_LEN, "finalize dst too small");

    let mut real = [0u8; FINALIZE_SRC_LEN];
    let n = src.len().min(FINALIZE_SRC_LEN);
    real[..n].copy_from_slice(&src[..n]);
    let s = |i: usize| rd(&real, i);

    wr(dst, 0, (s(0) & 0x0FFF_FFFF) << 2);
    wr(dst, 1, (s(1) << 0x06 | s(0) >> 0x1A) & 0x3FFF_FFFC);
    wr(dst, 2, (s(2) << 0x0A | s(1) >> 0x16) & 0x3FFF_FFFC);
    wr(dst, 3, (s(2) >> 0x12 | s(3) << 0x0E) & 0x3FFF_FFFC);
    wr(dst, 4, (s(4) << 0x12 | s(3) >> 0x0E) & 0x3FFF_FFFC);
    wr(dst, 5, s(4) >> 0x0A & 0x003F_FFFC);
}

/// `BigIntOperations.PrepareFinalize`: [`prepare`] then [`finalize`] fused, in place over
/// the packed 6-limb buffer. Panics if `buf < 24` bytes.
pub fn prepare_finalize(buf: &mut [u8]) {
    assert!(
        buf.len() >= FINALIZE_DST_LEN,
        "prepare_finalize buf too small"
    );
    let d = |i: usize| rd(buf, i);

    let temp0_0 = d(1).wrapping_mul(0x400_0000).wrapping_add(d(0) >> 2);
    let mut temp1 = carry(temp0_0, d(0) >> 2).wrapping_add(d(1) >> 6);

    let temp2_0 = d(2).wrapping_mul(0x40_0000).wrapping_add(temp1);
    temp1 = carry(temp2_0, temp1).wrapping_add(d(2) >> 10);

    let temp3_0 = d(3).wrapping_mul(0x4_0000).wrapping_add(temp1);
    temp1 = carry(temp3_0, temp1).wrapping_add(d(3) >> 0xe);

    let temp4_0 = d(4).wrapping_mul(0x4000).wrapping_add(temp1);
    temp1 = carry(temp4_0, temp1).wrapping_add(d(4) >> 0x12);

    let temp5_0 = d(5).wrapping_mul(0x400).wrapping_add(temp1);
    temp1 = carry(temp5_0, temp1)
        .wrapping_add(d(5) >> 0x16)
        .wrapping_mul(0x2f);

    let mut temp0 = temp0_0;
    let mut temp2 = temp2_0;
    let mut temp3 = temp3_0;
    let mut temp4 = temp4_0;
    let mut temp5 = temp5_0;

    if temp1 != 0 {
        let mut temp6 = carry(temp0.wrapping_add(temp1), temp1);

        temp2 = temp2.wrapping_add(temp6);
        temp6 = carry(temp2, temp6);

        temp3 = temp3.wrapping_add(temp6);
        temp6 = carry(temp3, temp6);

        temp4 = temp4.wrapping_add(temp6);
        temp6 = carry(temp4, temp6);

        temp5 = temp5.wrapping_add(temp6);
        temp0 = temp0
            .wrapping_add(temp1)
            .wrapping_add(carry(temp5, temp6).wrapping_mul(0x2f));
    }

    wr(buf, 0, (temp0 & 0x0fff_ffff) << 2);
    wr(buf, 1, (temp2 << 6 | temp0 >> 0x1a) & 0x3fff_fffc);
    wr(buf, 2, (temp3 << 10 | temp2 >> 0x16) & 0x3fff_fffc);
    wr(buf, 3, (temp4 << 0xe | temp3 >> 0x12) & 0x3fff_fffc);
    wr(buf, 4, (temp5 << 0x12 | temp4 >> 0xe) & 0x3fff_fffc);
    wr(buf, 5, temp5 >> 10 & 0x003f_fffc);
}

/// `BigIntOperations.RotateLeft31`: rotate the 4-limb (128-bit) value with the
/// curve's feedback polynomial (used as the AES-keystream IV chain in the RealPlc blob).
/// Panics if `buf < 16` bytes.
pub fn rotate_left31(buf: &mut [u8]) {
    assert!(buf.len() >= 16, "rotate_left31 buf too small");
    let mut d = [rd(buf, 0), rd(buf, 1), rd(buf, 2), rd(buf, 3)];
    let first = d[0];

    for i in 0..3 {
        d[i] = d[i + 1] << 0x1F | d[i] >> 1;
    }
    let mask = if first & 1 != 0 { 0xFFFF_FFFFu32 } else { 0 };
    d[3] = (d[3] >> 1) ^ (mask & 0xE100_0000);

    for (i, &v) in d.iter().enumerate() {
        wr(buf, i, v);
    }
}

/// `BigIntOperations.RotateRight30`: the 6-limb counterpart (used inside `Transform7`).
/// Panics if `buf < 24` bytes.
pub fn rotate_right30(buf: &mut [u8]) {
    assert!(
        buf.len() >= FINALIZE_DST_LEN,
        "rotate_right30 buf too small"
    );
    let mut d = [
        rd(buf, 0),
        rd(buf, 1),
        rd(buf, 2),
        rd(buf, 3),
        rd(buf, 4),
        rd(buf, 5),
    ];

    d[5] = d[4] >> 0x1E;
    for i in (1..=4).rev() {
        d[i] = d[i - 1] >> 0x1E | d[i] << 2;
    }
    d[0] <<= 2;

    for (i, &v) in d.iter().enumerate() {
        wr(buf, i, v);
    }
}

// ---------------------------------------------------------------------------
// Field arithmetic over GF(p), p = 2^160 - 47.
//
// These mirror `HarpoS7.Family0/Transforms/BigInt{Addition,Subtraction,
// Multiplication,Square}.cs`: [`prepare`] each packed 24-byte input to the
// canonical 160-bit form, do the integer op, apply the transform's specific
// mod-p reduction (Solinas fold factor 47), then [`finalize`] back to packed.
//
// The reduction is transliterated *exactly* (it is not always fully canonical —
// a residue can land in the `[p, 2^160)` gap — so a "clean" mod-p would not be
// byte-identical). Validated against the `transform8/9/10/11` golden vectors.
// ---------------------------------------------------------------------------

/// [`prepare`] a packed 24-byte input directly into five little-endian limbs.
pub fn prepare_limbs(src: &[u8]) -> [u32; 5] {
    let mut dst = [0u8; PREPARE_DST_LEN];
    prepare(&mut dst, src);
    [
        rd(&dst, 0),
        rd(&dst, 1),
        rd(&dst, 2),
        rd(&dst, 3),
        rd(&dst, 4),
    ]
}

#[inline]
fn limbs_to_bytes(limbs: &[u32; 5]) -> [u8; FINALIZE_SRC_LEN] {
    let mut out = [0u8; FINALIZE_SRC_LEN];
    for (i, &v) in limbs.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    out
}

/// `(a + b) mod p` in canonical 160-bit limbs, matching `BigIntAddition`.
fn add_limbs(a: &[u32; 5], b: &[u32; 5]) -> [u32; 5] {
    let mut sum = [0u32; 6];
    let mut c = 0u64;
    for i in 0..5 {
        let s = a[i] as u64 + b[i] as u64 + c;
        sum[i] = s as u32;
        c = s >> 32;
    }
    sum[5] = c as u32;

    let mut s = [sum[0], sum[1], sum[2], sum[3], sum[4]];
    // length > 20 bytes  <=>  value >= 2^160  <=>  the 6th limb is set.
    if sum[5] != 0 {
        s[0] = s[0].wrapping_add(0x2F);
        let mut carry = (s[0] < 0x2F) as u32;
        if carry != 0 {
            // Faithful to the C#: propagates through limbs 1..=3 only, never limb 4.
            for item in s.iter_mut().take(4).skip(1) {
                let added = carry;
                *item = item.wrapping_add(added);
                carry = (*item < added) as u32;
            }
            if carry != 0 {
                s[0] = s[0].wrapping_add(0x5E);
            }
        }
    }
    s
}

/// `(a - b) mod p` in canonical 160-bit limbs, matching `BigIntSubtraction`
/// (on borrow: subtract an extra 47 and take the 160-bit two's complement).
fn sub_limbs(a: &[u32; 5], b: &[u32; 5]) -> [u32; 5] {
    let mut r = [0u32; 5];
    let mut borrow = 0u32;
    for i in 0..5 {
        let (v0, o0) = a[i].overflowing_sub(b[i]);
        let (v1, o1) = v0.overflowing_sub(borrow);
        r[i] = v1;
        borrow = (o0 || o1) as u32;
    }
    if borrow != 0 {
        // r == a - b + 2^160; subtract a further 47 (mod 2^160).
        let mut b2;
        let (v, o) = r[0].overflowing_sub(0x2F);
        r[0] = v;
        b2 = o as u32;
        for item in r.iter_mut().skip(1) {
            let (v, o) = item.overflowing_sub(b2);
            *item = v;
            b2 = o as u32;
        }
    }
    r
}

/// One `BigIntegerCompressor.Compress` step over a 10-limb product: if the value
/// is `>= 2^160`, replace it with `(value >> 160) * 47 + (value mod 2^160)`.
/// Returns whether the *result* still needs compressing (`>= 2^160`).
fn compress(v: &mut [u32; 10]) -> bool {
    if !v[5..].iter().any(|&x| x != 0) {
        return false;
    }
    // t = high(limbs 5..10) * 47   (fits in 6 limbs; high < 2^160 => t < 2^166)
    let mut t = [0u64; 6];
    let mut carry = 0u64;
    for i in 0..5 {
        let p = v[5 + i] as u64 * 47 + carry;
        t[i] = p & 0xFFFF_FFFF;
        carry = p >> 32;
    }
    t[5] = carry;
    // result = t + low(limbs 0..5)
    let mut c = 0u64;
    let mut res = [0u32; 6];
    for i in 0..6 {
        let lo = if i < 5 { v[i] as u64 } else { 0 };
        let s = t[i] + lo + c;
        res[i] = s as u32;
        c = s >> 32;
    }
    v[..6].copy_from_slice(&res);
    for x in v[6..].iter_mut() {
        *x = 0;
    }
    v[5] != 0
}

/// `(a * b) mod p` in canonical 160-bit limbs, matching `BigIntMultiplication`
/// (schoolbook 5×5 → 10 limbs, then up to two `Compress` + a `FinalCompress`).
fn mul_limbs(a: &[u32; 5], b: &[u32; 5]) -> [u32; 5] {
    let mut prod = [0u32; 10];
    for i in 0..5 {
        let mut carry = 0u64;
        for j in 0..5 {
            let s = prod[i + j] as u64 + a[i] as u64 * b[j] as u64 + carry;
            prod[i + j] = s as u32;
            carry = s >> 32;
        }
        prod[i + 5] = (prod[i + 5] as u64 + carry) as u32;
    }

    if compress(&mut prod) && compress(&mut prod) {
        // FinalCompress: add 47 to limb 0 with NO carry propagation (byte-level wrap).
        prod[0] = prod[0].wrapping_add(0x2F);
    }
    [prod[0], prod[1], prod[2], prod[3], prod[4]]
}

/// `BigIntAddition.Execute`: `dst := (s1 + s2) mod p`, packed 24-byte in/out.
pub fn bigint_add(dst: &mut [u8], s1: &[u8], s2: &[u8]) {
    let r = add_limbs(&prepare_limbs(s1), &prepare_limbs(s2));
    finalize(dst, &limbs_to_bytes(&r));
}

/// `BigIntSubtraction.Execute`: `dst := (minuend - subtrahend) mod p`.
pub fn bigint_sub(dst: &mut [u8], minuend: &[u8], subtrahend: &[u8]) {
    let r = sub_limbs(&prepare_limbs(minuend), &prepare_limbs(subtrahend));
    finalize(dst, &limbs_to_bytes(&r));
}

/// `BigIntMultiplication.Execute`: `dst := (s1 * s2) mod p`.
pub fn bigint_mul(dst: &mut [u8], s1: &[u8], s2: &[u8]) {
    let r = mul_limbs(&prepare_limbs(s1), &prepare_limbs(s2));
    finalize(dst, &limbs_to_bytes(&r));
}

/// `BigIntSquare.Execute`: `dst := (src^2) mod p`.
pub fn bigint_square(dst: &mut [u8], src: &[u8]) {
    let a = prepare_limbs(src);
    let r = mul_limbs(&a, &a);
    finalize(dst, &limbs_to_bytes(&r));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden vectors captured from HarpoS7.Family0.Tests/Blobs/BitOperations
    // (bonk-dev/HarpoS7, MIT).
    const PREP_SRC: &[u8] = include_bytes!("../../../tests/vectors/family0/bitops/prep_src.bin");
    const PREP_DST: &[u8] = include_bytes!("../../../tests/vectors/family0/bitops/prep_dst.bin");
    const FINAL_SRC: &[u8] =
        include_bytes!("../../../tests/vectors/family0/bitops/finalize_src.bin");
    const FINAL_DST: &[u8] =
        include_bytes!("../../../tests/vectors/family0/bitops/finalize_dst.bin");
    const MIXED_SRC: &[u8] = include_bytes!("../../../tests/vectors/family0/bitops/mixed_src.bin");
    const MIXED_DST: &[u8] = include_bytes!("../../../tests/vectors/family0/bitops/mixed_dst.bin");

    #[test]
    fn prepare_golden() {
        let mut dst = [0u8; PREPARE_DST_LEN];
        prepare(&mut dst, PREP_SRC);
        assert_eq!(&dst[..], PREP_DST);
    }

    #[test]
    fn finalize_golden() {
        let mut dst = [0u8; FINALIZE_DST_LEN];
        finalize(&mut dst, FINAL_SRC);
        assert_eq!(&dst[..], FINAL_DST);
    }

    #[test]
    fn prepare_finalize_golden() {
        let mut buf = MIXED_SRC.to_vec();
        prepare_finalize(&mut buf);
        assert_eq!(&buf[..], MIXED_DST);
    }

    // Field arithmetic vs the BigInt transform golden vectors (transform8/9/10/11).
    macro_rules! vec_bin {
        ($n:literal) => {
            include_bytes!(concat!("../../../tests/vectors/family0/transforms/", $n))
        };
    }

    fn check_binop(f: fn(&mut [u8], &[u8], &[u8]), s1: &[u8], s2: &[u8], expected: &[u8]) {
        let mut dst = [0u8; FINALIZE_DST_LEN];
        f(&mut dst, s1, s2);
        assert_eq!(&dst[..], expected);
    }

    #[test]
    fn bigint_add_golden() {
        check_binop(
            bigint_add,
            vec_bin!("transform8-source1.bin"),
            vec_bin!("transform8-source2.bin"),
            vec_bin!("transform8-dst.bin"),
        );
        check_binop(
            bigint_add,
            vec_bin!("transform8_2-source1.bin"),
            vec_bin!("transform8_2-source2.bin"),
            vec_bin!("transform8_2-dst.bin"),
        );
    }

    #[test]
    fn bigint_mul_golden() {
        check_binop(
            bigint_mul,
            vec_bin!("transform9-source1.bin"),
            vec_bin!("transform9-source2.bin"),
            vec_bin!("transform9-dst.bin"),
        );
        check_binop(
            bigint_mul,
            vec_bin!("transform9_2-source1.bin"),
            vec_bin!("transform9_2-source2.bin"),
            vec_bin!("transform9_2-dst.bin"),
        );
        // transform9_3: "Resulting BigInteger is 19 bytes long, not 20" — exercises the short-result path.
        check_binop(
            bigint_mul,
            vec_bin!("transform9_3-source1.bin"),
            vec_bin!("transform9_3-source2.bin"),
            vec_bin!("transform9_3-dst.bin"),
        );
    }

    #[test]
    fn bigint_sub_golden() {
        check_binop(
            bigint_sub,
            vec_bin!("transform11-source1.bin"),
            vec_bin!("transform11-source2.bin"),
            vec_bin!("transform11-dst.bin"),
        );
        check_binop(
            bigint_sub,
            vec_bin!("transform11_2-source1.bin"),
            vec_bin!("transform11_2-source2.bin"),
            vec_bin!("transform11_2-dst.bin"),
        );
        check_binop(
            bigint_sub,
            vec_bin!("transform11_3-source1.bin"),
            vec_bin!("transform11_3-source2.bin"),
            vec_bin!("transform11_3-dst.bin"),
        );
        check_binop(
            bigint_sub,
            vec_bin!("transform11_4-source1.bin"),
            vec_bin!("transform11_4-source2.bin"),
            vec_bin!("transform11_4-dst.bin"),
        );
    }

    #[test]
    fn bigint_square_golden() {
        let mut dst = [0u8; FINALIZE_DST_LEN];
        bigint_square(&mut dst, vec_bin!("transform10-src.bin"));
        assert_eq!(&dst[..], vec_bin!("transform10-dst.bin"));
        bigint_square(&mut dst, vec_bin!("transform10_2-src.bin"));
        assert_eq!(&dst[..], vec_bin!("transform10_2-dst.bin"));
    }

    // Inline cases from BigIntOperationsTests.RotateLeft31Test.
    #[test]
    fn rotate_left31_cases() {
        let mut a = [0x25u8; 16];
        rotate_left31(&mut a);
        let mut e1 = [0x92u8; 16];
        e1[15] = 0xF3;
        assert_eq!(a, e1);

        // second case chains from the first's expected output
        let mut b = e1;
        rotate_left31(&mut b);
        let mut e2 = [0x49u8; 16];
        e2[14] = 0xC9;
        e2[15] = 0x79;
        assert_eq!(b, e2);
    }
}
