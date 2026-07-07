// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver ClientApi/ItemAddress.cs, LGPL-3.0-or-later.

//! `ItemAddress` — the symbolic address of a variable for Get/SetMultiVariables.
//!
//! An address is the symbol CRC, the access area (base relation id), the access sub-area,
//! and a list of LIDs (link/location ids) that walk into the symbol's structure. All
//! fields are VLQ-encoded; the serialized LID count is `LID.len() + 1` (the reference adds
//! one for the leading sub-area entry).

use std::io::Write;

use crate::error::Result;
use crate::wire::vlq;

/// A symbolic item address.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemAddress {
    /// CRC of the symbol name (or 0 when addressing purely by area + LID path).
    pub symbol_crc: u32,
    /// Access area — the DB relation id, or the M/Q/I area RID.
    pub access_area: u32,
    /// Access sub-area (e.g. `DB_ValueActual`).
    pub access_sub_area: u32,
    /// Link/location ids walking into the symbol's structure.
    pub lid: Vec<u32>,
}

impl ItemAddress {
    /// Serialize the address. Returns bytes written.
    pub fn serialize<W: Write>(&self, w: &mut W) -> Result<usize> {
        let mut n = 0;
        n += vlq::encode_u32(w, self.symbol_crc)?;
        n += vlq::encode_u32(w, self.access_area)?;
        n += vlq::encode_u32(w, self.lid.len() as u32 + 1)?;
        n += vlq::encode_u32(w, self.access_sub_area)?;
        for id in &self.lid {
            n += vlq::encode_u32(w, *id)?;
        }
        Ok(n)
    }

    /// Number of serialized VLQ fields this address contributes to the request's total
    /// field-count tally: the 4 leading fields (symbol_crc, access_area, id-count,
    /// access_sub_area) plus one per LID. Matches the reference `GetNumberOfFields`.
    pub fn field_count(&self) -> u32 {
        4 + self.lid.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_layout() {
        let addr = ItemAddress {
            symbol_crc: 0,
            access_area: 3,
            access_sub_area: 0,
            lid: vec![1, 2],
        };
        let mut out = Vec::new();
        addr.serialize(&mut out).unwrap();
        // symbol_crc=0, access_area=3, count=lid+1=3, sub_area=0, lids 1,2
        assert_eq!(out, vec![0x00, 0x03, 0x03, 0x00, 0x01, 0x02]);
        assert_eq!(addr.field_count(), 6); // 4 leading fields + 2 LIDs
    }

    #[test]
    fn vlq_widths_for_large_ids() {
        let addr = ItemAddress {
            symbol_crc: 0x4000,
            access_area: 0,
            access_sub_area: 0,
            lid: vec![],
        };
        let mut out = Vec::new();
        addr.serialize(&mut out).unwrap();
        // symbol_crc 0x4000 -> 81 80 00; access_area 0 -> 00; count 1 -> 01; sub_area 0 -> 00
        assert_eq!(out, vec![0x81, 0x80, 0x00, 0x00, 0x01, 0x00]);
    }
}
