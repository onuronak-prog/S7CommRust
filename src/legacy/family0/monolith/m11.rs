// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// GENERATED -- do not edit by hand. Mechanically transcribed from bonk-dev/HarpoS7
// (MIT) HarpoS7.Family0/Monolith11liths/Monolith11.cs by scratchpad/convert_named.ps1: a
// straight-line named-local circuit (only ^ & ~(->!) >> << *2(-><<1)). Validated
// byte-for-byte against the Monolith11-src/dst golden vector. See LICENSE-HarpoS7.
#![allow(clippy::all)]
#![allow(
    non_snake_case,
    unused_parens,
    unused_assignments,
    unused_variables,
    unused_mut
)]
pub fn execute(destination: &mut [u8], source: &[u8]) {
    let srcDwords: Vec<u32> = source
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    let mut dstDwords = [0u32; 5];
    let mut uVar1: u32 = 0;
    let mut uVar2: u32 = 0;
    let mut uVar3: u32 = 0;
    let mut uVar4: u32 = 0;
    let mut uVar5: u32 = 0;
    let mut uVar6: u32 = 0;
    let mut uVar7: u32 = 0;
    let mut uVar8: u32 = 0;
    let mut uVar9: u32 = 0;
    let mut uVar10: u32 = 0;
    let mut uVar11: u32 = 0;
    let mut uVar12: u32 = 0;
    let mut uVar13: u32 = 0;
    let mut uVar14: u32 = 0;
    let mut uVar15: u32 = 0;
    let mut uVar16: u32 = 0;
    let mut uVar17: u32 = 0;
    let mut uVar18: u32 = 0;
    let mut uVar19: u32 = 0;
    let mut uVar20: u32 = 0;
    let mut uVar21: u32 = 0;
    let mut uVar22: u32 = 0;
    let mut uVar23: u32 = 0;
    let mut uVar24: u32 = 0;
    let mut uVar25: u32 = 0;
    let mut uVar26: u32 = 0;
    let mut uVar27: u32 = 0;
    let mut uVar28: u32 = 0;
    let mut uVar29: u32 = 0;
    let mut uVar30: u32 = 0;
    let mut uVar31: u32 = 0;
    let mut uVar32: u32 = 0;
    let mut uVar33: u32 = 0;
    let mut uVar34: u32 = 0;
    let mut uVar35: u32 = 0;
    let mut uVar36: u32 = 0;
    let mut uVar37: u32 = 0;
    let mut uVar38: u32 = 0;
    let mut uVar39: u32 = 0;
    let mut uVar40: u32 = 0;
    let mut uVar41: u32 = 0;
    let mut uVar42: u32 = 0;
    let mut uVar43: u32 = 0;
    let mut uVar44: u32 = 0;
    let mut uVar45: u32 = 0;
    let mut uVar46: u32 = 0;
    let mut uVar47: u32 = 0;
    let mut uVar48: u32 = 0;
    let mut uVar49: u32 = 0;
    let mut uVar50: u32 = 0;
    let mut uVar51: u32 = 0;
    let mut uVar52: u32 = 0;
    let mut uVar53: u32 = 0;
    let mut uVar54: u32 = 0;
    let mut uVar55: u32 = 0;
    let mut uVar56: u32 = 0;
    let mut uVar57: u32 = 0;
    let mut uVar58: u32 = 0;
    let mut uVar59: u32 = 0;
    let mut uVar60: u32 = 0;
    let mut uVar61: u32 = 0;
    let mut uVar62: u32 = 0;
    let mut uVar63: u32 = 0;
    let mut uVar64: u32 = 0;
    let mut uVar65: u32 = 0;
    let mut uVar66: u32 = 0;
    let mut uVar67: u32 = 0;
    let mut uVar68: u32 = 0;
    let mut uVar69: u32 = 0;
    let mut uVar70: u32 = 0;
    let mut uVar71: u32 = 0;
    let mut uVar72: u32 = 0;
    let mut uVar73: u32 = 0;
    let mut uVar74: u32 = 0;
    let mut uVar75: u32 = 0;
    let mut uVar76: u32 = 0;
    let mut uVar77: u32 = 0;
    let mut uVar78: u32 = 0;
    let mut uVar79: u32 = 0;
    let mut uVar80: u32 = 0;
    let mut uVar81: u32 = 0;
    let mut uVar82: u32 = 0;
    let mut uVar83: u32 = 0;
    let mut uVar84: u32 = 0;
    let mut uVar85: u32 = 0;
    let mut uVar86: u32 = 0;
    let mut uVar87: u32 = 0;
    let mut uVar88: u32 = 0;
    let mut uVar89: u32 = 0;
    let mut uVar90: u32 = 0;
    let mut uVar91: u32 = 0;
    let mut uVar92: u32 = 0;
    let mut uVar93: u32 = 0;
    let mut uVar94: u32 = 0;
    let mut uVar95: u32 = 0;
    let mut uVar96: u32 = 0;
    let mut uVar97: u32 = 0;
    let mut uVar98: u32 = 0;
    let mut uVar99: u32 = 0;
    let mut uVar100: u32 = 0;
    let mut uVar101: u32 = 0;
    let mut uVar102: u32 = 0;
    let mut uVar103: u32 = 0;
    let mut uVar104: u32 = 0;
    let mut uVar105: u32 = 0;
    let mut uVar106: u32 = 0;
    let mut uVar107: u32 = 0;
    let mut uVar108: u32 = 0;
    uVar39 = srcDwords[0x1a];
    uVar24 = srcDwords[0x14];
    uVar26 = srcDwords[0x13];
    uVar66 = srcDwords[8];
    uVar50 = (uVar39 & 0x25673c9b ^ 0x82100160) & uVar24 ^ !uVar24 & uVar26 & 0x7deffe9f;
    uVar25 = srcDwords[2];
    uVar60 = srcDwords[0x1d];
    uVar73 = ((uVar26 ^ 0xda98c364) & 0x7deffe9f ^ uVar39 & 0x82100160) & uVar24
        ^ !(uVar26 & 0x25673c9b) & uVar39 & 0xa7773dfb;
    uVar51 = ((uVar25 ^ uVar66) & uVar60 & 0x6f39089 ^ uVar25 & uVar66) & 0x5ffbfbaf;
    uVar61 = srcDwords[5] & 0x104a21ab;
    uVar29 = (uVar60 ^ uVar61) & uVar51 ^ (uVar60 & 0x4fb1da04 ^ 0x104a21ab) & srcDwords[5];
    uVar55 = srcDwords[0xe];
    uVar28 = srcDwords[9];
    uVar1 = !(uVar55 & 0xffb4b8f) & uVar60 & 0xafff4fdf;
    uVar30 = ((!(uVar28 & 0x6ca2d69e) ^ uVar25 & 0x6ca2d69e) & uVar39 ^ uVar25 & 0xeeb2d7fe)
        & 0xfdfffe9f;
    uVar31 = !(srcDwords[1] & 0xffbdfff6) & uVar55 & 0x86f79499
        ^ (uVar60 & 0x86b59490 ^ 0x70086866) & srcDwords[1];
    uVar24 = !(!(srcDwords[5] & 0xefb5de54) & uVar60 & 0x5ffbfbaf)
        ^ (!uVar60 & uVar51 ^ srcDwords[5]) & 0x5ffbfbaf;
    uVar87 =
        !((uVar60 ^ srcDwords[1] & 0x420009) & uVar55 & 0x86f79499) ^ srcDwords[1] & 0xf6bdfcf6;
    uVar18 = srcDwords[0x17];
    uVar88 = (uVar66 ^ srcDwords[0x19]) & uVar18 & 0x6f39089 ^ uVar66 & srcDwords[0x19];
    uVar89 = uVar88 & 0x86f79499;
    uVar90 = !((srcDwords[0x1c] & 0xf6bdfdf6 ^ uVar60) & srcDwords[0xb] & 0x7deffe9f)
        ^ uVar60 & 0x7deffe9f;
    uVar19 = srcDwords[0x11];
    uVar2 = ((uVar25 ^ uVar19 & 0x6f39089) & uVar51 ^ uVar25) & 0x86f79499;
    uVar91 = !(srcDwords[0x14] & uVar24 & 0xa7773dfb) ^ srcDwords[5] & 0xa7773dfb;
    uVar3 = (!(uVar18 & 0x9000) & uVar26 ^ uVar18) & 0x86f79499;
    uVar66 = !uVar66;
    uVar92 = !((uVar39 & 0x82100160 ^ 0x7deffe9f) & srcDwords[0x14]) ^ uVar26 & uVar39 & 0x25673c9b;
    uVar93 = !((!srcDwords[5] ^ uVar25 & 0xffb5fe74) & uVar39 & 0x104a21ab)
        ^ (srcDwords[5] & 0x4a018b ^ 0xafb54e54) & uVar25;
    uVar48 = (uVar19 & 0x82100140 ^ 0x2def4e9f) & srcDwords[5]
        ^ (srcDwords[5] & 0x2def4e9f ^ uVar19) & srcDwords[0xd] & 0xafff4fdf;
    uVar4 = (uVar60 & 0x6f39089 ^ uVar66) & uVar25 ^ srcDwords[8];
    uVar62 = !uVar61 & uVar60 ^ srcDwords[5] & uVar51 & 0x104a21ab;
    uVar63 = uVar62 & 0x5ffbfbaf;
    uVar64 = !uVar25;
    uVar49 = (uVar64 & 0x6ca2d69e ^ uVar28 & 0x80100000) & uVar39 ^ uVar25 & uVar28 & 0xecb2d69e;
    uVar47 = ((uVar28 & 0xdfdfff77 ^ uVar66) & srcDwords[0x16] & 0xafff6fdf
        ^ !(srcDwords[8] & 0xafff6fdf) & srcDwords[9])
        & 0xffffdfff;
    uVar67 = uVar66 & srcDwords[0x19] & 0x86f79499 ^ uVar18 & 0x5ffbfbaf;
    uVar68 =
        (!(srcDwords[0xb] & 0xffffdf77) ^ srcDwords[4] & 0x2088) & srcDwords[0x14] & 0x104a21ab
            ^ (srcDwords[0xb] ^ 0xefb5fedc) & srcDwords[4] & 0xdfdfdf77;
    uVar69 = (!(uVar19 & 0x8ff79799) & uVar24 ^ srcDwords[0x16] & uVar19 & 0x8ff79799) & 0xf6bdfcf6;
    uVar5 = !srcDwords[0xd];
    uVar70 =
        !((srcDwords[5] & 0xd210b160 ^ uVar5) & uVar19 & 0xafff4fdf) ^ srcDwords[5] & 0x7deffe9f;
    uVar6 = (uVar60 & srcDwords[0x1c] & 0xf6bdfdf6 ^ srcDwords[0xb]) & 0x7deffe9f;
    uVar23 = (uVar25 & 0xffb5fe74 ^ srcDwords[5]) & uVar39 & 0x104a21ab ^ uVar25 & 0xafff4fdf;
    uVar7 = !(uVar26 & 0x9000) & uVar18 & 0xafffdfdf;
    uVar106 = (!(uVar19 & 0x6f39089) & uVar51 ^ uVar25) & 0x86f79499 ^ uVar19 & 0x5ffbfbaf;
    uVar8 = ((srcDwords[1] & 0x420009 ^ !uVar60) & uVar55 ^ uVar60) & 0x86f79499
        ^ (uVar60 & 0x86b59490 ^ 0x420009) & srcDwords[1];
    uVar52 = !(!(uVar25 & 0x6f39089) & uVar19 & 0x5ffbfbaf) ^ uVar25 & uVar51 & 0x86f79499;
    uVar53 = !(srcDwords[0x16] & uVar24 & 0xf6bdfcf6) ^ uVar19 & 0x86f79499;
    uVar54 = !(!srcDwords[7] & srcDwords[0x18] & 0xf6bdfcf6) ^ uVar5 & srcDwords[7] & 0xf6bdfcf6;
    uVar9 = !uVar87 & uVar55 & 0xafff4fdf ^ srcDwords[0x19] & 0xdfdfdf77;
    uVar10 = ((uVar62 & 0x4250a323 ^ 0x9daf5c9c) & uVar24 ^ 0x62400022) & uVar29
        ^ (uVar62 & 0x1dab588c ^ 0x10a341) & uVar24
        ^ uVar63
        ^ 0x9daf5c9c;
    uVar62 = !(uVar60 & !(uVar25 & 0x6f39089) & 0x86f79499) ^ srcDwords[8] & 0x5ffbfbaf;
    uVar32 = (srcDwords[9] & 0x20200088 ^ !srcDwords[0x16]) & srcDwords[8] & 0xafff4fdf
        ^ (srcDwords[0x16] & 0x8fdf4f57 ^ 0x50009020) & srcDwords[9];
    uVar33 = !(uVar25 & 0x5fdbdb27) & srcDwords[0x14] & 0xdfdfdf77;
    uVar27 =
        (srcDwords[0x14] & 0xdfdfdf77 ^ uVar64 & 0x202088) & uVar26 ^ uVar25 & 0x5ffbfbaf ^ uVar33;
    uVar11 = !((srcDwords[0x19] & 0xdfdfff77 ^ uVar55) & uVar87 & 0xafff4fdf)
        ^ srcDwords[0x19] & 0xdfdfdf77;
    uVar12 = !(!(uVar19 & 0x2def4e9f) & srcDwords[5] & 0xffffffdf)
        ^ !(srcDwords[5] & 0x2def4e9f) & srcDwords[0xd] & 0xafff4fdf;
    uVar34 = !(uVar60 & 0x74adfc96) & srcDwords[0x1c] & 0xf6bdfcf6
        ^ uVar60 & srcDwords[0xb] & 0x7deffe9f;
    uVar94 = ((uVar19 & 0x8ff79799 ^ srcDwords[0x16]) & uVar24 ^ srcDwords[0x16]) & 0xf6bdfcf6;
    uVar19 = srcDwords[9];
    uVar13 = (srcDwords[8] & 0x8fdf4f57 ^ 0x20200088) & uVar19
        ^ !(uVar28 & 0xdfdfff77) & srcDwords[0x16] & 0xafff4fdf;
    uVar14 = uVar92 ^ 0xc6d87455;
    uVar95 = (!uVar94 ^ uVar53) & uVar69 ^ uVar53;
    uVar95 = (uVar14 & uVar73 ^ uVar95 & 0x39278baa ^ uVar92 ^ 0xf6dcf55f) & uVar50
        ^ (uVar95 ^ uVar92 ^ 0xf614c50f) & uVar73
        ^ uVar92
        ^ 0xc6d87455;
    uVar35 = ((srcDwords[0x14] & 0x104a0123 ^ !srcDwords[4]) & srcDwords[0xb] & 0xffffdf77
        ^ !(srcDwords[0x14] & 0x104a21ab) & srcDwords[4])
        & 0xdfdfffff;
    uVar74 = (uVar26 & 0x86f79499 ^ 0xafff4fdf) & uVar18;
    uVar36 = (!(uVar50 & 0xaf773ffb) ^ uVar19 & 0x5088c004) & srcDwords[0x10] & 0xf6bdfcf6
        ^ uVar19 & 0xa7773dfb;
    uVar64 = (uVar28 & 0x80100000 ^ 0x7deffe9f) & uVar39 ^ !(uVar64 & uVar19 & 0xecb2d69e);
    uVar28 = (!(uVar4 & 0x1debf98e) & 0xef73efe7 ^ uVar62 & 0x30ccb15a) & uVar51
        ^ (uVar4 & 0x4f334aa5 ^ 0x9daf5c9c) & uVar62
        ^ (uVar4 & 0x5ffbfbaf ^ 0x9daf5c9c) & 0xef73efe7;
    uVar96 = ((uVar106 & 0x30ccb15a ^ 0x9daf5c9c) & uVar2 ^ uVar106 & 0x6250a363 ^ 0x9daf5c9c)
        & uVar52
        ^ (!(uVar106 & 0xdfbf5ebd) & uVar2 ^ uVar106 ^ 0x108c1018) & 0x72dcb37b;
    uVar37 = !((srcDwords[0x10] & 0xfebdfef6 ^ !uVar19) & uVar50 & 0xa7773dfb)
        ^ !(srcDwords[0x10] & 0x5088c004) & uVar19 & 0xf7fffdff;
    uVar38 = uVar74 ^ uVar7;
    uVar71 = ((uVar53 ^ uVar7) & uVar74 ^ uVar53 ^ uVar7) & uVar94
        ^ !((uVar74 ^ uVar94) & uVar53) & uVar69
        ^ (uVar38 & uVar94 ^ !uVar7 & uVar74) & uVar3
        ^ uVar74;
    uVar15 = (!srcDwords[0x18] & srcDwords[0xd] ^ srcDwords[7]) & 0xf6bdfcf6;
    uVar16 = !uVar23;
    uVar61 = ((uVar39 ^ srcDwords[5]) & 0x4a018b ^ 0x10002020) & uVar25 ^ uVar61;
    uVar39 = (uVar23 ^ uVar93 ^ uVar31) & uVar61;
    uVar75 = (!uVar39 ^ uVar93 ^ uVar31) & uVar87
        ^ (uVar61 ^ uVar87) & uVar8 & uVar31
        ^ uVar16 & uVar61
        ^ uVar23
        ^ uVar93;
    uVar76 = uVar55 & 0x5ffbfbaf;
    uVar97 = !((srcDwords[7] ^ srcDwords[0xd]) & srcDwords[0x18] & 0xf6bdfcf6)
        ^ srcDwords[7] & 0xf6bdfcf6;
    uVar77 = (uVar54 ^ uVar47) & uVar97;
    uVar17 = !uVar97;
    uVar98 = (!((uVar97 ^ uVar47) & uVar13) ^ uVar17 & uVar47) & uVar32
        ^ (!((uVar17 ^ uVar13) & uVar54) ^ uVar97 ^ uVar13) & uVar15
        ^ (uVar77 ^ uVar54) & uVar13
        ^ uVar17 & uVar54
        ^ uVar97;
    uVar78 = !(uVar19 & 0xaf773ffb) & srcDwords[0x10] & 0xf6bdfcf6
        ^ (uVar50 ^ 0xfebdfef6) & uVar19 & 0xa7773dfb;
    uVar18 = ((uVar18 & 0x6f39089 ^ uVar66) & srcDwords[0x19] ^ srcDwords[8]) & 0x86f79499;
    uVar55 = !(uVar55 & 0xafff6fdf) & srcDwords[0x19] & 0xdfdfdf77
        ^ !(srcDwords[0x19] & 0xdfdfff77) & uVar87 & 0xafff4fdf;
    uVar56 = !(((uVar16 ^ uVar93) & uVar31 ^ uVar39 ^ uVar23) & uVar87)
        ^ (uVar61 ^ uVar23 ^ uVar93 ^ uVar87) & uVar8 & uVar31
        ^ (uVar61 ^ uVar93) & uVar23
        ^ uVar93;
    uVar66 = uVar8 ^ uVar31 ^ 0xcf334ea5;
    uVar39 = !uVar31 & uVar8;
    uVar19 = ((uVar66 & uVar74 ^ uVar8 ^ uVar31 ^ 0xcf334ea5) & uVar7
        ^ (uVar39 ^ uVar31) & 0xcf334ea5)
        & uVar87
        ^ (!(!uVar74 & uVar31) & 0xcf334ea5 ^ (uVar8 ^ 0xcf334ea5) & uVar74 ^ uVar8) & uVar7
        ^ (uVar66 & uVar87 ^ !uVar31 & 0xcf334ea5 ^ uVar8) & uVar74 & uVar3
        ^ uVar39 & 0xcf334ea5
        ^ 0x30ccb15a;
    uVar20 = uVar78 ^ uVar37;
    uVar57 = (uVar24 & 0x7fca4832 ^ 0x10a341) & uVar29
        ^ !uVar24 & uVar63 & 0xe26514ae
        ^ !(uVar24 & 0x10a341) & 0x9dbfffdd;
    uVar79 = !uVar78;
    uVar21 = uVar60 & 0xafff4fdf;
    uVar99 = (!((uVar20 ^ uVar48) & uVar36) ^ uVar20 & uVar48 ^ uVar37) & uVar70
        ^ (!((uVar79 ^ uVar37 ^ uVar36) & uVar70) ^ uVar78 ^ uVar37 ^ uVar36) & uVar12
        ^ (uVar37 ^ uVar36) & uVar78
        ^ uVar36;
    uVar40 = !uVar21;
    uVar39 = (uVar40 ^ uVar1) & uVar34;
    uVar66 = (uVar21 ^ uVar1) & uVar76;
    uVar22 = !((!((uVar40 ^ uVar34 ^ uVar1) & uVar6) ^ uVar66 ^ uVar21 ^ uVar34 ^ uVar1) & uVar90)
        ^ (uVar39 ^ uVar66 ^ uVar21 ^ uVar1) & uVar6
        ^ uVar39
        ^ uVar1;
    uVar100 = (!((uVar74 ^ uVar69) & uVar53) ^ uVar74 ^ uVar69) & uVar94
        ^ ((!uVar53 ^ uVar7) & uVar74 ^ uVar53 ^ uVar7) & uVar69
        ^ !((uVar38 & uVar69 ^ !uVar7 & uVar74) & uVar3);
    uVar41 = (srcDwords[0xb] & 0x104a0123 ^ srcDwords[4] & 0x2088) & srcDwords[0x14]
        ^ srcDwords[4] & 0xdfdfdf77;
    uVar42 = uVar11 ^ uVar9;
    uVar39 = !uVar92 & uVar50;
    uVar65 = ((uVar42 ^ uVar92) & uVar73 ^ uVar39 ^ uVar9 ^ uVar92) & uVar55
        ^ (!uVar39 ^ uVar11) & uVar73
        ^ uVar92;
    uVar39 = (!uVar61 ^ uVar93) & uVar23;
    uVar58 = !uVar70 ^ uVar48;
    uVar43 = ((uVar36 ^ 0x104a21ab) & uVar23 ^ uVar16 & uVar61 & 0xe26514ae ^ uVar36 ^ 0x104a21ab)
        & uVar93
        ^ ((uVar39 ^ uVar93 ^ 0xe26514ae) & (uVar37 ^ uVar36) ^ uVar39 ^ uVar93 ^ 0xe26514ae)
            & uVar78
        ^ ((uVar36 ^ 0xf22f3505) & uVar61 ^ uVar36 ^ 0xf22f3505) & uVar23
        ^ (uVar36 ^ 0x4000aa) & 0xe26514ae;
    uVar72 = uVar70 ^ uVar48;
    uVar59 = uVar58 & uVar12 ^ uVar70;
    uVar44 = ((!(uVar58 & uVar1) ^ uVar58 & uVar21 ^ uVar70 ^ uVar48) & uVar12
        ^ (uVar60 & 0x86d84455 ^ uVar70) & uVar1
        ^ (uVar70 ^ 0x39278baa) & uVar21
        ^ uVar70
        ^ 0x19270888)
        & uVar76
        ^ ((uVar21 ^ 0x6250a363) & uVar72 ^ uVar21 ^ 0x6250a363) & uVar12
        ^ ((uVar59 ^ 0x39278baa) & uVar21 ^ 0x9daf5c9c) & uVar1
        ^ (uVar21 ^ 0x6250a363) & uVar70
        ^ uVar60 & 0x29270b8a
        ^ 0xdfff7cdd;
    uVar39 = (!((!uVar76 ^ uVar34 ^ uVar90) & uVar6) ^ uVar76 ^ uVar34 ^ uVar90) & uVar21
        ^ !((uVar21 ^ uVar6) & uVar76) & uVar1
        ^ uVar90;
    uVar101 = (!(!uVar6 & uVar34) ^ uVar40 & uVar76 ^ uVar21) & uVar1
        ^ ((!uVar34 ^ uVar1) & uVar6 ^ uVar66 ^ uVar21 ^ uVar34 ^ uVar1) & uVar90
        ^ uVar21
        ^ uVar6;
    uVar45 = (srcDwords[0x14] & srcDwords[5] ^ !srcDwords[5] & uVar24) & 0xa7773dfb;
    uVar102 = uVar94 ^ uVar53;
    uVar46 = (uVar34 ^ uVar6) & uVar90 ^ (uVar32 ^ uVar47) & uVar13 ^ uVar34 ^ uVar47;
    uVar66 = uVar92 ^ uVar73 ^ 0xc6d87455;
    uVar60 = (uVar73 ^ 0x30ccb15a) & uVar92;
    uVar80 = uVar92 ^ uVar73 ^ 0x39278baa;
    uVar80 = ((uVar66 & uVar102 ^ uVar92 ^ uVar73 ^ 0xc6d87455) & uVar69
        ^ (uVar73 ^ 0xc83050) & 0x9eb3af0
        ^ uVar66 & uVar53
        ^ uVar60)
        & uVar50
        ^ (uVar80 & uVar102 ^ uVar92 ^ uVar73 ^ 0x39278baa) & uVar69
        ^ uVar80 & uVar53
        ^ uVar73 & 0x9eb3af0
        ^ uVar60
        ^ 0x3004810a;
    uVar6 = !(((uVar34 ^ uVar32 ^ uVar6 ^ uVar47) & uVar13 ^ uVar6 ^ uVar47) & uVar90)
        ^ (uVar34 ^ uVar32) & uVar13;
    uVar81 = ((uVar24 & 0x6250a363 ^ 0x9daf5c9c) & uVar63 ^ uVar24 & 0x7fca4832 ^ 0x9dbfffdd)
        & uVar29
        ^ (uVar24 & 0x1d9aeb51 ^ 0xe26514ae) & uVar63
        ^ !(uVar24 & 0xe26514ae) & 0x7fdaeb73;
    uVar16 = uVar16 & uVar93;
    uVar29 = ((!uVar37 & uVar78 ^ uVar79 & uVar36 ^ uVar16 ^ uVar23 ^ 0xffbfff55) & uVar61
        ^ (uVar36 ^ 0x4000aa) & uVar23)
        & 0xe26514ae
        ^ (!((!uVar37 ^ uVar36) & uVar23 & 0xe26514ae) ^ uVar37 ^ uVar36) & uVar78
        ^ uVar36
        ^ 0x104a21ab;
    uVar60 = !(((uVar94 ^ uVar69) & uVar38 ^ uVar74 ^ uVar7) & uVar3)
        ^ (!((!uVar94 ^ uVar69) & uVar74) ^ uVar94 ^ uVar69) & uVar7
        ^ uVar74
        ^ uVar94;
    uVar66 = uVar21 ^ uVar1 ^ 0xc6d87455;
    uVar34 = !uVar92 ^ uVar73;
    uVar66 = ((uVar66 & uVar72 ^ uVar21 ^ uVar1 ^ 0xc6d87455) & uVar12
        ^ !(!uVar1 & uVar21 & 0xdfff7cdd) & 0xe6d8f777
        ^ uVar66 & uVar70
        ^ uVar1)
        & uVar76
        ^ ((!(uVar58 & uVar21) ^ uVar70 ^ uVar48) & uVar1
            ^ (uVar21 ^ 0x9daf5c9c) & uVar72
            ^ uVar21
            ^ 0x9daf5c9c)
            & uVar12
        ^ ((uVar70 ^ 0xc6d87455) & uVar21 ^ uVar70 ^ 0x5b7728c9) & uVar1
        ^ (uVar21 ^ 0x9daf5c9c) & uVar70
        ^ (uVar21 ^ 0xbdafdfbe) & 0xc6d87455;
    uVar38 = (uVar17 ^ uVar15) & uVar54 ^ uVar97 ^ uVar15;
    uVar103 = ((!(uVar34 & uVar94) ^ uVar34 & uVar53 ^ uVar92 ^ uVar73) & uVar69
        ^ (uVar53 ^ uVar73 ^ 0xcf334ea5) & uVar92
        ^ (uVar53 ^ 0xf614c50f) & uVar73
        ^ uVar53
        ^ 0xcf334ea5)
        & uVar50
        ^ (uVar102 & uVar14 ^ uVar92 ^ 0xc6d87455) & uVar69
        ^ (uVar73 ^ 0xcf334ea5) & uVar92
        ^ uVar14 & uVar53
        ^ uVar73 & 0x39278baa
        ^ 0xc6104405;
    uVar53 = (uVar32 ^ uVar47 ^ uVar38) & uVar13 ^ (uVar47 ^ uVar38) & uVar32 ^ uVar97;
    uVar14 = (!uVar35 ^ uVar68) & uVar41 ^ uVar68 ^ 0x1d9aeb51;
    uVar90 = uVar13 ^ uVar90;
    uVar34 = (uVar49 ^ 0xc6d87455) & uVar68;
    uVar58 = (!(uVar40 & uVar1 & 0xdfff7cdd) & 0xe6d8f777 ^ uVar59 & 0x39278baa ^ uVar21) & uVar76
        ^ (uVar58 & uVar12 ^ uVar21 ^ uVar70 ^ 0xa488d736) & uVar1
        ^ uVar21
        ^ 0x9daf5c9c;
    uVar76 = (!(uVar64 & uVar14) ^ uVar14 & uVar49) & uVar30
        ^ (uVar35 & (uVar49 ^ 0x39278baa) ^ uVar34 ^ uVar49 ^ 0xdb429f04) & uVar41
        ^ uVar49 & 0x1d9aeb51
        ^ uVar34
        ^ 0xfb679fae;
    uVar82 = (uVar4 & 0x52981229 ^ 0x6250a363) & uVar62
        ^ (uVar4 & 0x1debf98e ^ !(uVar62 & 0xbdeffdde)) & uVar51 & 0xef73efe7
        ^ !(uVar4 & 0x10881008) & 0xbdeffdde;
    uVar104 = (!(uVar67 & 0x30ccb15a) & 0xbdeffdde ^ uVar88 & 0x86330481) & uVar18
        ^ !((uVar67 ^ 0xdfbf5ebd) & uVar89 & 0xef73efe7) & 0x72dcb37b
        ^ uVar67 & 0xcf334ea5;
    uVar1 = (uVar101 & uVar39 ^ uVar22) & 0xa258db45;
    uVar105 = (!(uVar62 & 0xdfbf5ebd) ^ uVar4 & 0x5fbb5aad) & uVar51 & 0xbdeffdde
        ^ (!(uVar62 & 0xbdeffdde) & uVar4 & 0x5ffbfbaf ^ 0xbdeffdde) & 0xdfbf5ebd;
    uVar62 = (uVar7 ^ uVar3) & uVar74;
    uVar4 = (uVar8 ^ 0x30ccb15a) & uVar74;
    uVar38 = ((uVar31 & 0xcf334ea5 ^ uVar62 ^ uVar7 ^ 0x30ccb15a) & uVar8
        ^ (uVar62 ^ uVar7 ^ 0x30ccb15a) & uVar31
        ^ 0xcf334ea5)
        & uVar87
        ^ (uVar31 & 0xcf334ea5 ^ 0x30ccb15a) & !uVar8
        ^ (uVar8 ^ uVar4 ^ 0x30ccb15a) & uVar7
        ^ uVar4 & uVar3;
    uVar4 = (!uVar90 & uVar46 & 0x39278baa ^ 0xc6d87455) & uVar6 ^ uVar90;
    uVar7 = !uVar74 & uVar7;
    uVar94 = ((uVar46 & 0x39278baa ^ 0xc6d87455) & uVar6 ^ uVar46) & uVar90
        ^ !(uVar46 & 0x39278baa) & uVar6
        ^ uVar46
        ^ 0xc6d87455;
    uVar102 = ((!(!uVar87 & uVar8) ^ uVar7) & 0xcf334ea5 ^ uVar87) & uVar31
        ^ (!uVar7 & 0xcf334ea5 ^ uVar8) & uVar87
        ^ (uVar87 ^ uVar31) & uVar74 & uVar3 & 0xcf334ea5
        ^ uVar8
        ^ 0x30ccb15a;
    uVar14 = (!uVar47 & uVar32 ^ !((uVar32 ^ uVar47) & uVar97)) & uVar13
        ^ ((uVar97 ^ uVar32) & uVar54 ^ uVar97 ^ uVar32) & uVar15
        ^ (!uVar77 ^ uVar54 ^ uVar47) & uVar32;
    uVar47 = uVar98 & 0x5da724ba;
    uVar3 = (!(uVar53 & 0x5da724ba) ^ uVar47) & uVar14 ^ uVar47;
    uVar87 = ((uVar8 ^ uVar87) & uVar23 ^ uVar8 ^ uVar87) & uVar31
        ^ ((uVar8 ^ uVar87) & uVar31 ^ uVar23) & uVar93
        ^ uVar61
        ^ uVar87;
    uVar59 = ((uVar64 ^ uVar35 ^ uVar68 ^ uVar49) & uVar30 ^ uVar35 ^ uVar49 ^ 0x24bd60fb) & uVar41
        ^ (uVar68 ^ uVar49 ^ 0xdb429f04) & uVar30
        ^ uVar49
        ^ 0x39278baa;
    uVar8 = (!uVar98 & uVar14 & 0xa258db45 ^ 0x5da724ba) & uVar53 ^ (uVar47 ^ 0xa258db45) & uVar14;
    uVar62 = (!uVar106 & !uVar2 & uVar52 & 0xef73efe7 ^ uVar106) & 0x72dcb37b
        ^ (uVar106 & 0xcf334ea5 ^ 0xad63edc6) & uVar2
        ^ 0xbdeffdde;
    uVar2 = ((uVar52 & 0x30ccb15a ^ 0x9daf5c9c) & uVar106 ^ 0xdfbf5ebd) & uVar2
        ^ (!(uVar52 & 0xdfbf5ebd) & uVar106 ^ 0x6250a363) & 0xef73efe7;
    uVar13 = !(uVar67 & 0xdfbf5ebd) & uVar18 & 0xbdeffdde
        ^ (uVar67 & 0x30ccb15a ^ 0x42100221) & uVar89
        ^ 0xcf334ea5;
    uVar63 = !(((uVar42 ^ uVar73 ^ uVar50) & uVar92 ^ uVar11 ^ uVar73 ^ uVar50) & uVar55)
        ^ (!uVar11 ^ uVar73 ^ uVar50) & uVar92
        ^ uVar11
        ^ uVar50;
    uVar106 = (uVar93 ^ 0x1d9aeb51) & uVar23;
    uVar47 = uVar106 ^ uVar93 ^ 0xe26514ae;
    uVar31 = ((uVar78 & (uVar23 ^ 0xe26514ae) ^ uVar23 ^ 0xe26514ae) & uVar61
        ^ uVar78 & uVar47
        ^ uVar106
        ^ uVar93
        ^ 0xe26514ae)
        & uVar36
        ^ ((uVar37 & (uVar23 ^ 0xe26514ae) ^ uVar23 ^ 0xe26514ae) & uVar78
            ^ (uVar16 ^ 0x4000aa) & 0xe26514ae
            ^ uVar23 & 0xdd0cafa)
            & uVar61
        ^ (uVar37 & uVar47 ^ uVar106 ^ uVar93 ^ 0xe26514ae) & uVar78
        ^ !(uVar23 & 0xd90ca50) & 0x1ddaebfb
        ^ uVar16 & 0xefb5de54;
    uVar32 = (!uVar39 & uVar101 ^ !uVar22 & uVar39) & 0xa258db45;
    uVar98 = ((uVar98 & 0xa258db45 ^ 0x5da724ba) & uVar14 ^ uVar98 & 0xa258db45 ^ 0x5da724ba)
        & uVar53
        ^ !(!uVar98 & uVar14) & 0xa258db45
        ^ uVar98;
    uVar40 = !((!(((uVar58 ^ srcDwords[0xd]) & uVar44 ^ uVar58 ^ srcDwords[0xd]) & uVar66)
        ^ !uVar44 & srcDwords[0xd])
        & srcDwords[4])
        ^ uVar66;
    uVar47 = !uVar55 ^ uVar97;
    uVar90 = (!uVar46 & uVar90 ^ uVar46 ^ 0x39278baa) & uVar6 ^ uVar90;
    uVar77 = (!(uVar47 & uVar9) ^ uVar55 ^ uVar97) & uVar11
        ^ (uVar15 ^ uVar9) & uVar55 & uVar97
        ^ uVar47 & uVar15 & uVar54
        ^ uVar97
        ^ uVar15;
    uVar106 = !uVar80 ^ uVar95;
    uVar93 = !uVar2;
    uVar83 = !(uVar2 & uVar106) ^ uVar106 & uVar96 ^ uVar80 ^ uVar95;
    uVar107 = !(uVar103 & uVar106) ^ uVar95;
    uVar23 = (uVar93 ^ uVar96) & uVar95;
    uVar46 = !uVar95;
    uVar84 = (!(((!(uVar103 & uVar83) ^ uVar2 ^ uVar23 ^ uVar96) & uVar62 ^ uVar107 & uVar96)
        & srcDwords[0x10])
        ^ uVar103 & uVar46
        ^ uVar95)
        & uVar26
        ^ uVar103
        ^ srcDwords[0x10];
    uVar108 =
        !((srcDwords[0x14] ^ srcDwords[5]) & uVar24 & 0xa7773dfb) ^ srcDwords[0x14] & 0xa7773dfb;
    uVar51 = (!((uVar45 ^ uVar91) & uVar35) ^ uVar45 ^ uVar91) & uVar68
        ^ (uVar35 ^ uVar68) & uVar41 & (uVar45 ^ uVar91)
        ^ uVar108 & uVar45 & !uVar91
        ^ uVar91;
    uVar69 = (!uVar98 ^ uVar8) & uVar3;
    uVar85 = (!(uVar98 & !srcDwords[6]) ^ srcDwords[6]) & uVar8 ^ uVar98 ^ uVar69;
    uVar86 =
        (!(!uVar4 & srcDwords[10]) ^ uVar90) & uVar94 ^ (uVar4 ^ !uVar90) & srcDwords[10] ^ uVar90;
    uVar6 = !((!(uVar100 & 0xa258db45) & uVar71 ^ 0xa258db45) & uVar60) ^ uVar71;
    uVar52 = !uVar57 & uVar10;
    uVar53 = ((uVar88 & 0x84638480 ^ 0x30ccb15a) & uVar67 ^ uVar89 ^ 0x42100221) & uVar18
        ^ (uVar88 & 0x2508001 ^ 0x30ccb15a) & uVar67
        ^ uVar89
        ^ 0x2040a142;
    uVar14 = !((uVar57 ^ uVar10) & uVar81) ^ uVar57 ^ uVar52;
    uVar16 = uVar53 & uVar14;
    uVar21 = uVar13 ^ uVar104;
    uVar74 = uVar13 & !uVar104;
    uVar18 = ((uVar57 & uVar21 ^ uVar104 ^ uVar74) & uVar53 ^ uVar57 & uVar13 & uVar104)
        & srcDwords[0x16]
        ^ ((!(uVar81 & !uVar57) ^ uVar57) & uVar10 ^ !uVar16 ^ uVar57) & srcDwords[0x1c]
        ^ uVar53
        ^ uVar57;
    uVar34 = uVar87 & uVar75 & 0xa258db45 ^ uVar56;
    uVar24 = (uVar17 ^ uVar15) & uVar9;
    uVar61 = (!uVar24 ^ uVar97 ^ uVar15) & uVar11
        ^ (uVar24 ^ uVar97 ^ uVar15) & uVar55
        ^ !(uVar97 & uVar54) & uVar15;
    uVar67 = (!(uVar60 & 0xa258db45) ^ uVar100) & uVar71
        ^ (uVar100 ^ 0x5da724ba) & uVar60
        ^ uVar100
        ^ 0xa258db45;
    uVar97 = (!((!uVar55 ^ uVar15) & uVar9) ^ uVar55 ^ uVar15) & uVar11
        ^ !((uVar97 ^ uVar9 ^ uVar54) & uVar15) & uVar55
        ^ uVar97;
    uVar7 = ((uVar12 ^ uVar48) & (uVar79 ^ uVar37) ^ uVar78 ^ uVar37) & uVar70
        ^ (uVar79 ^ uVar36 ^ uVar12) & uVar37
        ^ (!uVar36 ^ uVar12) & uVar78
        ^ uVar12;
    uVar54 =
        (!(uVar101 & 0x5da724ba) ^ uVar39 & 0x5da724ba) & uVar22 ^ !uVar101 & uVar39 ^ 0xa258db45;
    uVar47 = ((uVar87 & 0x5da724ba ^ 0xa258db45) & uVar75 ^ 0x5da724ba) & uVar56
        ^ (uVar75 ^ 0xa258db45) & uVar87
        ^ 0xa258db45;
    uVar39 = (!uVar69 ^ uVar98 ^ uVar8) & srcDwords[6] ^ uVar98 ^ uVar3;
    uVar79 = (!((!(uVar26 & uVar106) ^ uVar62 & (uVar93 ^ uVar96) ^ uVar96) & srcDwords[0x10])
        ^ uVar26 & uVar80)
        & uVar103
        ^ !(uVar26 & uVar46) & srcDwords[0x10];
    uVar106 = ((!(srcDwords[0x14] & 0xffdfdf77) ^ uVar25 & 0x5fdbdb27) & uVar26
        ^ srcDwords[0x14] & 0xffdfdf77)
        & 0xdfffffff;
    uVar33 = !(uVar25 & 0x202088) & uVar26 & 0x5ffbfbaf ^ uVar33;
    uVar25 = !uVar1;
    uVar69 = !(uVar54 & uVar25) & srcDwords[0x1b] ^ uVar32;
    uVar72 = (((uVar81 ^ uVar10) & uVar21 ^ uVar13 ^ uVar104) & srcDwords[0x1c] ^ uVar13 ^ uVar104)
        & uVar57
        ^ ((uVar81 & uVar21 ^ uVar13 ^ uVar104) & uVar10 ^ uVar13 ^ uVar104) & srcDwords[0x1c];
    uVar14 = uVar14 & srcDwords[0x1c];
    uVar88 = ((uVar57 ^ uVar14) & uVar13 & uVar104 ^ (uVar72 ^ uVar104 ^ uVar74) & uVar53)
        & srcDwords[0x16]
        ^ (!uVar10 & uVar81 & uVar57 ^ uVar16) & srcDwords[0x1c];
    uVar9 = !uVar27 ^ uVar33;
    uVar15 = uVar9 & uVar106;
    uVar74 = (uVar108 & uVar45 ^ uVar15 ^ uVar27 ^ uVar33) & uVar91
        ^ (!uVar15 ^ uVar45 ^ uVar27 ^ uVar33) & uVar108
        ^ uVar27;
    uVar89 = (!uVar27 ^ uVar106) & uVar33;
    uVar21 = !uVar34;
    uVar101 = (((uVar75 & 0x5da724ba ^ !(uVar87 & 0x5da724ba)) & uVar56
        ^ (uVar87 ^ 0xa258db45) & uVar75
        ^ 0xa258db45)
        & (uVar21 ^ uVar47)
        ^ uVar47)
        & srcDwords[0];
    uVar87 = uVar89 ^ uVar27;
    uVar75 = ((uVar27 ^ uVar106) & (uVar64 ^ 0x1d9aeb51) ^ uVar64 ^ 0x1d9aeb51) & uVar33;
    uVar22 = ((uVar27 ^ 0x9daf5c9c) & uVar64 ^ uVar27 & 0x1d9aeb51 ^ uVar75 ^ 0xffef5cbe) & uVar30
        ^ !((uVar87 ^ 0x9daf5c9c) & uVar49) & uVar64
        ^ (uVar87 ^ 0x9dbfffdd) & 0xe26514ae;
    uVar16 = !(uVar94 & !uVar90) & srcDwords[10] ^ uVar90;
    uVar60 = uVar60 ^ !(uVar60 & uVar100 & 0xa258db45) & uVar71;
    uVar17 = uVar77 & 0xa258db45;
    uVar24 = ((uVar17 ^ 0x5da724ba) & uVar61 ^ uVar77) & uVar97 ^ !(uVar61 & uVar77) & 0x5da724ba;
    uVar55 = (!uVar50 & uVar73 ^ uVar42 & uVar55 ^ uVar11) & uVar92
        ^ (uVar42 & uVar55 ^ uVar11 ^ uVar50) & uVar73
        ^ uVar55;
    uVar11 = srcDwords[0] & uVar21 & uVar47;
    uVar50 = (!(uVar49 & 0xe26514ae) & uVar30 ^ uVar89 ^ uVar27 ^ 0x7fca4832) & uVar64
        ^ (uVar87 & 0xe26514ae ^ 0x7fdaeb73) & uVar30
        ^ 0xe26514ae;
    uVar21 = (uVar61 ^ uVar17) & uVar97 ^ !(uVar61 & uVar77) & 0xa258db45;
    uVar71 = !(uVar6 & srcDwords[0x15] & !uVar67 & uVar60) ^ uVar6 ^ srcDwords[0x15];
    uVar98 = (uVar8 & !srcDwords[6] ^ uVar98) & uVar3 ^ (uVar98 ^ srcDwords[6]) & uVar8 ^ uVar98;
    uVar3 = (uVar64 ^ uVar49) & uVar30 ^ uVar49;
    uVar42 = !(((!(uVar94 & !uVar4) ^ uVar4) & srcDwords[10] ^ uVar94) & uVar90) ^ srcDwords[10];
    uVar56 = ((uVar3 ^ 0xc6d87455) & uVar35 ^ (uVar3 ^ 0x39278baa) & uVar68 ^ uVar30 ^ 0xe26514ae)
        & uVar41
        ^ ((uVar68 ^ 0x1d9aeb51) & uVar49 ^ (uVar68 ^ 0xe26514ae) & uVar64 ^ 0xc6d87455) & uVar30
        ^ (uVar49 ^ 0xfb679fae) & 0x1d9aeb51
        ^ (uVar49 ^ 0x39278baa) & uVar68;
    uVar12 = ((uVar20 ^ uVar12 ^ uVar48) & uVar70 ^ uVar37 ^ uVar12) & uVar36
        ^ (uVar37 ^ uVar12) & uVar70
        ^ uVar78
        ^ uVar12;
    uVar3 = !uVar45;
    uVar48 = (!((uVar59 ^ uVar56) & uVar76) ^ uVar59 & uVar56) & srcDwords[0x19];
    uVar73 = ((uVar108 ^ uVar35 ^ uVar91) & uVar45 ^ (uVar3 ^ uVar35) & uVar41 ^ uVar35 ^ uVar91)
        & uVar68
        ^ (!(uVar41 & uVar35) ^ uVar108) & uVar45
        ^ uVar91;
    uVar37 = uVar59 ^ uVar48;
    uVar20 = !uVar48 ^ uVar59;
    uVar48 = uVar29 & uVar20;
    uVar36 = !uVar59 ^ uVar29;
    uVar4 = uVar59 & uVar29;
    uVar8 = (!((uVar36 & uVar56 ^ uVar4) & uVar76) ^ !(uVar29 & uVar56) & uVar59 ^ uVar29)
        & srcDwords[0x19]
        ^ (((uVar29 ^ uVar37) & uVar43 ^ uVar48) & uVar31 ^ uVar43 & uVar48) & srcDwords[0xb]
        ^ uVar59
        ^ uVar29;
    uVar48 = srcDwords[0x1b];
    uVar70 = !(((!(uVar32 & uVar25) ^ uVar1) & uVar54 ^ uVar32 ^ uVar1) & uVar48) ^ !uVar32 & uVar1;
    uVar25 = (!uVar48 & uVar1 ^ uVar48) & uVar32 ^ uVar48 & uVar25;
    uVar54 = !uVar102;
    uVar14 = !(((!uVar14 ^ uVar57) & uVar13 & uVar104 ^ (uVar72 ^ uVar13 & uVar104) & uVar53)
        & srcDwords[0x16])
        ^ uVar53
        ^ uVar57;
    uVar48 =
        ((uVar18 ^ uVar102 ^ uVar88) & uVar14 ^ (uVar102 ^ uVar14) & uVar38 ^ uVar18 ^ uVar102)
            & uVar19
            ^ (!(uVar54 & uVar38) ^ uVar88) & uVar14
            ^ uVar88;
    uVar94 = ((uVar49 & 0xe26514ae ^ uVar27 ^ 0x9daf5c9c) & uVar64
        ^ uVar27 & 0x1d9aeb51
        ^ uVar75
        ^ 0xffef5cbe)
        & uVar30
        ^ ((uVar87 ^ 0x6250a363) & uVar49 ^ 0xe26514ae) & uVar64
        ^ (uVar87 ^ 0xffef5cbe) & 0x1d9aeb51;
    uVar34 = uVar34 ^ uVar47;
    uVar90 =
        (!((!uVar18 ^ uVar102 ^ uVar88) & uVar14) ^ (uVar54 ^ uVar14) & uVar38 ^ uVar18 ^ uVar88)
            & uVar19
            ^ (!(uVar54 & uVar14) ^ uVar102) & uVar38
            ^ (uVar18 ^ uVar88) & uVar14
            ^ uVar18;
    uVar1 = uVar105 ^ uVar28;
    uVar87 = (!(uVar9 & uVar91) ^ uVar108 & uVar9 ^ uVar27 ^ uVar33) & uVar106
        ^ (!(!uVar108 & uVar45) ^ uVar33) & uVar91
        ^ (uVar45 ^ uVar33) & uVar108
        ^ uVar45
        ^ uVar27
        ^ uVar33;
    uVar64 = (!(uVar1 & uVar18) ^ uVar105 ^ uVar28) & uVar14
        ^ uVar1 & (uVar18 ^ uVar14) & uVar88
        ^ uVar28;
    uVar78 = !(!uVar99 & uVar7) & 0x5da724ba ^ (uVar99 & 0x5da724ba ^ 0xa258db45) & uVar12;
    uVar83 = !((!((!((!(uVar26 & uVar83) ^ uVar2 ^ uVar96) & uVar103)
        ^ (!uVar23 ^ uVar2 ^ uVar96) & uVar26
        ^ uVar2
        ^ uVar96)
        & uVar62)
        ^ (!(uVar26 & uVar107) ^ uVar103) & uVar96
        ^ uVar103)
        & srcDwords[0x10])
        ^ uVar26 & uVar103 & uVar80;
    uVar26 = !uVar84;
    uVar72 = !(((uVar59 ^ uVar83 ^ uVar56 ^ uVar79) & uVar84 ^ uVar83) & uVar76)
        ^ uVar26 & uVar83
        ^ uVar84
        ^ uVar59;
    uVar75 = (!uVar106 & uVar33 ^ uVar45 & !uVar91) & uVar27
        ^ !(((uVar3 ^ uVar27) & uVar91 ^ uVar15 ^ uVar45 ^ uVar27 ^ uVar33) & uVar108)
        ^ uVar91;
    uVar33 = !uVar59 & uVar83;
    uVar89 = ((uVar76 ^ uVar83 ^ uVar79) & uVar59 ^ uVar83) & uVar84
        ^ !((uVar26 ^ uVar59) & uVar56) & uVar76
        ^ uVar59
        ^ uVar33;
    uVar49 = (!uVar77 & uVar61 & 0xa258db45 ^ 0x5da724ba) & uVar97 ^ !uVar17 & uVar61;
    uVar27 = !((!(uVar12 & 0xa258db45) & uVar99 ^ 0xa258db45) & uVar7) ^ uVar12;
    uVar7 = !((!(uVar7 & 0xa258db45) & uVar99 ^ 0x5da724ba) & uVar12) ^ uVar99 & 0x5da724ba ^ uVar7;
    uVar47 = (uVar94 ^ uVar50) & uVar22;
    uVar32 = !uVar47 ^ uVar94;
    uVar92 = srcDwords[7] & uVar32;
    uVar47 = !(uVar105 & uVar92) ^ uVar94 ^ uVar47;
    uVar12 = (((uVar1 & uVar22 ^ uVar105 ^ uVar28) & uVar94
        ^ uVar1 & uVar50 & uVar22
        ^ uVar105
        ^ uVar28)
        & uVar82
        & srcDwords[7]
        ^ uVar47 & uVar28)
        & srcDwords[1]
        ^ uVar28
        ^ srcDwords[7];
    uVar1 = !(((uVar66 ^ srcDwords[4]) & uVar58 ^ uVar66 ^ srcDwords[4]) & uVar44 & srcDwords[0xd])
        ^ uVar66
        ^ srcDwords[4];
    uVar106 = srcDwords[0xc];
    uVar77 = (!(!uVar49 & uVar106) & uVar24 ^ uVar106) & uVar21 ^ uVar24;
    uVar15 = (uVar24 & !uVar106 ^ uVar106) & uVar21 ^ uVar106;
    uVar17 = ((uVar80 ^ uVar95) & (uVar84 ^ uVar83) ^ uVar80 ^ uVar95) & uVar103
        ^ (uVar84 ^ uVar95 ^ uVar79) & uVar83
        ^ !(uVar95 & (uVar26 ^ uVar83)) & uVar80
        ^ (uVar95 ^ uVar79) & uVar84
        ^ uVar95;
    uVar91 = !(((uVar45 ^ uVar35) & uVar68 ^ uVar3 & uVar35) & uVar41)
        ^ !((!uVar108 ^ uVar35 ^ uVar91) & uVar68) & uVar45
        ^ uVar91;
    uVar33 = (!(uVar84 & (!uVar83 ^ uVar79)) ^ uVar59 & uVar56 ^ uVar83) & uVar76
        ^ (uVar59 & (!uVar83 ^ uVar79) ^ uVar83 ^ uVar79) & uVar84
        ^ uVar33;
    uVar3 = (uVar75 ^ uVar74) & uVar87 ^ uVar75 ^ 0xecb2d69e;
    uVar37 = uVar29 & uVar37;
    uVar20 = !((((uVar29 ^ uVar20) & uVar43 ^ uVar37) & uVar31 ^ uVar43 & uVar37) & srcDwords[0xb])
        ^ uVar59;
    uVar30 = uVar20 ^ uVar29;
    uVar32 = uVar32 & srcDwords[1];
    uVar9 = !(((uVar105 ^ uVar32) & uVar28 ^ !uVar32 & uVar105) & uVar82 & srcDwords[7])
        ^ (!(uVar47 & srcDwords[1]) ^ srcDwords[7]) & uVar28
        ^ uVar32;
    uVar47 = ((uVar27 ^ !uVar7) & uVar78 ^ uVar7 ^ uVar27) & srcDwords[0xf] ^ uVar27 & !uVar7;
    uVar61 = ((uVar18 ^ uVar14) & uVar88 ^ uVar18 & uVar14 ^ uVar82) & (uVar28 ^ !uVar105)
        ^ uVar105 & !uVar28
        ^ uVar14;
    uVar23 = !((!((uVar102 ^ uVar38) & uVar14) ^ (uVar102 ^ uVar38) & uVar88) & uVar19)
        ^ ((uVar14 ^ uVar88) & uVar102 ^ uVar14 ^ uVar88) & uVar38
        ^ !(!uVar14 & uVar18) & uVar88;
    uVar35 = ((uVar74 ^ 0xecb2d69e) & uVar75 ^ uVar74 & 0xecb2d69e ^ 0x134d2961) & uVar87
        ^ !(uVar74 & 0x134d2961) & uVar75;
    uVar21 = !((!(!uVar24 & uVar21) ^ uVar24) & uVar49 & uVar106)
        ^ (uVar21 ^ !uVar106) & uVar24
        ^ uVar21;
    uVar37 =
        !(((uVar91 ^ 0x134d2961) & uVar51 ^ !uVar91 & 0xecb2d69e) & uVar73) ^ uVar91 & 0xecb2d69e;
    uVar88 = (!uVar28 & uVar14 ^ uVar18 & (uVar28 ^ uVar14)) & uVar88
        ^ ((!uVar18 ^ uVar82) & uVar28 ^ uVar18 ^ uVar82) & uVar14
        ^ !(uVar82 & (uVar28 ^ uVar14)) & uVar105;
    uVar14 = (uVar73 & 0x134d2961 ^ 0xecb2d69e) & uVar51
        ^ !uVar73 & uVar91 & 0x134d2961
        ^ uVar73
        ^ 0xecb2d69e;
    uVar87 = (uVar75 & uVar74 ^ 0xecb2d69e) & uVar87 ^ uVar75 ^ uVar74 & 0x134d2961 ^ 0xecb2d69e;
    uVar97 = ((!uVar67 ^ uVar60) & srcDwords[0x15] ^ uVar67 ^ uVar60) & uVar6
        ^ (uVar67 ^ uVar60) & srcDwords[0x15];
    uVar41 = !(((!uVar61 ^ uVar25 ^ uVar70) & uVar88 ^ uVar61 ^ uVar25 ^ uVar70) & uVar69)
        ^ (uVar88 ^ uVar69) & uVar61 & uVar64
        ^ uVar25;
    uVar49 = srcDwords[0x12];
    uVar68 = (!((!uVar3 & uVar35 ^ uVar3) & uVar49) ^ uVar35) & uVar87 ^ uVar35 & uVar49;
    uVar5 = !((((srcDwords[4] ^ srcDwords[0xd]) & uVar66
        ^ !srcDwords[4] & srcDwords[0xd]
        ^ srcDwords[4])
        & uVar58
        ^ (uVar5 ^ uVar66) & srcDwords[4])
        & uVar44)
        ^ (uVar5 ^ srcDwords[4]) & uVar66
        ^ srcDwords[0xd];
    uVar106 = (!((!uVar38 ^ uVar5) & uVar1) ^ uVar38 ^ uVar5) & uVar40
        ^ ((uVar54 ^ uVar19 ^ uVar1) & uVar38 ^ uVar102) & uVar5
        ^ uVar54 & uVar38
        ^ uVar102
        ^ uVar1;
    uVar24 = !(((uVar102 ^ uVar19) & uVar38 ^ (uVar40 ^ uVar38) & uVar1 ^ uVar40 ^ uVar102)
        & uVar5)
        ^ (!(!uVar40 & uVar1) ^ uVar40 ^ uVar19) & uVar38
        ^ uVar1;
    uVar44 = (!(uVar3 & uVar49) ^ uVar87) & uVar35 ^ (uVar87 ^ uVar3) & uVar49 ^ uVar87;
    uVar74 = !uVar29;
    uVar60 = !((!((!(!uVar6 & uVar60) ^ uVar6) & srcDwords[0x15]) ^ uVar6) & uVar67)
        ^ (srcDwords[0x15] ^ uVar60) & uVar6
        ^ uVar60;
    uVar54 = (((uVar59 ^ uVar29) & uVar56 ^ uVar59 & uVar74) & uVar76
        ^ !(uVar74 & uVar56) & uVar59
        ^ uVar29)
        & srcDwords[0x19]
        ^ ((uVar36 & uVar43 ^ uVar4) & uVar31 ^ uVar4 & uVar43) & srcDwords[0xb];
    uVar45 = (!(!uVar27 & uVar7) ^ uVar27) & uVar78 & srcDwords[0xf] ^ uVar7 ^ uVar27;
    uVar6 = !((!((uVar40 ^ uVar102 ^ uVar19 ^ uVar5) & uVar38) ^ uVar102) & uVar1)
        ^ (uVar40 ^ uVar19 ^ uVar5) & uVar38
        ^ uVar5;
    uVar4 = (uVar54 ^ uVar30) & (uVar31 ^ uVar29);
    uVar56 = (!(uVar54 & (uVar31 ^ uVar29)) ^ uVar31 ^ uVar29) & uVar30 ^ uVar4 & uVar8 ^ uVar29;
    uVar58 = !((!((uVar84 ^ uVar79) & uVar83)
        ^ (uVar26 ^ uVar95) & uVar79
        ^ uVar103 & (uVar46 ^ uVar79))
        & uVar80)
        ^ (!uVar103 & uVar95 ^ uVar26 & uVar83 ^ uVar84) & uVar79
        ^ uVar84
        ^ uVar83;
    uVar59 = !uVar88 ^ uVar64;
    uVar66 = uVar59 & uVar61;
    uVar49 = !uVar23 ^ uVar90;
    uVar18 = !(((uVar49 ^ uVar71) & uVar48 ^ uVar71) & uVar60)
        ^ !((uVar60 ^ uVar48) & uVar97) & uVar71
        ^ uVar49 & uVar48
        ^ uVar90;
    uVar49 = !uVar5;
    uVar19 = !((!(uVar81 & (uVar49 ^ uVar1)) ^ uVar49 & uVar1 ^ uVar5) & uVar40)
        ^ ((uVar5 ^ uVar10) & uVar81 ^ uVar49 & uVar10) & uVar57
        ^ uVar5
        ^ uVar10;
    uVar7 = !((!(!uVar78 & srcDwords[0xf]) & uVar27 ^ srcDwords[0xf]) & uVar7) ^ srcDwords[0xf];
    uVar27 = (uVar73 ^ 0xecb2d69e) & uVar51 & uVar91 ^ (uVar91 ^ 0xecb2d69e) & uVar73 ^ 0xecb2d69e;
    uVar51 = !((uVar54 ^ uVar4 ^ uVar30) & uVar8)
        ^ !(uVar54 & (!uVar31 ^ uVar29)) & uVar30
        ^ uVar31 & uVar74
        ^ (!uVar31 ^ uVar29) & uVar43;
    uVar4 = uVar40 & (uVar49 ^ uVar1);
    uVar1 = ((uVar49 ^ uVar10) & uVar57 ^ uVar4 ^ uVar5 ^ uVar10) & uVar81
        ^ (uVar40 & uVar1 ^ uVar52) & uVar5
        ^ uVar10;
    uVar49 = uVar106 ^ !uVar6;
    uVar36 = (uVar49 & uVar15 ^ uVar6 ^ uVar106) & uVar77
        ^ ((uVar15 ^ uVar77) & uVar21 ^ uVar24) & uVar49
        ^ uVar106;
    uVar5 = (!uVar4 ^ uVar57) & uVar81 ^ (uVar4 ^ uVar57) & uVar10 ^ uVar5;
    uVar10 = !(((!(uVar82 & !uVar105) ^ uVar105) & srcDwords[7] ^ uVar32) & uVar28)
        ^ uVar92 & srcDwords[1];
    uVar32 = (!uVar35 & uVar3 & srcDwords[0x12] ^ uVar35) & uVar87 ^ srcDwords[0x12];
    uVar87 = !uVar54;
    uVar20 = ((uVar87 ^ uVar29) & uVar30 ^ uVar54 & uVar29) & uVar8
        ^ ((uVar87 ^ uVar43) & uVar29 ^ uVar54 ^ uVar43) & uVar30
        ^ !(uVar20 & uVar43) & uVar31;
    uVar4 = !uVar10;
    uVar3 = (uVar13 ^ uVar10 ^ uVar104 ^ uVar9) & uVar12;
    uVar52 = !((uVar4 & uVar9 ^ uVar10 ^ uVar3) & uVar53)
        ^ (uVar10 & uVar9 ^ uVar13 ^ uVar104) & uVar12
        ^ uVar10
        ^ uVar104;
    uVar49 = (uVar4 ^ uVar104) & uVar13;
    uVar91 = uVar10 & !uVar104;
    uVar28 = !uVar89 ^ uVar72;
    uVar67 = !(((uVar104 ^ uVar9) & uVar10 ^ uVar49 ^ uVar3 ^ uVar104 ^ uVar9) & uVar53)
        ^ ((uVar10 ^ uVar9) & uVar104 ^ uVar13 ^ uVar10 ^ uVar9) & uVar12
        ^ (!uVar91 ^ uVar104) & uVar9
        ^ uVar10
        ^ uVar49
        ^ uVar104;
    uVar57 = !((!(uVar44 & uVar28) ^ uVar28 & uVar68 ^ uVar89 ^ uVar72) & uVar32)
        ^ (!uVar68 ^ uVar33 ^ uVar89) & uVar72
        ^ (uVar28 & uVar68 ^ uVar89 ^ uVar72) & uVar44
        ^ (uVar68 ^ uVar33) & uVar89
        ^ uVar68
        ^ uVar33;
    uVar95 = ((uVar95 ^ uVar79 ^ uVar26 ^ uVar83) & uVar103
        ^ (uVar26 ^ uVar95 ^ uVar79) & uVar83
        ^ uVar84 & (uVar46 ^ uVar79)
        ^ uVar46 & uVar79)
        & uVar80
        ^ ((uVar79 ^ uVar84 ^ uVar83) & uVar103 ^ uVar84 ^ uVar83 ^ uVar79) & uVar95
        ^ !(uVar84 & uVar83) & uVar79;
    uVar49 = srcDwords[3];
    uVar29 = (!uVar49 & uVar14 ^ uVar49) & uVar27 ^ uVar14;
    uVar31 = !((!((!uVar37 & uVar14 ^ uVar37) & uVar27) ^ uVar14) & uVar49) ^ uVar27;
    uVar26 = !uVar50;
    uVar87 = uVar87 ^ uVar8;
    uVar35 = !(((uVar54 ^ uVar8) & (uVar26 ^ uVar22) ^ uVar50 ^ uVar22) & uVar94)
        ^ (!(uVar87 & uVar50) ^ uVar54 ^ uVar8) & uVar22
        ^ (uVar30 ^ uVar50) & uVar87
        ^ uVar54;
    uVar3 = (!uVar65 ^ uVar55) & uVar63 & 0xecb2d69e ^ uVar55;
    uVar38 = !uVar62 ^ uVar10;
    uVar46 = ((uVar4 ^ uVar9) & uVar12 ^ uVar62 & uVar10 ^ uVar38 & uVar96 ^ uVar9) & uVar2
        ^ (!uVar96 & uVar62 ^ !uVar12 & uVar9) & uVar10
        ^ uVar62
        ^ uVar12;
    uVar4 = (uVar2 ^ uVar62) & uVar96;
    uVar80 = (!(!uVar55 & uVar65 & 0x134d2961) ^ uVar55) & uVar63
        ^ (uVar55 ^ 0x134d2961) & uVar65
        ^ 0x134d2961;
    uVar40 = (!((uVar2 ^ uVar10 ^ uVar9) & uVar62) ^ uVar2 ^ uVar4) & uVar12
        ^ (!(uVar93 & uVar96) ^ uVar10 ^ uVar9) & uVar62
        ^ uVar2
        ^ uVar10;
    uVar63 = ((uVar65 & 0x134d2961 ^ 0xecb2d69e) & uVar63 ^ !uVar65 & 0xecb2d69e) & uVar55
        ^ uVar65 & 0xecb2d69e;
    uVar14 = (!(uVar49 & uVar37) & uVar27 ^ uVar49) & uVar14 ^ uVar27 & !uVar49;
    uVar27 = (!((!uVar3 ^ uVar63) & uVar80) ^ uVar3 ^ uVar63) & srcDwords[0x18] ^ uVar3 ^ uVar80;
    uVar49 = !((uVar50 ^ uVar22) & uVar94) ^ uVar26 & uVar22;
    uVar37 = (uVar49 ^ uVar54 ^ uVar50) & uVar8 ^ (uVar49 ^ uVar50) & uVar54 ^ uVar94;
    uVar49 = (srcDwords[0x18] ^ uVar63) & uVar80;
    uVar43 = !((!srcDwords[0x18] & uVar63 ^ srcDwords[0x18]) & uVar3) ^ uVar49;
    uVar53 = ((uVar53 ^ uVar9) & uVar10 ^ uVar53 ^ uVar9) & uVar104
        ^ (!((uVar10 ^ uVar104) & uVar9) ^ uVar91) & uVar12
        ^ (uVar53 & (uVar10 ^ uVar104) ^ uVar10 ^ uVar104) & uVar13
        ^ uVar53;
    uVar8 = ((uVar54 ^ uVar50) & uVar22 ^ uVar87 & uVar30 ^ uVar54 & uVar26) & uVar94
        ^ (uVar30 & uVar8 ^ uVar26 & uVar22 ^ uVar50) & uVar54
        ^ uVar8;
    uVar63 =
        ((uVar63 ^ uVar80) & srcDwords[0x18] ^ uVar63) & uVar3 ^ uVar49 ^ srcDwords[0x18] ^ uVar63;
    uVar49 = !((!uVar7 ^ uVar45) & uVar17);
    uVar13 = !(((!uVar7 ^ uVar45) & uVar95 ^ uVar49 ^ uVar7 ^ uVar45) & uVar58)
        ^ (!(!uVar47 & uVar7) ^ uVar47 ^ uVar17) & uVar45
        ^ (uVar49 ^ uVar7 ^ uVar45) & uVar95
        ^ (uVar47 ^ uVar17) & uVar7
        ^ uVar47;
    uVar22 = (!((uVar31 ^ uVar19) & uVar1) ^ uVar31 ^ uVar19) & uVar5
        ^ (!((uVar29 ^ uVar1 ^ uVar14) & uVar19) ^ uVar14) & uVar31
        ^ (uVar29 ^ uVar1) & uVar19;
    uVar2 = !(((uVar38 ^ uVar9 ^ uVar96) & uVar2 ^ (uVar10 ^ uVar96) & uVar62 ^ uVar38 & uVar9)
        & uVar12)
        ^ (uVar62 & uVar93 ^ !uVar4) & uVar10
        ^ (uVar62 ^ uVar10 ^ uVar93) & uVar9
        ^ uVar2;
    uVar62 = (!uVar63 ^ uVar43) & uVar27;
    uVar3 = !uVar51 & uVar56 ^ uVar62 ^ uVar51;
    uVar9 = (!((!uVar63 ^ uVar43) & uVar51) ^ uVar63 ^ uVar43) & uVar27
        ^ (!uVar56 ^ uVar63 ^ uVar43) & uVar51
        ^ (uVar3 ^ uVar63 ^ uVar43) & uVar20
        ^ uVar56
        ^ uVar43;
    uVar55 = (!uVar86 ^ uVar42) & uVar16;
    uVar4 = ((uVar35 ^ uVar16) & uVar37 ^ uVar55 ^ uVar42) & uVar8
        ^ (!uVar35 & uVar37 ^ uVar86) & uVar16
        ^ uVar35;
    uVar49 = (uVar34 ^ uVar11) & uVar2;
    uVar30 = !uVar34 & uVar101;
    uVar38 = !(((uVar34 ^ uVar11) & uVar40 ^ !uVar49 ^ uVar34 ^ uVar11) & uVar46)
        ^ (uVar49 ^ uVar34 ^ uVar11) & uVar40
        ^ !uVar30 & uVar11
        ^ uVar34
        ^ uVar2;
    uVar49 = (!uVar11 ^ uVar101) & uVar34;
    uVar55 = !((!uVar55 ^ uVar42) & uVar8) ^ (uVar55 ^ uVar42) & uVar35 ^ uVar16;
    uVar12 = !uVar67;
    uVar10 = (uVar39 & (uVar67 ^ uVar52) ^ uVar67 ^ uVar52) & uVar98
        ^ !(uVar85 & (uVar67 ^ uVar52)) & uVar39
        ^ uVar52;
    uVar26 = (uVar52 ^ uVar12) & uVar53;
    uVar49 = !(!((!uVar49 ^ uVar46 ^ uVar11 ^ uVar101) & uVar2)
        ^ (uVar49 ^ uVar11 ^ uVar101) & uVar46
        ^ uVar34
        ^ uVar11);
    uVar12 = ((uVar85 ^ uVar52 ^ uVar98) & uVar67 ^ uVar98 ^ uVar26) & uVar39
        ^ (!(uVar52 & uVar12) ^ uVar67) & uVar53
        ^ uVar98 & uVar12
        ^ uVar67
        ^ uVar52;
    dstDwords[0] = !((!((!((uVar95 ^ uVar7 ^ uVar47) & uVar17)
        ^ (!uVar95 ^ uVar17) & uVar58
        ^ uVar95)
        & uVar45)
        ^ (uVar95 & uVar58 ^ uVar7 ^ uVar47) & uVar17
        ^ uVar7
        ^ uVar13)
        & (((uVar95 ^ uVar17) & (uVar7 ^ uVar45) ^ uVar7 ^ uVar45) & uVar58
            ^ (uVar17 & (uVar7 ^ uVar45) ^ uVar7 ^ uVar45) & uVar95
            ^ (!uVar47 & uVar45 ^ uVar47) & uVar7
            ^ uVar17))
        ^ ((!((uVar46 ^ uVar34 ^ uVar40) & uVar11)
            ^ (uVar46 ^ uVar101 ^ uVar40) & uVar34
            ^ uVar101
            ^ uVar40)
            & uVar2
            ^ (!((!uVar101 ^ uVar40) & uVar34) ^ (!uVar34 ^ uVar40) & uVar11 ^ uVar101 ^ uVar40)
                & uVar46
            ^ (uVar30 ^ uVar34 ^ uVar40) & uVar11
            ^ (uVar101 ^ uVar40) & uVar34
            ^ uVar101
            ^ uVar40)
            & (uVar38 ^ uVar49)
        ^ uVar38 & uVar49
        ^ uVar13;
    uVar47 = !(((!uVar31 ^ uVar19) & uVar1 ^ (!uVar29 ^ uVar14) & uVar31 ^ uVar29) & uVar5
        ^ (uVar1 & uVar19 ^ uVar14) & uVar31
        ^ uVar19);
    uVar49 = !(!((!((!uVar60 ^ uVar23) & uVar48) ^ (uVar60 ^ uVar48) & uVar71 ^ uVar60) & uVar90)
        ^ ((uVar60 ^ uVar23) & uVar71 ^ uVar60 & !uVar23) & uVar48
        ^ !((uVar60 ^ uVar90 ^ uVar48) & uVar97) & uVar71);
    dstDwords[1] = !(((!((uVar68 ^ uVar89) & uVar32)
        ^ (uVar68 ^ uVar72) & uVar89
        ^ uVar28 & uVar33
        ^ uVar72)
        & uVar44
        ^ (uVar33 & uVar72 ^ uVar32 & !uVar68 ^ uVar68) & uVar89
        ^ uVar57
        ^ uVar72)
        & (((uVar44 ^ uVar68) & (uVar89 ^ uVar72) ^ uVar89 ^ uVar72) & uVar32
            ^ (uVar44 & (uVar89 ^ uVar72) ^ uVar89 ^ uVar72) & uVar68
            ^ !uVar89 & uVar72
            ^ uVar44
            ^ uVar89))
        ^ (!(((uVar5 ^ uVar19) & (!uVar29 ^ uVar14) ^ uVar29 ^ uVar14) & uVar31)
            ^ (!uVar5 ^ uVar19) & uVar29
            ^ uVar5)
            & (uVar47 ^ uVar22)
        ^ uVar47 & uVar22
        ^ uVar57;
    dstDwords[2] = !((!(((!uVar85 ^ uVar67 ^ uVar98) & uVar52 ^ uVar85 ^ uVar67 ^ uVar26)
        & uVar39)
        ^ (uVar67 & uVar53 ^ uVar98) & uVar52
        ^ uVar67)
        & (uVar12 ^ uVar10))
        ^ ((!((uVar60 ^ uVar97 ^ uVar48) & uVar90) ^ uVar60 ^ uVar97 ^ uVar48) & uVar71
            ^ !((uVar90 ^ uVar71) & uVar23) & uVar48
            ^ uVar60
            ^ uVar90)
            & (uVar18 ^ uVar49)
        ^ uVar12 & uVar10
        ^ uVar18 & uVar49;
    uVar48 = !((!(uVar59 & uVar69) ^ uVar88 ^ uVar64) & uVar61
        ^ !((!uVar66 ^ uVar70 & uVar69) & uVar25)
        ^ uVar88);
    dstDwords[3] = (((uVar20 ^ uVar56 ^ uVar27) & uVar51 ^ uVar20 ^ uVar56) & uVar63
        ^ ((uVar51 ^ uVar63) & uVar27 ^ uVar51 ^ uVar63) & uVar43
        ^ !uVar27 & uVar51
        ^ uVar20
        ^ uVar9)
        & ((uVar3 ^ uVar43) & uVar20 ^ (uVar62 ^ uVar43) & uVar51 ^ uVar63)
        ^ (!((!((uVar86 ^ uVar42 ^ uVar37) & uVar35) ^ uVar42 ^ uVar37) & uVar16)
            ^ ((!uVar35 ^ uVar16) & uVar37 ^ uVar35 ^ uVar16) & uVar8
            ^ (uVar42 ^ uVar37) & uVar35
            ^ uVar42
            ^ uVar37)
            & (uVar55 ^ uVar4)
        ^ !uVar55 & uVar4
        ^ uVar55
        ^ uVar9;
    dstDwords[4] = (!((uVar15 & (uVar6 ^ uVar77) ^ !uVar6 & uVar77) & uVar21)
        ^ (!((uVar24 ^ uVar15) & uVar6) ^ uVar24 ^ uVar15) & uVar77
        ^ (!(uVar24 & (uVar6 ^ uVar77)) ^ uVar6 ^ uVar77) & uVar106
        ^ uVar36)
        & (!(uVar15 & (uVar6 ^ uVar106)) & uVar77
            ^ (uVar15 ^ uVar77) & uVar21 & (uVar6 ^ uVar106)
            ^ !uVar106 & uVar6
            ^ uVar106)
        ^ ((!((!uVar88 ^ uVar70) & uVar69) ^ uVar66 ^ uVar88) & uVar25
            ^ (uVar61 & uVar64 ^ !(uVar70 & uVar69)) & uVar88
            ^ uVar69)
            & (uVar41 ^ uVar48)
        ^ uVar41 & uVar48
        ^ uVar36;
    for (i, &w) in dstDwords.iter().enumerate() {
        destination[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
}
