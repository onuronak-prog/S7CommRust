// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from bonk-dev/HarpoS7 `HarpoS7/Seed/HarpoSeedUtilities.cs` (MIT, (c) 2024 bonk).

//! The encrypted-seed elliptic-curve arithmetic for the PLCSIM (family-03) auth path.
//!
//! The decompiled HarpoS7 source operates over NIST **P-256** (`seed_consts::MASK` is the
//! P-256 prime, `MASK3` is `a = p - 3`). The reference threads computation through
//! overlapping byte/qword spans; this port instead uses value-typed 4-limb field elements,
//! which is clearer and side-steps Rust's aliasing rules while producing identical results.
//! Each function is validated against HarpoS7's own unit-test vectors.

#![allow(clippy::needless_range_loop)]

use super::aes::HarpoAesCtr;
use super::keys::derive_seed_encryption_key_and_iv;
use super::seed_consts::{MASK, MASK2, MASK3, SEED1};

/// A field element / 256-bit value as four little-endian 64-bit limbs.
type Fe = [u64; 4];

/// `PubKeyOp5`: `output[0..size] += a2 * data[0..size]` with carry-in `a4_in`; returns the
/// out-carry. A schoolbook multiply-accumulate over 64-bit limbs (32×32→64 split).
fn op5(output: &mut [u64], a2: u64, data: &[u64], a4_in: u64, size: usize) -> u64 {
    let mut a4 = a4_in;
    let mut v2 = 0usize;
    while v2 < size {
        let v3lo = data[v2] & 0xffff_ffff;
        let v4 = data[v2] >> 0x20;
        let v5 = v3lo.wrapping_mul(a2 >> 0x20);
        let v6 = v3lo.wrapping_mul(a2 & 0xffff_ffff).wrapping_add(a4);
        let v3 = v4.wrapping_mul(a2 & 0xffff_ffff).wrapping_add(v5);
        let v7 = v6.wrapping_add(v3 << 0x20);
        let new = output[v2].wrapping_add(v7);

        let t1 = (v3 < v5) as u64;
        let t2 = (v6 < a4) as u64;
        let t3 = (new < v7) as u64;
        let t4 = (v7 < (v3 << 0x20)) as u64;

        output[v2] = new;
        a4 = (t1.wrapping_neg() & 0x1_0000_0000)
            .wrapping_add(t2)
            .wrapping_add(v4.wrapping_mul(a2 >> 0x20))
            .wrapping_add(v3 >> 0x20)
            .wrapping_add(t3)
            .wrapping_add(t4);
        v2 += 1;
    }
    a4
}

/// A 32-bit "dword" of a limb array viewed little-endian.
fn dword(a: &[u64], k: usize) -> u64 {
    if k & 1 == 0 {
        a[k / 2] & 0xffff_ffff
    } else {
        a[k / 2] >> 0x20
    }
}

/// `Op7Math`: OR one selected dword (or its high half) into `a1[i]`.
fn op7_math(a1: &mut [u64; 4], i: usize, a2: &[u64], const_var: u32, shift_high: bool) {
    if const_var != 0 {
        let t1 = if const_var & 1 == 0 {
            dword(a2, ((const_var >> 1) * 2) as usize)
        } else {
            a2[(const_var >> 1) as usize] >> 0x20
        };
        a1[i] |= if shift_high { t1 << 0x20 } else { t1 };
    }
}

/// `PubKeyOp7`: gather eight selected dwords of the 8-limb product `a2` into a 4-limb value.
fn op7(a2: &[u64], idx: [u32; 8]) -> Fe {
    let mut a1 = [0u64; 4];
    op7_math(&mut a1, 3, a2, idx[0], true);
    op7_math(&mut a1, 3, a2, idx[1], false);
    op7_math(&mut a1, 2, a2, idx[2], true);
    op7_math(&mut a1, 2, a2, idx[3], false);
    op7_math(&mut a1, 1, a2, idx[4], true);
    op7_math(&mut a1, 1, a2, idx[5], false);
    op7_math(&mut a1, 0, a2, idx[6], true);
    op7_math(&mut a1, 0, a2, idx[7], false);
    a1
}

/// `PubKeyOp8`: 256-bit add; returns `(sum, carry_out)`.
fn op8(a2: Fe, a3: Fe) -> (Fe, u64) {
    let mut out = [0u64; 4];
    let v1 = a3[0].wrapping_add(a2[0]);
    out[0] = v1;
    let mut of = (a2[0] > v1) as u64;
    for index in 1..4 {
        let v2 = a2[index];
        let v3 = a3[index];
        let v4 = v3.wrapping_add(of).wrapping_add(v2);
        out[index] = v4;
        of = (v3.wrapping_add(of) < of || v4 < v2) as u64;
    }
    (out, of)
}

/// `PubKeyOp9` (the `a4 == 0` form used by the PLCSIM path): 256-bit subtract; returns
/// `(diff, borrow_out)`.
fn op9(a2: Fe, a3: Fe) -> (Fe, u64) {
    let mut out = [0u64; 4];
    let t1 = a2[0];
    let v1 = t1.wrapping_sub(a3[0]);
    out[0] = v1;
    let mut v2 = (t1 < v1) as u64;
    for index in 1..4 {
        let v3 = a2[index];
        v2 = v3.wrapping_sub(v2);
        let v4 = a3[index];
        let diff = v2.wrapping_sub(v4);
        out[index] = diff;
        v2 = (v3 < v2 || v2 < diff) as u64;
    }
    (out, v2)
}

/// `SeedMaskSub2` (size 4): compare two 256-bit values. Returns 1 if `a1 > mask`, 0 if equal,
/// -1 if `a1 < mask`.
fn seed_mask_sub2(a1: &[u64], mask: &[u64]) -> i32 {
    let mut size = 3usize;
    loop {
        if a1[size] != mask[size] {
            break;
        }
        if size == 0 {
            break;
        }
        size -= 1;
    }
    if a1[size] > mask[size] {
        1
    } else if mask[size] > a1[size] {
        -1
    } else {
        0
    }
}

/// `ZeroCheck32`: true iff the first 4 limbs (32 bytes) are all zero.
fn zero_check32(data: &[u64]) -> bool {
    data[..4].iter().all(|&x| x == 0)
}

/// `ZeroCheck64`: true iff the first 8 limbs (64 bytes) are all zero.
fn zero_check64(data: &[u64]) -> bool {
    data[..8].iter().all(|&x| x == 0)
}

/// `PubKeyOp6`: reduce the 8-limb product `a2` modulo the P-256 prime to 4 limbs (the
/// Solinas fast reduction, with a final small multiple-of-p correction via `MASK2`).
fn op6(a2: &[u64]) -> Fe {
    let mut l1 = op7(a2, [0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0, 0, 0]);
    let l2g = op7(a2, [0x00, 0x0F, 0x0E, 0x0D, 0x0C, 0, 0, 0]);
    let (s, l4a) = op8(l1, l2g);
    l1 = s;
    let mut l2 = [a2[0], a2[1], a2[2], a2[3]];
    let mut l5: i64 = if l4a != 0 { 2 } else { 0 };

    // l1 <<= 1, carry out into l6.
    let mut l6 = 0u64;
    for i in 0..4 {
        let t1 = l1[i];
        l1[i] = t1.wrapping_mul(2) | l6;
        l6 = (t1 >> 63) & 1;
    }

    let (s, l4b) = op8(l2, l1);
    l2 = s;
    let (s, l11) = op8(l2, op7(a2, [0x0F, 0x0E, 0, 0, 0, 0x0A, 0x09, 0x08]));
    l2 = s;
    let (s, l7) = op8(
        l2,
        op7(a2, [0x08, 0x0D, 0x0F, 0x0E, 0x0D, 0x0B, 0x0A, 0x09]),
    );
    l2 = s;
    let (s, l8) = op9(l2, op7(a2, [0x0A, 0x08, 0, 0, 0, 0x0D, 0x0C, 0x0B]));
    l2 = s;
    let (s, l9) = op9(l2, op7(a2, [0x0B, 0x09, 0, 0, 0x0F, 0x0E, 0x0D, 0x0C]));
    l2 = s;
    let (s, l10) = op9(l2, op7(a2, [0x0C, 0, 0x0A, 0x09, 0x08, 0x0F, 0x0E, 0x0D]));
    l2 = s;
    let (mut dest, l12) = op9(l2, op7(a2, [0x0D, 0, 0x0B, 0x0A, 0x09, 0, 0x0F, 0x0E]));

    l5 += l4b as i64 + l6 as i64 + l11 as i64 + l7 as i64
        - l8 as i64
        - l9 as i64
        - l10 as i64
        - l12 as i64;
    if l5 != 0 {
        let m = MASK2[l5.unsigned_abs() as usize - 1];
        let carry = if l5 < 1 {
            let (s, b) = op9(dest, m);
            dest = s;
            b
        } else {
            let (s, c) = op8(dest, m);
            dest = s;
            c
        };
        if carry != 0 {
            dest = op9(dest, MASK2[0]).0;
        }
    }
    if seed_mask_sub2(&dest, &MASK) > -1 {
        dest = op9(dest, MASK).0;
    }
    dest
}

/// `PubKeyOp4`: modular square (`a2^2 mod p`). Builds the 512-bit square (off-diagonal terms
/// doubled, then diagonal squares) and reduces via [`op6`].
fn op4(a2: &[u64]) -> Fe {
    let mut v1 = [0u64; 9];

    // Off-diagonal partial products a2[i]*a2[j], i<j.
    let (mut read, mut write, mut v3, mut a2i) = (1usize, 4usize, 0i32, 0usize);
    loop {
        let v2 = op5(
            &mut v1[read..],
            a2[a2i],
            &a2[a2i + 1..],
            0,
            (3 - v3) as usize,
        );
        v3 += 1;
        a2i += 1;
        read += 2;
        v1[write] = v2;
        write += 1;
        if v3 >= 3 {
            break;
        }
    }

    // Double the off-diagonal sum (v1[1..7] <<= 1).
    v1[7] = v1[6] >> 0x3F;
    let (mut index, mut read, mut write) = (6, 5usize, 6usize);
    loop {
        let v4 = v1[read];
        read = read.wrapping_sub(1);
        v1[write] = v1[write].wrapping_mul(2) | (v4 >> 0x3F);
        write = write.wrapping_sub(1);
        index -= 1;
        if index == 0 {
            break;
        }
    }

    // Add the diagonal squares a2[i]^2.
    let (mut v5, mut write, mut a2i, mut index) = (0u64, 1usize, 0usize, 4);
    loop {
        let v6 = op5(&mut v1[write - 1..], a2[a2i], &a2[a2i..], v5, 1);
        a2i += 1;
        v1[write] = v1[write].wrapping_add(v6);
        v5 = (v1[write] < v6) as u64;
        write += 2;
        index -= 1;
        if index == 0 {
            break;
        }
    }

    op6(&v1[..8])
}

/// `PubKeyOp12`: modular multiply (`a2 * a3 mod p`).
fn op12(a2: Fe, a3: Fe) -> Fe {
    let mut v1 = [0u64; 10];
    for i in 0..4 {
        let v2 = op5(&mut v1[i..], a2[i], &a3, 0, 4);
        v1[i + 4] = v2;
    }
    op6(&v1[..8])
}

/// `SeedMask`: reduce a 256-bit value modulo `p` (repeated conditional subtract).
fn seed_mask(a2: Fe) -> Fe {
    let mut d = a2;
    while seed_mask_sub2(&d, &MASK) > -1 {
        d = op9(d, MASK).0;
    }
    d
}

/// `PubKeyOp15`: modular subtract (`a2 - a3 mod p`).
fn op15(a2: Fe, a3: Fe) -> Fe {
    let (d, borrow) = op9(a2, a3);
    if borrow != 0 {
        op8(d, MASK).0
    } else {
        d
    }
}

/// `PubKeyOp16`: modular add (`a2 + a3 mod p`).
fn op16(a2: Fe, a3: Fe) -> Fe {
    let (d, carry) = op8(a2, a3);
    if carry != 0 || seed_mask_sub2(&d, &MASK) > -1 {
        op9(d, MASK).0
    } else {
        d
    }
}

/// `PubKeyOp17` for the `(mask1=1, mask2=p)` case used by the affine conversion: the modular
/// inverse `a^-1 mod p`. The reference uses a binary GCD; since the field is P-256 we compute
/// the (identical) value by Fermat's little theorem, `a^(p-2) mod p`.
fn op17(a: Fe) -> Fe {
    const P_MINUS_2: [u64; 4] = [
        0xFFFF_FFFF_FFFF_FFFD,
        0x0000_0000_FFFF_FFFF,
        0x0000_0000_0000_0000,
        0xFFFF_FFFF_0000_0001,
    ];
    let mut result: Fe = [1, 0, 0, 0];
    for limb in (0..4).rev() {
        for bit in (0..64).rev() {
            result = op4(&result);
            if (P_MINUS_2[limb] >> bit) & 1 == 1 {
                result = op12(result, a);
            }
        }
    }
    result
}

/// `SeedFunction3`: P-256 point doubling. Maps point `(a4,a5,a6)` to `2·(a4,a5,a6)`, returned
/// as `(a1,a2,a3)`. A zero `a5`/`a6` yields the identity (`a3 = 0`).
fn seed_function3(a4: Fe, a5: Fe, a6: Fe) -> (Fe, Fe, Fe) {
    if zero_check32(&seed_mask(a6)) || zero_check32(&seed_mask(a5)) {
        return (a4, a5, [0, 0, 0, 0]);
    }
    let mut m2 = op4(&a6);
    let mut m1 = op4(&m2);
    m1 = op12(MASK3, m1);
    let mut m0 = op4(&a4);
    let m3 = op16(m0, m0);
    m0 = op16(m3, m0);
    m0 = op16(m0, m1);
    let mut a3 = op12(a5, a6);
    a3 = op16(a3, a3);
    m2 = op4(&a5);
    let mut a2 = op12(m2, m2);
    for _ in 0..3 {
        a2 = op16(a2, a2);
    }
    m2 = op12(m2, a4);
    for _ in 0..2 {
        m2 = op16(m2, m2);
    }
    m1 = op4(&m0);
    let mut a1 = op15(m1, m2);
    a1 = op15(a1, m2);
    m1 = op15(m2, a1);
    m1 = op12(m0, m1);
    a2 = op15(m1, a2);
    (a1, a2, a3)
}

/// `SeedFunction4`: P-256 point addition. Adds base point `(a7,a8)` to `(a4,a5,a6)`. Handles
/// the identity (`a6 = 0`), the equal-point (double) and opposite-point (identity) cases.
fn seed_function4(a4: Fe, a5: Fe, a6: Fe, a7: Fe, a8: Fe) -> (Fe, Fe, Fe) {
    if zero_check32(&a6) {
        return (a7, a8, [1, 0, 0, 0]);
    }
    let acc0 = op4(&a6);
    let v1 = seed_mask(op12(acc0, a7));
    if seed_mask_sub2(&v1, &seed_mask(a4)) == 0 {
        let v1b = seed_mask(op12(op12(a6, acc0), a8));
        if seed_mask_sub2(&v1b, &seed_mask(a5)) == 0 {
            return seed_function3(a4, a5, a6);
        }
        return (a4, a5, [0, 0, 0, 0]);
    }

    let mut v2 = op12(op12(acc0, a6), a8);
    let acc = op15(v1, a4);
    v2 = op15(v2, a5);
    let a3 = op12(a6, acc);
    let v1c = op4(&acc);
    let v3 = op12(a4, v1c);
    let mut a1 = op12(acc, v1c);
    let mut a2 = op12(a5, a1);
    a1 = op16(a1, v3);
    a1 = op16(a1, v3);
    let acc2 = op4(&v2);
    a1 = op15(acc2, a1);
    let acc3 = op15(v3, a1);
    let acc4 = op12(v2, acc3);
    a2 = op15(acc4, a2);
    (a1, a2, a3)
}

/// `SeedFunction5`: convert the Jacobian point `(X=a2, Y=a3, Z=a4)` to affine. Returns 9
/// limbs: `x = X/Z^2` (`[0..4]`), `y = Y/Z^3` (`[4..8]`), and a 0 flag (`[8]`).
fn seed_function5(a2: Fe, a3: Fe, a4: Fe) -> [u64; 9] {
    let mut out = [0u64; 9];
    let v1 = seed_mask(a4);
    if !zero_check32(&v1) {
        let y_zinv = op12(a3, v1);
        let inv = op17(v1);
        let inv2 = op4(&inv);
        let x = seed_mask(op12(inv2, a2));
        let inv4 = op4(&inv2);
        let y = seed_mask(op12(inv4, y_zinv));
        out[0..4].copy_from_slice(&x);
        out[4..8].copy_from_slice(&y);
    }
    out
}

/// `SeedFunction2`: scalar multiplication `a3 · base`. `base` is 9 limbs (affine X/Y plus a
/// flag); the result is the affine product (9 limbs). Left-to-right binary method.
fn seed_function2(base: &[u64], scalar: Fe) -> [u64; 9] {
    if base[8] & 0xffff_ffff == 1 {
        let mut out = [0u64; 9];
        out[8] = 1;
        return out;
    }
    let base_x = [base[0], base[1], base[2], base[3]];
    let base_y = [base[4], base[5], base[6], base[7]];
    let (mut x, mut y, mut z) = ([0u64; 4], [0u64; 4], [0u64; 4]); // identity
    for limb in (0..4).rev() {
        for bit in (0..64).rev() {
            let (dx, dy, dz) = seed_function3(x, y, z);
            x = dx;
            y = dy;
            z = dz;
            if (scalar[limb] >> bit) & 1 == 1 {
                let (sx, sy, sz) = seed_function4(x, y, z, base_x, base_y);
                x = sx;
                y = sy;
                z = sz;
            }
        }
    }
    seed_function5(x, y, z)
}

/// Full byte-reverse 32 bytes, then read as a little-endian field element
/// (`Span<byte>.ReverseBytes` for a 32-byte span).
fn reverse_bytes_to_fe(b: &[u8]) -> Fe {
    let mut rev = [0u8; 32];
    for i in 0..32 {
        rev[i] = b[31 - i];
    }
    fe_from_le(&rev)
}

/// 32 little-endian bytes -> field element.
fn fe_from_le(b: &[u8]) -> Fe {
    [
        u64::from_le_bytes(b[0..8].try_into().unwrap()),
        u64::from_le_bytes(b[8..16].try_into().unwrap()),
        u64::from_le_bytes(b[16..24].try_into().unwrap()),
        u64::from_le_bytes(b[24..32].try_into().unwrap()),
    ]
}

/// 9 limbs -> 72 little-endian bytes.
fn limbs9_to_bytes(v: &[u64; 9]) -> [u8; 72] {
    let mut out = [0u8; 72];
    for (i, &limb) in v.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&limb.to_le_bytes());
    }
    out
}

/// `OmsReverseRows`: byte-reverse the two 32-byte halves of `public_key` into a 9-limb point
/// buffer (flag 0). Returns `None` if the result is all zero.
fn oms_reverse_rows(public_key: &[u8]) -> Option<[u64; 9]> {
    let mut dest = [0u64; 9];
    dest[0..4].copy_from_slice(&reverse_bytes_to_fe(&public_key[0..32]));
    dest[4..8].copy_from_slice(&reverse_bytes_to_fe(&public_key[32..64]));
    if zero_check64(&dest) {
        None
    } else {
        Some(dest)
    }
}

/// `GenerateEncryptedSeed` (PLCSIM path): the 96-byte encrypted-seed blob fragment. The base
/// point is scalar-multiplied by an ephemeral random scalar (`fill_random`, a CSPRNG in
/// production), the result encrypts the challenge key, and an auth tag is appended. Returns
/// `None` only for an all-zero public key.
pub fn generate_encrypted_seed(
    public_key: &[u8],
    challenge_encryption_key: &[u8],
    fill_random: &mut dyn FnMut(&mut [u8]),
) -> Option<[u8; 96]> {
    let start = oms_reverse_rows(public_key)?;

    let mut l1 = [0u8; 32];
    let (v3, v4) = loop {
        fill_random(&mut l1);
        l1.reverse();
        let scalar = fe_from_le(&l1);
        let v3 = seed_function2(&SEED1, scalar);
        let v4 = seed_function2(&start, scalar);
        // Retry on a point-at-infinity result or a zero x-coordinate.
        if v3[8] & 0xffff_ffff == 1 || v4[8] & 0xffff_ffff == 1 || zero_check32(&v4) {
            continue;
        }
        break (v3, v4);
    };

    let mut destination = [0u8; 96];
    let v3_bytes = limbs9_to_bytes(&v3);
    for i in 0..32 {
        destination[i] = v3_bytes[31 - i];
        destination[32 + i] = v3_bytes[63 - i];
    }

    let v4_bytes = limbs9_to_bytes(&v4);
    let key_iv = derive_seed_encryption_key_and_iv(&v4_bytes[0..32], &destination[0..64]);

    let mut aes = HarpoAesCtr::new(&key_iv[0..16]);
    aes.init(&key_iv[32..48]);
    aes.encrypt_ctr(&challenge_encryption_key[0..16], &mut destination[64..80]);
    aes.calculate_checksum(&mut destination[80..96]);

    Some(destination)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    /// Little-endian bytes -> Vec<u64>.
    fn u64s(b: &[u8]) -> Vec<u64> {
        b.chunks_exact(8)
            .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
            .collect()
    }

    /// Little-endian 32 bytes -> a field element.
    fn fe(b: &[u8]) -> Fe {
        let v = u64s(b);
        [v[0], v[1], v[2], v[3]]
    }

    // All vectors from HarpoS7 `HarpoSeedUtilitiesTests.cs` (MIT).
    #[test]
    fn op5_matches_harpos7() {
        let a3 = u64s(&hex!(
            "7a879789bae6c926e4315324d7b516ddaa3ef0dd99d7a6ece9397f6971d28e90"
            "03c9eb0fc154c3d63bc4d5a4fb832443235208189822e4530000000000000000"
        ));
        let mut out = [0u64; 8];
        let r = op5(&mut out, 0x7394_F328_6532_A6B0, &a3, 0, 3);
        assert_eq!(r, 0x6AD8_A416_79EC_9E7D);
        assert_eq!(
            &out[..3],
            &u64s(&hex!("e03fda719236f1b4ae349f923cec8e199d0541e4b7203e68"))[..],
        );
    }

    #[test]
    fn op7_matches_harpos7() {
        let a2 = u64s(&hex!(
            "00b948698c38c7cb90ba4a3c4596119e814fdcd2727645aaddd31c8b641f0b9b"
            "d8ee0fafa1ba739fbad9c2de8b1aef4dba2f007af92d0fe060965202400ac4da"
        ));
        let got = op7(&a2, [0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0, 0, 0]);
        assert_eq!(
            got,
            fe(&hex!(
                "0000000000000000000000008b1aef4dba2f007af92d0fe060965202400ac4da"
            )),
        );
    }

    #[test]
    fn op8_matches_harpos7() {
        let a1 = fe(&hex!(
            "0000000000000000000000008b1aef4dba2f007af92d0fe060965202400ac4da"
        ));
        let a3 = fe(&hex!(
            "000000000000000000000000ba2f007af92d0fe060965202400ac4da00000000"
        ));
        let (out, carry) = op8(a1, a3);
        assert_eq!(carry, 0);
        assert_eq!(
            out,
            fe(&hex!(
                "000000000000000000000000454aefc7b35d0f5a5ac461e2a0a016dd400ac4da"
            )),
        );
    }

    #[test]
    fn op9_matches_harpos7() {
        let a2 = fe(&hex!(
            "7962ccb7e8ccfd49d7aefc68c958ff0d4aa14d896709cd4979d9ab27ff2c67da"
        ));
        let a3 = fe(&hex!(
            "8b1aef4dba2f007af92d0fe0000000000000000000000000d8ee0fafbad9c2de"
        ));
        let (out, borrow) = op9(a2, a3);
        assert_eq!(borrow, 1);
        assert_eq!(
            out,
            fe(&hex!(
                "ee47dd692e9dfdcfdd80ed88c858ff0d4aa14d896709cd49a1ea9b784453a4fb"
            )),
        );
    }

    #[test]
    fn seed_mask_sub2_matches_harpos7() {
        use super::super::seed_consts::MASK;
        let data = fe(&hex!(
            "eaa37e6fd0806c979dbd62d6ae669c572c3c1ff894d6cf1d37ff34fcc585c69f"
        ));
        assert_eq!(seed_mask_sub2(&data, &MASK), -1);

        let a1 = fe(&hex!(
            "8fdb644d685be30286f119bee740bdff2df747561ca11e0d0100000000000000"
        ));
        let a2 = fe(&hex!(
            "a583933ae6a05fd756708c66696eb3f1df61754f277b7d0c0100000000000000"
        ));
        assert_eq!(seed_mask_sub2(&a1, &a2), 1);
    }

    #[test]
    fn zero_checks() {
        assert!(zero_check32(&[0, 0, 0, 0]));
        assert!(!zero_check32(&[0, 0, 1, 0]));
        assert!(zero_check64(&[0; 8]));
    }

    #[test]
    fn op4_matches_harpos7() {
        assert_eq!(
            op4(&fe(&hex!(
                "b0a6326528f394737a879789bae6c926e4315324d7b516ddaa3ef0dd99d7a6ec"
            ))),
            fe(&hex!(
                "dc537b0d94ced7123ce0d6ab0da5b7e4ec0c170b21151b1dfe2f28d906dba553"
            )),
        );
        assert_eq!(
            op4(&fe(&hex!(
                "4f054f5ce327f40804457cc6c0aeeb972d14b15d652354e8681da4f204fd6b1b"
            ))),
            fe(&hex!(
                "f1200a5710c90ed28621e9a7e84bb8e02ab5b2c59414f109bcf02a9904c7b388"
            )),
        );
    }

    #[test]
    fn op6_matches_harpos7() {
        let prod = u64s(&hex!(
            "00b948698c38c7cb90ba4a3c4596119e814fdcd2727645aaddd31c8b641f0b9b"
            "d8ee0fafa1ba739fbad9c2de8b1aef4dba2f007af92d0fe060965202400ac4da"
        ));
        assert_eq!(
            op6(&prod),
            fe(&hex!(
                "dc537b0d94ced7123ce0d6ab0da5b7e4ec0c170b21151b1dfe2f28d906dba553"
            )),
        );
    }

    #[test]
    fn op12_matches_harpos7() {
        let a2 = fe(&hex!(
            "45b60f87f0b374fb82d74a9089402a26f9a74d5ee6445208b5eed14362757d57"
        ));
        let a3 = fe(&hex!(
            "96c298d84539a1f4a033eb2d817d0377f240a463e5e6bcf847422ce1f2d1176b"
        ));
        assert_eq!(
            op12(a2, a3),
            fe(&hex!(
                "ef66b7a68a375cbc28480e9c03a2b13b37ca1b3c3445caecae41191ff053fc1b"
            )),
        );
    }

    #[test]
    fn seed_mask_matches_harpos7() {
        // Both inputs are already < p, so reduction is the identity (matches HarpoS7).
        let a = fe(&hex!(
            "0fc7d69183847d1ecfdd614f27424b8043defcdc527d0e57adb5d1ac598f979a"
        ));
        assert_eq!(seed_mask(a), a);
        let b = fe(&hex!(
            "ef66b7a68a375cbc28480e9c03a2b13b37ca1b3c3445caecae41191ff053fc1b"
        ));
        assert_eq!(seed_mask(b), b);
    }

    #[test]
    fn op17_is_modular_inverse() {
        // a * a^-1 == 1 (mod p): exercises op17 (Fermat) together with op12/op4/op6.
        let cases = [
            [7u64, 0, 0, 0],
            [1, 0, 0, 0],
            fe(&hex!(
                "96c298d84539a1f4a033eb2d817d0377f240a463e5e6bcf847422ce1f2d1176b"
            )),
        ];
        for a in cases {
            assert_eq!(op12(a, op17(a)), [1, 0, 0, 0]);
        }
    }

    #[test]
    fn seed_function5_matches_harpos7() {
        let a2 = fe(&hex!(
            "5434424d4655e0ef66858b9558d8e370f33b12424e17a37e75cfc9efab46881e"
        ));
        let a3 = fe(&hex!(
            "696f49bc229ee62771c7f6da91af9c0142612f992b6d95c7d9cea8bce3b02006"
        ));
        let a4 = fe(&hex!(
            "5ee998a8eff083193aad75bf3eead7ee8e8b5ca3981060d6c1bde3fe17b93f3e"
        ));
        assert_eq!(
            &seed_function5(a2, a3, a4)[..],
            &u64s(&hex!(
                "fe02dc30066c7759245bfe5852c8b8f1523d90ed5384fcce64d9a6b4e024f118"
                "25f34b70d287c2c38ada731c0ba1ccd7eb69ae3700a528a71e0bd1d682b0d0f6"
                "0000000000000000"
            ))[..],
        );
    }

    #[test]
    fn seed_function2_scalar_mult_matches_harpos7() {
        let scalar = [0xDDDD_DDDD_DDDD_DDDDu64; 4];

        let base1 = u64s(&hex!(
            "96c298d84539a1f4a033eb2d817d0377f240a463e5e6bcf847422ce1f2d1176b"
            "f551bf376840b6cbce5e316b5733ce2b169e0f7c4aebe78e9b7f1afee242e34f"
            "0000000000000000"
        ));
        assert_eq!(
            &seed_function2(&base1, scalar)[..],
            &u64s(&hex!(
                "fe02dc30066c7759245bfe5852c8b8f1523d90ed5384fcce64d9a6b4e024f118"
                "25f34b70d287c2c38ada731c0ba1ccd7eb69ae3700a528a71e0bd1d682b0d0f6"
                "0000000000000000"
            ))[..],
        );

        let base2 = u64s(&hex!(
            "b0a6326528f394737a879789bae6c926e4315324d7b516ddaa3ef0dd99d7a6ec"
            "e9397f6971d28e9003c9eb0fc154c3d63bc4d5a4fb832443235208189822e453"
            "0000000000000000"
        ));
        assert_eq!(
            &seed_function2(&base2, scalar)[..],
            &u64s(&hex!(
                "b3420d0c6242b150d6862a4d61559e78a00da5dc7b68551ad86df007aba5bbd9"
                "948377e4448feff69f288a014c0832b241ed17b17c7ec69cd7d9efbffa692391"
                "0000000000000000"
            ))[..],
        );
    }

    #[test]
    fn generate_encrypted_seed_matches_harpos7() {
        // HarpoS7 GenerateEncryptedSeedTest, whose mocked PRNG fills the scalar with 0x33.
        let public_key = hex!(
            "eca6d799ddf03eaadd16b5d7245331e4"
            "26c9e6ba8997877a7394f3286532a6b0"
            "53e4229818085223432483fba4d5c43b"
            "d6c354c10febc903908ed271697f39e9"
        );
        let seed_key = hex!("4e001016db625dcce9105bdcd8a1b42c");
        let mut fill = |b: &mut [u8]| b.fill(0x33);

        let got = generate_encrypted_seed(&public_key, &seed_key, &mut fill).unwrap();

        assert_eq!(
            got,
            hex!(
                "51a7580833898ea1b183cbd7350a4099"
                "078c6ef1c1e18e970cd7683035f25e7d"
                "0110522712b0b5a7cff081685486984a"
                "94e6831edac46e7360fa9d834a7a81a1"
                "436d6fe9ae2435c7de4a8234d810f5aa"
                "2e5f612c49d50559f8da6502572b3add"
            ),
        );
    }
}
