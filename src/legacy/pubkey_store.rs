// SPDX-License-Identifier: LGPL-3.0-or-later
// GENERATED: bundled S7-1500/1200 public keys from bonk-dev/HarpoS7
// (HarpoS7.PublicKeys/Keys/{00,01}), MIT. Indexed by fingerprint (family + key id).

use super::family0::blob::PublicKeyFamily;

/// Look up a bundled public key by family + uppercase-hex fingerprint id (16 hex chars).
pub fn lookup(family: PublicKeyFamily, fingerprint: &str) -> Option<&'static [u8]> {
    match (family, fingerprint) {
        (PublicKeyFamily::S71500, "0448ACCBD5A0BFD2") => {
            Some(include_bytes!("pubkeys/00/0448ACCBD5A0BFD2.bin"))
        }
        (PublicKeyFamily::S71500, "181B7B0847D11694") => {
            Some(include_bytes!("pubkeys/00/181B7B0847D11694.bin"))
        }
        (PublicKeyFamily::S71500, "1B580465BB0551B2") => {
            Some(include_bytes!("pubkeys/00/1B580465BB0551B2.bin"))
        }
        (PublicKeyFamily::S71500, "2C1DD211E529278D") => {
            Some(include_bytes!("pubkeys/00/2C1DD211E529278D.bin"))
        }
        (PublicKeyFamily::S71500, "580B8A122D42D1C0") => {
            Some(include_bytes!("pubkeys/00/580B8A122D42D1C0.bin"))
        }
        (PublicKeyFamily::S71500, "60CDAAA33E0B5D20") => {
            Some(include_bytes!("pubkeys/00/60CDAAA33E0B5D20.bin"))
        }
        (PublicKeyFamily::S71500, "65227E2580029B7F") => {
            Some(include_bytes!("pubkeys/00/65227E2580029B7F.bin"))
        }
        (PublicKeyFamily::S71500, "6BA412F7F1D965AA") => {
            Some(include_bytes!("pubkeys/00/6BA412F7F1D965AA.bin"))
        }
        (PublicKeyFamily::S71500, "99E4632334CC7993") => {
            Some(include_bytes!("pubkeys/00/99E4632334CC7993.bin"))
        }
        (PublicKeyFamily::S71500, "ACD68E9BF9901F8B") => {
            Some(include_bytes!("pubkeys/00/ACD68E9BF9901F8B.bin"))
        }
        (PublicKeyFamily::S71500, "C4F47B876DA76D52") => {
            Some(include_bytes!("pubkeys/00/C4F47B876DA76D52.bin"))
        }
        (PublicKeyFamily::S71500, "D3F9CD55A57FE4EB") => {
            Some(include_bytes!("pubkeys/00/D3F9CD55A57FE4EB.bin"))
        }
        (PublicKeyFamily::S71500, "E69E7A996524AFAC") => {
            Some(include_bytes!("pubkeys/00/E69E7A996524AFAC.bin"))
        }
        (PublicKeyFamily::S71200, "A95850575DF7B3DE") => {
            Some(include_bytes!("pubkeys/01/A95850575DF7B3DE.bin"))
        }
        (PublicKeyFamily::S71200, "AC9BE476CB324E65") => {
            Some(include_bytes!("pubkeys/01/AC9BE476CB324E65.bin"))
        }
        (PublicKeyFamily::S71200, "BD426B091F08731A") => {
            Some(include_bytes!("pubkeys/01/BD426B091F08731A.bin"))
        }
        _ => None,
    }
}

/// Every bundled public key for a family, in fingerprint order. Used to auto-try keys when a PLC
/// advertises only its family (not its key-id fingerprint), so its key can't be looked up directly.
pub fn candidates(family: PublicKeyFamily) -> &'static [&'static [u8]] {
    match family {
        PublicKeyFamily::S71500 => S71500_KEYS,
        PublicKeyFamily::S71200 => S71200_KEYS,
    }
}

static S71500_KEYS: &[&[u8]] = &[
    include_bytes!("pubkeys/00/0448ACCBD5A0BFD2.bin"),
    include_bytes!("pubkeys/00/181B7B0847D11694.bin"),
    include_bytes!("pubkeys/00/1B580465BB0551B2.bin"),
    include_bytes!("pubkeys/00/2C1DD211E529278D.bin"),
    include_bytes!("pubkeys/00/580B8A122D42D1C0.bin"),
    include_bytes!("pubkeys/00/60CDAAA33E0B5D20.bin"),
    include_bytes!("pubkeys/00/65227E2580029B7F.bin"),
    include_bytes!("pubkeys/00/6BA412F7F1D965AA.bin"),
    include_bytes!("pubkeys/00/99E4632334CC7993.bin"),
    include_bytes!("pubkeys/00/ACD68E9BF9901F8B.bin"),
    include_bytes!("pubkeys/00/C4F47B876DA76D52.bin"),
    include_bytes!("pubkeys/00/D3F9CD55A57FE4EB.bin"),
    include_bytes!("pubkeys/00/E69E7A996524AFAC.bin"),
];

static S71200_KEYS: &[&[u8]] = &[
    include_bytes!("pubkeys/01/A95850575DF7B3DE.bin"),
    include_bytes!("pubkeys/01/AC9BE476CB324E65.bin"),
    include_bytes!("pubkeys/01/BD426B091F08731A.bin"),
];
