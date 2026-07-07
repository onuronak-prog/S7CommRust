// SPDX-License-Identifier: LGPL-3.0-or-later
// AUTO-GENERATED from HarpoS7 Seed/SeedConsts.cs (MIT, (c) 2024 bonk). Do not edit by hand.
// Field = NIST P-256. MASK = prime p; MASK3 = a (= p-3); MASK2 = small multiples of p used in reduction.
// MASK4 (= 1) is part of the SeedConsts set but unused here (the inverse uses Fermat, not the
// reference binary-GCD that consumed it).
#![allow(dead_code)]

pub const MASK: [u64; 4] = [
    0xFFFFFFFFFFFFFFFF,
    0x00000000FFFFFFFF,
    0x0000000000000000,
    0xFFFFFFFF00000001,
];
pub const MASK3: [u64; 4] = [
    0xFFFFFFFFFFFFFFFC,
    0x00000000FFFFFFFF,
    0x0000000000000000,
    0xFFFFFFFF00000001,
];
pub const MASK4: [u64; 4] = [
    0x0000000000000001,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
];
pub const SEED1: [u64; 9] = [
    0xF4A13945D898C296,
    0x77037D812DEB33A0,
    0xF8BCE6E563A440F2,
    0x6B17D1F2E12C4247,
    0xCBB6406837BF51F5,
    0x2BCE33576B315ECE,
    0x8EE7EB4A7C0F9E16,
    0x4FE342E2FE1A7F9B,
    0x0000000000000000,
];
pub const MASK2: [[u64; 4]; 6] = [
    [
        0x0000000000000001,
        0xFFFFFFFF00000000,
        0xFFFFFFFFFFFFFFFF,
        0x00000000FFFFFFFEu64,
    ],
    [
        0x0000000000000002,
        0xFFFFFFFE00000000,
        0xFFFFFFFFFFFFFFFF,
        0x00000001FFFFFFFDu64,
    ],
    [
        0x0000000000000003,
        0xFFFFFFFD00000000,
        0xFFFFFFFFFFFFFFFF,
        0x00000002FFFFFFFCu64,
    ],
    [
        0x0000000000000004,
        0xFFFFFFFC00000000,
        0xFFFFFFFFFFFFFFFF,
        0x00000003FFFFFFFBu64,
    ],
    [
        0x0000000000000005,
        0xFFFFFFFB00000000,
        0xFFFFFFFFFFFFFFFF,
        0x00000004FFFFFFFAu64,
    ],
    [
        0x0000000000000006,
        0xFFFFFFFA00000000,
        0xFFFFFFFFFFFFFFFF,
        0x00000005FFFFFFF9u64,
    ],
];
