// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors

//! The Family-0 "Monolith" bit-sliced boolean transforms.
//!
//! [`nine`] (`Monolith9`, 788→24 bytes) and [`ten`] (`Monolith10`, 72→768 bytes) are large,
//! branch-free boolean circuits — a keyed compression and a table expansion — used by
//! `PreSeedTransform` / `KeyDerivationTransform` / `Transform13` (they are *not* the elliptic
//! curve; that lives in [`super::ec`]). They are not any standard algorithm and are nonlinear,
//! so they are transcribed verbatim from the decompiled C# (`^ & ~ >> << *2` only) rather than
//! reimplemented. Both files are machine-generated (`scratchpad/convert_monolith.ps1`) and
//! validated byte-for-byte against HarpoS7's golden vectors.

pub mod m1;
pub mod m11;
pub mod m2;
pub mod m3;
pub mod m4;
pub mod m5;
pub mod m6;
pub mod m7;
pub mod m8;
pub mod nine;
pub mod ten;

// The `WithCopy` wrappers: concatenate the inputs into the monolith's source, run `execute`,
// then split its output. Used throughout `Transform7`.

/// `Monolith3.WithCopy(d1, d2, s1[0x48], s2[0x48], s3[0x18])`.
pub fn withcopy3(d1: &mut [u8], d2: &mut [u8], s1: &[u8], s2: &[u8], s3: &[u8]) {
    let mut s = [0u8; 168];
    s[..0x48].copy_from_slice(&s1[..0x48]);
    s[0x48..0x90].copy_from_slice(&s2[..0x48]);
    s[0x90..0xA8].copy_from_slice(&s3[..0x18]);
    let mut d = [0u8; 144];
    m3::execute(&mut d, &s);
    d1[..0x48].copy_from_slice(&d[..0x48]);
    d2[..0x48].copy_from_slice(&d[0x48..0x90]);
}

/// `Monolith4.WithCopy(dst, s1[0x48], s2[0x48])` — single output.
pub fn withcopy4(dst: &mut [u8], s1: &[u8], s2: &[u8]) {
    let mut s = [0u8; 144];
    s[..0x48].copy_from_slice(&s1[..0x48]);
    s[0x48..0x90].copy_from_slice(&s2[..0x48]);
    m4::execute(dst, &s);
}

/// `Monolith5.WithCopy(d1, d2, s1[0x48], s2[0x48], s3[0x48])`.
pub fn withcopy5(d1: &mut [u8], d2: &mut [u8], s1: &[u8], s2: &[u8], s3: &[u8]) {
    let mut s = [0u8; 216];
    s[..0x48].copy_from_slice(&s1[..0x48]);
    s[0x48..0x90].copy_from_slice(&s2[..0x48]);
    s[0x90..0xD8].copy_from_slice(&s3[..0x48]);
    let mut d = [0u8; 48];
    m5::execute(&mut d, &s);
    d1[..0x18].copy_from_slice(&d[..0x18]);
    d2[..0x18].copy_from_slice(&d[0x18..0x30]);
}

/// `Monolith6.WithCopy(d1, d2, s1[0x48], s2[0x48], s3[0x48])`.
pub fn withcopy6(d1: &mut [u8], d2: &mut [u8], s1: &[u8], s2: &[u8], s3: &[u8]) {
    let mut s = [0u8; 216];
    s[..0x48].copy_from_slice(&s1[..0x48]);
    s[0x48..0x90].copy_from_slice(&s2[..0x48]);
    s[0x90..0xD8].copy_from_slice(&s3[..0x48]);
    let mut d = [0u8; 144];
    m6::execute(&mut d, &s);
    d1[..0x48].copy_from_slice(&d[..0x48]);
    d2[..0x48].copy_from_slice(&d[0x48..0x90]);
}

/// `Monolith7.WithCopy(d1, d2, s1[0x18], s2[0x48])`.
pub fn withcopy7(d1: &mut [u8], d2: &mut [u8], s1: &[u8], s2: &[u8]) {
    let mut s = [0u8; 96];
    s[..0x18].copy_from_slice(&s1[..0x18]);
    s[0x18..0x60].copy_from_slice(&s2[..0x48]);
    let mut d = [0u8; 144];
    m7::execute(&mut d, &s);
    d1[..0x48].copy_from_slice(&d[..0x48]);
    d2[..0x48].copy_from_slice(&d[0x48..0x90]);
}

#[cfg(test)]
mod tests {
    macro_rules! golden {
        ($modname:ident, $n:literal, $dstlen:literal) => {{
            let src = include_bytes!(concat!(
                "../../../../tests/vectors/family0/monoliths/monolith",
                $n,
                "-src.bin"
            ));
            let expected = include_bytes!(concat!(
                "../../../../tests/vectors/family0/monoliths/monolith",
                $n,
                "-dst.bin"
            ));
            let mut dst = [0u8; $dstlen];
            super::$modname::execute(&mut dst, src);
            assert_eq!(&dst[..], &expected[..]);
        }};
    }

    #[test]
    fn monolith1_golden() {
        let src = include_bytes!("../../../../tests/vectors/family0/monoliths/monolith1-src.bin");
        let expected =
            include_bytes!("../../../../tests/vectors/family0/monoliths/monolith1-dst.bin");
        let mut dst = [0u8; 72];
        let r = super::m1::execute(&mut dst, src);
        assert_eq!(r, 0);
        assert_eq!(&dst[..], &expected[..]);
    }

    #[test]
    fn monolith2_golden() {
        golden!(m2, "2", 20);
    }

    #[test]
    fn monolith3_golden() {
        golden!(m3, "3", 144);
    }

    #[test]
    fn monolith4_golden() {
        golden!(m4, "4", 72);
    }

    #[test]
    fn monolith5_golden() {
        golden!(m5, "5", 48);
    }

    #[test]
    fn monolith6_golden() {
        golden!(m6, "6", 144);
    }

    #[test]
    fn monolith7_golden() {
        golden!(m7, "7", 144);
    }

    #[test]
    fn monolith8_golden() {
        golden!(m8, "8", 60);
    }

    #[test]
    fn monolith11_golden() {
        golden!(m11, "11", 20);
    }

    macro_rules! wc {
        ($f:literal) => {
            include_bytes!(concat!("../../../../tests/vectors/family0/withcopy/", $f))
        };
    }

    #[test]
    fn withcopy3_golden() {
        let (mut d1, mut d2) = ([0u8; 72], [0u8; 72]);
        super::withcopy3(
            &mut d1,
            &mut d2,
            wc!("3_in1.bin"),
            wc!("3_in2.bin"),
            wc!("3_in3.bin"),
        );
        assert_eq!(&d1[..], wc!("3_out1.bin"));
        assert_eq!(&d2[..], wc!("3_out2.bin"));
    }

    #[test]
    fn withcopy4_golden() {
        let mut d = [0u8; 72];
        super::withcopy4(&mut d, wc!("4_in1.bin"), wc!("4_in2.bin"));
        assert_eq!(&d[..], wc!("4_out1.bin"));
    }

    #[test]
    fn withcopy5_golden() {
        let (mut d1, mut d2) = ([0u8; 24], [0u8; 24]);
        super::withcopy5(
            &mut d1,
            &mut d2,
            wc!("5_in1.bin"),
            wc!("5_in2.bin"),
            wc!("5_in3.bin"),
        );
        assert_eq!(&d1[..], wc!("5_out1.bin"));
        assert_eq!(&d2[..], wc!("5_out2.bin"));
    }

    #[test]
    fn withcopy6_golden() {
        let (mut d1, mut d2) = ([0u8; 72], [0u8; 72]);
        super::withcopy6(
            &mut d1,
            &mut d2,
            wc!("6_in1.bin"),
            wc!("6_in2.bin"),
            wc!("6_in3.bin"),
        );
        assert_eq!(&d1[..], wc!("6_out1.bin"));
        assert_eq!(&d2[..], wc!("6_out2.bin"));
    }

    #[test]
    fn withcopy7_golden() {
        let (mut d1, mut d2) = ([0u8; 72], [0u8; 72]);
        super::withcopy7(&mut d1, &mut d2, wc!("7_in1.bin"), wc!("7_in2.bin"));
        assert_eq!(&d1[..], wc!("7_out1.bin"));
        assert_eq!(&d2[..], wc!("7_out2.bin"));
    }

    #[test]
    fn monolith10_golden() {
        let src = include_bytes!("../../../../tests/vectors/family0/monoliths/monolith10-src.bin");
        let expected =
            include_bytes!("../../../../tests/vectors/family0/monoliths/monolith10-dst.bin");
        let mut dst = [0u8; 768];
        super::ten::execute(&mut dst, src);
        assert_eq!(&dst[..], &expected[..]);
    }

    #[test]
    fn monolith9_golden() {
        let src = include_bytes!("../../../../tests/vectors/family0/monoliths/monolith9-src.bin");
        let expected =
            include_bytes!("../../../../tests/vectors/family0/monoliths/monolith9-dst.bin");
        let mut dst = [0u8; 24];
        super::nine::execute(&mut dst, src);
        assert_eq!(&dst[..], &expected[..]);
    }
}
