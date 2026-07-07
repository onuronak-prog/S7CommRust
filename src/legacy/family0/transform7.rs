// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Ported from `bonk-dev/HarpoS7` (MIT):
//   HarpoS7.Family0/Transforms/Transform7.cs
// See `LICENSE-HarpoS7`.

//! `Transform7`: the blinded fixed-base comb scalar-multiplication `k·source` (in the opaque
//! internal point format), driven by `prng1`/`prng2`. A faithful transcription of the C#
//! sequence of `Monolith{3,4,5,6,7}.WithCopy` + `Transform12` calls over a byte workspace.
//!
//! Byte offsets below are into the C# `workBuffer` (2400 B) and `ctx` (`Transform12` context,
//! 3576 B). Because Rust cannot alias one `&mut` buffer across a call's inputs and outputs, we
//! gather each `WithCopy` input into an owned array first, then write its outputs back.

use super::data::{TRANSFORM7_COUNTS, TRANSFORM7_DATA, TRANSFORM7_INDEXES};
use super::transforms::transform12;
use super::{field, monolith};

#[inline]
fn get<const N: usize>(b: &[u8], o: usize) -> [u8; N] {
    b[o..o + N].try_into().unwrap()
}

#[inline]
fn or_dword(buf: &mut [u8], off: usize) {
    let v = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) | 4;
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn wc3(s1: &[u8], s2: &[u8], s3: &[u8]) -> ([u8; 72], [u8; 72]) {
    let (mut o1, mut o2) = ([0u8; 72], [0u8; 72]);
    monolith::withcopy3(&mut o1, &mut o2, s1, s2, s3);
    (o1, o2)
}
fn wc4(s1: &[u8], s2: &[u8]) -> [u8; 72] {
    let mut o = [0u8; 72];
    monolith::withcopy4(&mut o, s1, s2);
    o
}
fn wc5(s1: &[u8], s2: &[u8], s3: &[u8]) -> ([u8; 24], [u8; 24]) {
    let (mut o1, mut o2) = ([0u8; 24], [0u8; 24]);
    monolith::withcopy5(&mut o1, &mut o2, s1, s2, s3);
    (o1, o2)
}
fn wc6(s1: &[u8], s2: &[u8], s3: &[u8]) -> ([u8; 72], [u8; 72]) {
    let (mut o1, mut o2) = ([0u8; 72], [0u8; 72]);
    monolith::withcopy6(&mut o1, &mut o2, s1, s2, s3);
    (o1, o2)
}
fn wc7(s1: &[u8], s2: &[u8]) -> ([u8; 72], [u8; 72]) {
    let (mut o1, mut o2) = ([0u8; 72], [0u8; 72]);
    monolith::withcopy7(&mut o1, &mut o2, s1, s2);
    (o1, o2)
}

/// Write a 72-byte block into `buf` at byte offset `o`.
#[inline]
fn p72(buf: &mut [u8], o: usize, v: &[u8; 72]) {
    buf[o..o + 72].copy_from_slice(v);
}
/// Write a 24-byte block into `buf` at byte offset `o`.
#[inline]
fn p24(buf: &mut [u8], o: usize, v: &[u8; 24]) {
    buf[o..o + 24].copy_from_slice(v);
}

/// `BigIntAddition.Execute(ctx[d], ctx[d], wb[s])` — add the 24-byte cell at `wb[s]` into `ctx[d]`.
fn add_into(ctx: &mut [u8], d: usize, wb: &[u8], s: usize) {
    let a = get::<24>(ctx, d);
    let b = get::<24>(wb, s);
    field::bigint_add(&mut ctx[d..d + 24], &a, &b);
}

/// `Transform7.Execute(destination[0x48], prng1[0x14], prng2[0x14], source[0x28])`.
pub fn transform7(destination: &mut [u8], prng1: &[u8], prng2: &[u8], source: &[u8]) {
    let mut wb = vec![0u8; 2400];
    let mut ctx = vec![0u8; 3576];
    let data = &TRANSFORM7_DATA[..];

    // Load prng1 (5 dwords) at dword 0xC (byte 0x30), source (5+5 dwords) at 0 and dword 0x12.
    wb[0x30..0x44].copy_from_slice(&prng1[..20]);
    or_dword(&mut wb, 0x30);
    wb[0..20].copy_from_slice(&source[..20]);
    wb[0x48..0x5C].copy_from_slice(&source[20..40]);
    or_dword(&mut wb, 0x48);

    // 1. M3(a8,60, data, data+48, wb+30)
    let (o1, o2) = wc3(&data[0..], &data[0x48..], &get::<24>(&wb, 0x30));
    p72(&mut wb, 0xA8, &o1);
    p72(&mut wb, 0x60, &o2);
    // 2. M3(2a0,210, data, data+48, wb+48)
    let (o1, o2) = wc3(&data[0..], &data[0x48..], &get::<24>(&wb, 0x48));
    p72(&mut wb, 0x2A0, &o1);
    p72(&mut wb, 0x210, &o2);
    // 3. M3(1c8,f0, wb+a8, wb+60, wb+0)
    let (o1, o2) = wc3(
        &get::<72>(&wb, 0xA8),
        &get::<72>(&wb, 0x60),
        &get::<24>(&wb, 0),
    );
    p72(&mut wb, 0x1C8, &o1);
    p72(&mut wb, 0xF0, &o2);
    // 4. M3(180,138, wb+1c8, wb+f0, wb+48)
    let (o1, o2) = wc3(
        &get::<72>(&wb, 0x1C8),
        &get::<72>(&wb, 0xF0),
        &get::<24>(&wb, 0x48),
    );
    p72(&mut wb, 0x180, &o1);
    p72(&mut wb, 0x138, &o2);
    // 5. M4(4e0, wb+180, wb+138)
    let o = wc4(&get::<72>(&wb, 0x180), &get::<72>(&wb, 0x138));
    p72(&mut wb, 0x4E0, &o);
    // 6. M5(ctx+450, wb+18, wb+4e0, wb+180, wb+138)
    let (o1, o2) = wc5(
        &get::<72>(&wb, 0x4E0),
        &get::<72>(&wb, 0x180),
        &get::<72>(&wb, 0x138),
    );
    p24(&mut ctx, 0x450, &o1);
    p24(&mut wb, 0x18, &o2);
    // 7. add
    add_into(&mut ctx, 0x450, &wb, 0x18);
    // 8. M3(498,768, wb+180, wb+138, wb+30)
    let (o1, o2) = wc3(
        &get::<72>(&wb, 0x180),
        &get::<72>(&wb, 0x138),
        &get::<24>(&wb, 0x30),
    );
    p72(&mut wb, 0x498, &o1);
    p72(&mut wb, 0x768, &o2);
    // 9. M4(648, data, data+48)
    let o = wc4(&data[0..], &data[0x48..]);
    p72(&mut wb, 0x648, &o);
    // 10. M6(8d0,888, wb+648, wb+a8, wb+60)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x648),
        &get::<72>(&wb, 0xA8),
        &get::<72>(&wb, 0x60),
    );
    p72(&mut wb, 0x8D0, &o1);
    p72(&mut wb, 0x888, &o2);
    // 11. M4(570, wb+498, wb+768)
    let o = wc4(&get::<72>(&wb, 0x498), &get::<72>(&wb, 0x768));
    p72(&mut wb, 0x570, &o);
    // 12. M6(330,690, wb+570, wb+2a0, wb+210)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x570),
        &get::<72>(&wb, 0x2A0),
        &get::<72>(&wb, 0x210),
    );
    p72(&mut wb, 0x330, &o1);
    p72(&mut wb, 0x690, &o2);
    // 13. M4(840, wb+a8, wb+60)
    let o = wc4(&get::<72>(&wb, 0xA8), &get::<72>(&wb, 0x60));
    p72(&mut wb, 0x840, &o);
    // 14. M6(7f8,6d8, wb+840, wb+1c8, wb+f0)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x840),
        &get::<72>(&wb, 0x1C8),
        &get::<72>(&wb, 0xF0),
    );
    p72(&mut wb, 0x7F8, &o1);
    p72(&mut wb, 0x6D8, &o2);
    // 15. M4(5b8, wb+7f8, wb+6d8)
    let o = wc4(&get::<72>(&wb, 0x7F8), &get::<72>(&wb, 0x6D8));
    p72(&mut wb, 0x5B8, &o);
    // 16. M6(3c0,450, wb+5b8, wb+2a0, wb+210)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x5B8),
        &get::<72>(&wb, 0x2A0),
        &get::<72>(&wb, 0x210),
    );
    p72(&mut wb, 0x3C0, &o1);
    p72(&mut wb, 0x450, &o2);
    // 17. M4(600, wb+3c0, wb+450)
    let o = wc4(&get::<72>(&wb, 0x3C0), &get::<72>(&wb, 0x450));
    p72(&mut wb, 0x600, &o);
    // 18. M5(ctx+690, wb+18, wb+600, wb+3c0, wb+450)
    let (o1, o2) = wc5(
        &get::<72>(&wb, 0x600),
        &get::<72>(&wb, 0x3C0),
        &get::<72>(&wb, 0x450),
    );
    p24(&mut ctx, 0x690, &o1);
    p24(&mut wb, 0x18, &o2);
    // 19. add
    add_into(&mut ctx, 0x690, &wb, 0x18);
    // 20. M4(378, wb+330, wb+690)
    let o = wc4(&get::<72>(&wb, 0x330), &get::<72>(&wb, 0x690));
    p72(&mut wb, 0x378, &o);
    // 21. M6(2e8,258, wb+378, wb+3c0, wb+450)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x378),
        &get::<72>(&wb, 0x3C0),
        &get::<72>(&wb, 0x450),
    );
    p72(&mut wb, 0x2E8, &o1);
    p72(&mut wb, 0x258, &o2);
    // 22. M4(408, wb+2e8, wb+258)
    let o = wc4(&get::<72>(&wb, 0x2E8), &get::<72>(&wb, 0x258));
    p72(&mut wb, 0x408, &o);
    // 23. M5(ctx+480, wb+18, wb+408, wb+2e8, wb+258)
    let (o1, o2) = wc5(
        &get::<72>(&wb, 0x408),
        &get::<72>(&wb, 0x2E8),
        &get::<72>(&wb, 0x258),
    );
    p24(&mut ctx, 0x480, &o1);
    p24(&mut wb, 0x18, &o2);
    // 24. add
    add_into(&mut ctx, 0x480, &wb, 0x18);
    // 25. RotateRight30(wb[..24])
    field::rotate_right30(&mut wb[..24]);
    // 26. M3(a8,60, data, data, wb+0)   [note: src2 = data, not data+48]
    let (o1, o2) = wc3(&data[0..], &data[0..], &get::<24>(&wb, 0));
    p72(&mut wb, 0xA8, &o1);
    p72(&mut wb, 0x60, &o2);
    // 27. M5(ctx+8d0, wb+18, data, wb+a8, wb+60)
    let (o1, o2) = wc5(&data[0..], &get::<72>(&wb, 0xA8), &get::<72>(&wb, 0x60));
    p24(&mut ctx, 0x8D0, &o1);
    p24(&mut wb, 0x18, &o2);
    // 28. add
    add_into(&mut ctx, 0x8D0, &wb, 0x18);

    // --- Transform12 comb loops ---
    for i in 0..0xA0u32 {
        let prng2_index = ((0x9F - i) >> 5) as usize;
        let p2w = u32::from_le_bytes(
            prng2[prng2_index * 4..prng2_index * 4 + 4]
                .try_into()
                .unwrap(),
        );
        let t = (((p2w >> ((0xFFFF_FFFFu32 - i) & 0x1F)) & 1) + i * 2) as usize;
        transform12(&mut ctx, TRANSFORM7_INDEXES[t], TRANSFORM7_COUNTS[t]);
    }
    for i in 0xA0u32..0xF9 {
        let didx = ((i - 0xA0) >> 5) as usize + 0xC;
        let wbw = u32::from_le_bytes(wb[didx * 4..didx * 4 + 4].try_into().unwrap());
        let t = (((wbw >> (i & 0x1F)) & 1) + i * 2) as usize;
        transform12(&mut ctx, TRANSFORM7_INDEXES[t], TRANSFORM7_COUNTS[t]);
    }

    // 29-32. PrepareFinalize on four ctx cells.
    field::prepare_finalize(&mut ctx[0x918..0x930]);
    field::prepare_finalize(&mut ctx[0x6A8..0x6C0]);
    field::prepare_finalize(&mut ctx[0x5B8..0x5D0]);
    field::prepare_finalize(&mut ctx[0x288..0x2A0]);

    // 33. M7(7b0,720, ctx+288, data+90)
    let (o1, o2) = wc7(&get::<24>(&ctx, 0x288), &data[0x90..]);
    p72(&mut wb, 0x7B0, &o1);
    p72(&mut wb, 0x720, &o2);
    // 34. M4(528, wb+7b0, wb+720)
    let o = wc4(&get::<72>(&wb, 0x7B0), &get::<72>(&wb, 0x720));
    p72(&mut wb, 0x528, &o);
    // 35. M7(a8,60, ctx+288, wb+528)
    let (o1, o2) = wc7(&get::<24>(&ctx, 0x288), &get::<72>(&wb, 0x528));
    p72(&mut wb, 0xA8, &o1);
    p72(&mut wb, 0x60, &o2);
    // 36. M4(2a0, wb+a8, wb+60)
    let o = wc4(&get::<72>(&wb, 0xA8), &get::<72>(&wb, 0x60));
    p72(&mut wb, 0x2A0, &o);
    // 37. M7(210,1c8, ctx+5b8, wb+2a0)
    let (o1, o2) = wc7(&get::<24>(&ctx, 0x5B8), &get::<72>(&wb, 0x2A0));
    p72(&mut wb, 0x210, &o1);
    p72(&mut wb, 0x1C8, &o2);
    // 38. M4(f0, wb+210, wb+1c8)
    let o = wc4(&get::<72>(&wb, 0x210), &get::<72>(&wb, 0x1C8));
    p72(&mut wb, 0xF0, &o);
    // 39. M7(180,138, ctx+918, wb+f0)
    let (o1, o2) = wc7(&get::<24>(&ctx, 0x918), &get::<72>(&wb, 0xF0));
    p72(&mut wb, 0x180, &o1);
    p72(&mut wb, 0x138, &o2);
    // 40. M7(4e0,498, ctx+6a8, wb+f0)
    let (o1, o2) = wc7(&get::<24>(&ctx, 0x6A8), &get::<72>(&wb, 0xF0));
    p72(&mut wb, 0x4E0, &o1);
    p72(&mut wb, 0x498, &o2);
    // 41. M4(330, wb+180, wb+138)
    let o = wc4(&get::<72>(&wb, 0x180), &get::<72>(&wb, 0x138));
    p72(&mut wb, 0x330, &o);
    // 42. M6(2e8,258, wb+330, wb+a8, wb+60)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x330),
        &get::<72>(&wb, 0xA8),
        &get::<72>(&wb, 0x60),
    );
    p72(&mut wb, 0x2E8, &o1);
    p72(&mut wb, 0x258, &o2);
    // 43. M6(378,408, wb+528, wb+2e8, wb+258)
    let (o1, o2) = wc6(
        &get::<72>(&wb, 0x528),
        &get::<72>(&wb, 0x2E8),
        &get::<72>(&wb, 0x258),
    );
    p72(&mut wb, 0x378, &o1);
    p72(&mut wb, 0x408, &o2);
    // 44. M4(destination, wb+378, wb+408)
    let o = wc4(&get::<72>(&wb, 0x378), &get::<72>(&wb, 0x408));
    destination[..72].copy_from_slice(&o);
}

#[cfg(test)]
mod tests {
    #[test]
    fn transform7_golden() {
        let prng1 =
            include_bytes!("../../../tests/vectors/family0/transforms/transform7-prng1.bin");
        let prng2 =
            include_bytes!("../../../tests/vectors/family0/transforms/transform7-prng2.bin");
        let src = include_bytes!("../../../tests/vectors/family0/transforms/transform7-src.bin");
        let expected =
            include_bytes!("../../../tests/vectors/family0/transforms/transform7-dst.bin");
        let mut dst = [0u8; 72];
        super::transform7(&mut dst, prng1, prng2, src);
        assert_eq!(&dst[..], &expected[..]);
    }
}
