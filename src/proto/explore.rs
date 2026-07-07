// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/ExploreRequest.cs +
// Core/ExploreResponse.cs, LGPL-3.0-or-later.

//! `Explore` (functioncode 0x04bb) — discover the PLC's object/symbol tree.
//!
//! The request names a root object (`explore_id`), whether to recurse into children, and
//! whether to walk parents; the response is a list of [`PObject`]s (decoded via
//! [`crate::proto::object::decode_object_list`]).

use std::io::Cursor;

use crate::error::{Error, Result};
use crate::proto::header::{RequestHeader, ResponseHeader};
use crate::proto::object::{decode_object_list, PObject};
use crate::wire::pdu::{self, functioncode};
use crate::wire::{primitives as p, vlq};

const TRANSPORT_FLAGS: u8 = 0x34;

/// Build a framed `ExploreRequest`.
///
/// Mirrors `ExploreRequest.Serialize`: header, then the request set (explore id, request
/// id, recursion/parents flags), an (omitted) filter, an empty following-objects marker,
/// the address list, the optional integrity id, and the trailing `u32 + byte` fill that
/// PLCSIM requires (5 bytes; 4 alone makes it not respond).
#[allow(clippy::too_many_arguments)]
pub fn build_explore_request(
    protocol_version: u8,
    sequence_number: u16,
    session_id: u32,
    explore_id: u32,
    explore_request_id: u32,
    explore_childs_recursive: u8,
    explore_parents: u8,
    address_list: &[u32],
    with_integrity: bool,
    integrity_id: u32,
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::EXPLORE,
        sequence_number,
        session_id,
        transport_flags: TRANSPORT_FLAGS,
    };

    let mut body = Vec::new();
    header.serialize(&mut body)?;

    // Request set.
    p::encode_u32(&mut body, explore_id)?; // fixed-width
    vlq::encode_u32(&mut body, explore_request_id)?;
    p::encode_u8(&mut body, explore_childs_recursive)?;
    p::encode_u8(&mut body, 1)?; // unknown (0 or 1)
    p::encode_u8(&mut body, explore_parents)?;
    // FilterData omitted (null) — skip the `byte 1 + ValueStruct` block.
    p::encode_u8(&mut body, 0)?; // number of following objects / unknown

    vlq::encode_u32(&mut body, address_list.len() as u32)?;
    for id in address_list {
        vlq::encode_u32(&mut body, *id)?;
    }
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    p::encode_u32(&mut body, 0)?; // fill
    p::encode_u8(&mut body, 0)?; // extra fill byte (PLCSIM wants 5 bytes here)

    Ok(pdu::frame_single_pdu(protocol_version, &body))
}

/// A parsed `ExploreResponse`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExploreResponse {
    /// The response header.
    pub header: ResponseHeader,
    /// The explore id echoed by the PLC.
    pub explore_id: u32,
    /// Integrity id for the next request in the session.
    pub integrity_id: u32,
    /// The explored objects.
    pub objects: Vec<PObject>,
}

/// Parse an `ExploreResponse` telegram (mirrors `ExploreResponse.Deserialize`).
pub fn parse_explore_response(buf: &[u8], with_integrity: bool) -> Result<ExploreResponse> {
    let pdu_header = pdu::parse_header(buf)?;
    let mut cur = Cursor::new(&buf[pdu_header.body_offset..]);

    let header = ResponseHeader::read(&mut cur)?;
    if header.function_code != functioncode::EXPLORE {
        return Err(Error::protocol(format!(
            "ExploreResponse: expected function 0x{:04x}, got 0x{:04x}",
            functioncode::EXPLORE,
            header.function_code
        )));
    }

    let explore_id = p::decode_u32(&mut cur)?; // fixed-width
    let integrity_id = if with_integrity {
        vlq::decode_u32(&mut cur)?
    } else {
        0
    };

    let objects = decode_object_list(&mut cur)?;

    Ok(ExploreResponse {
        header,
        explore_id,
        integrity_id,
        objects,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::PValue;
    use crate::wire::pdu::{ids, protocol_version};

    #[test]
    fn build_request_byte_exact() {
        // Shallow explore of the PLC program (rid 3), seq 4, session 0x70000fd7,
        // integrity id 1 — the request validated as accepted by a live PLCSIM Advanced 8.
        let framed = build_explore_request(
            protocol_version::V2,
            4,
            0x7000_0fd7,
            3,
            ids::NONE,
            0,
            0,
            &[],
            true,
            1,
        )
        .unwrap();
        let expected = vec![
            0x72, 0x02, 0x00, 0x1e, // PDU header V2, len 30
            0x31, 0x00, 0x00, 0x04, 0xbb, 0x00, 0x00, 0x00, 0x04, // op, rsv, func, rsv, seq
            0x70, 0x00, 0x0f, 0xd7, // session id
            0x34, // transport flags
            0x00, 0x00, 0x00, 0x03, // ExploreId = 3 (fixed)
            0x00, // ExploreRequestId VLQ = 0
            0x00, // recursive = 0
            0x01, // unknown = 1
            0x00, // parents = 0
            0x00, // following objects = 0
            0x00, // address list count = 0
            0x01, // integrity id VLQ = 1
            0x00, 0x00, 0x00, 0x00, // fill u32
            0x00, // fill byte
            0x72, 0x02, 0x00, 0x00, // trailer
        ];
        assert_eq!(framed, expected);
    }

    #[test]
    fn parse_plc_program_golden() {
        // 152-byte shallow Explore(rid=3) response captured live from PLCSIM Advanced 8
        // (TIA V21). Exercises the object decoder + WString/array/scalar value decode.
        let raw: Vec<u8> = vec![
            0x72, 0x02, 0x00, 0x90, 0x32, 0x00, 0x00, 0x04, 0xbb, 0x00, 0x00, 0x00, 0x04, 0x34,
            0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0xa1, 0x00, 0x00, 0x00, 0x03, 0x93, 0x58, 0x30,
            0x00, 0xa3, 0x81, 0x69, 0x00, 0x15, 0x0a, 0x50, 0x4c, 0x43, 0x50, 0x72, 0x6f, 0x67,
            0x72, 0x61, 0x6d, 0xa3, 0x93, 0x15, 0x00, 0x05, 0x8c, 0xaf, 0xba, 0xd0, 0x9f, 0x9a,
            0xc9, 0x94, 0x04, 0xa3, 0x93, 0x16, 0x00, 0x04, 0x00, 0xa3, 0x93, 0x2f, 0x00, 0x04,
            0x00, 0xa3, 0x9c, 0x33, 0x00, 0x01, 0x01, 0xa3, 0x9d, 0x22, 0x00, 0x05, 0x00, 0xa3,
            0xa1, 0x2f, 0x00, 0x03, 0x04, 0x09, 0xa3, 0xa3, 0x3f, 0x10, 0x03, 0x02, 0xea, 0x60,
            0xee, 0x47, 0xa3, 0xa4, 0x14, 0x00, 0x01, 0x01, 0xa3, 0xbb, 0x26, 0x00, 0x0c, 0x00,
            0x00, 0x00, 0x00, 0xa3, 0xbc, 0x31, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xbc,
            0x32, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xbf, 0x14,
            0x00, 0x04, 0x00, 0xa2, 0x00, 0x00, 0x00, 0x00, 0x72, 0x02, 0x00, 0x00,
        ];
        let resp = parse_explore_response(&raw, true).unwrap();
        assert!(resp.header.is_ok());
        assert_eq!(resp.integrity_id, 5); // request seq 4 + integrity 1
        assert_eq!(resp.objects.len(), 1);

        let obj = &resp.objects[0];
        assert_eq!(obj.relation_id, 3);
        assert_eq!(obj.class_id, 2520);
        assert_eq!(obj.attributes.len(), 13);

        let attr = |id: u32| {
            obj.attributes
                .iter()
                .find(|(k, _)| *k == id)
                .map(|(_, v)| v)
        };
        assert_eq!(attr(233), Some(&PValue::WString("PLCProgram".to_string())));
        assert_eq!(attr(4271), Some(&PValue::UInt(1033)));
        assert_eq!(
            attr(4543),
            Some(&PValue::Array {
                element_type: crate::value::datatype::tag::UINT,
                flags: crate::value::datatype::flags::ARRAY,
                items: vec![PValue::UInt(0xea60), PValue::UInt(0xee47)],
            })
        );
    }

    #[test]
    fn parse_type_info_golden() {
        // 195-byte Explore(ti_relid=0x92000001) response captured live from PLCSIM Advanced
        // 8 — the type info for Data_block_1, carrying its VartypeList (member LIDs) and
        // VarnameList (member names). Exercises the type-metadata decoder end-to-end.
        let raw: Vec<u8> = vec![
            0x72, 0x02, 0x00, 0xbb, 0x32, 0x00, 0x00, 0x04, 0xbb, 0x00, 0x00, 0x00, 0x04, 0x34,
            0x00, 0x00, 0x00, 0x02, 0x19, 0x05, 0xa1, 0x92, 0x00, 0x00, 0x01, 0x83, 0x7f, 0x20,
            0x00, 0xa3, 0x81, 0x69, 0x00, 0x15, 0x00, 0xa3, 0x83, 0x08, 0x00, 0x0c, 0x80, 0x00,
            0x00, 0x03, 0xa3, 0x83, 0x14, 0x00, 0x04, 0x00, 0xa3, 0x84, 0x11, 0x00, 0x05, 0xba,
            0xab, 0xc5, 0xab, 0xdd, 0xea, 0x9c, 0xdf, 0x88, 0xa3, 0x8b, 0x5e, 0x00, 0x04, 0x06,
            0xa3, 0x8b, 0x5f, 0x00, 0x08, 0x01, 0xa3, 0x84, 0x63, 0x00, 0x17, 0x00, 0x00, 0x06,
            0x06, 0x8c, 0x07, 0x00, 0x04, 0x00, 0x8c, 0x08, 0x00, 0x04, 0x00, 0x8c, 0x09, 0x00,
            0x04, 0x74, 0x00, 0xa3, 0x83, 0x1a, 0x00, 0x05, 0x8c, 0xaf, 0xbb, 0xb7, 0x8e, 0x88,
            0xa6, 0xee, 0x0c, 0xa3, 0x83, 0x1c, 0x00, 0x0b, 0x00, 0x01, 0xa3, 0x83, 0x1d, 0x00,
            0x04, 0x00, 0xab, 0x00, 0x24, 0x02, 0x00, 0x00, 0x80, 0x0a, 0x00, 0x00, 0x00, 0x3b,
            0xd1, 0xb9, 0x9d, 0x08, 0x8a, 0xc0, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00,
            0x00, 0xd5, 0xee, 0xb8, 0x2c, 0x05, 0x8a, 0xc0, 0x00, 0x08, 0x00, 0x04, 0x00, 0x00,
            0x00, 0xac, 0x00, 0x0c, 0x04, 0x74, 0x6f, 0x74, 0x6f, 0x00, 0x04, 0x74, 0x69, 0x74,
            0x69, 0x00, 0x00, 0x00, 0xa2, 0x00, 0x00, 0x00, 0x00, 0x72, 0x02, 0x00, 0x00,
        ];
        let resp = parse_explore_response(&raw, true).unwrap();
        assert_eq!(resp.objects.len(), 1);
        let obj = &resp.objects[0];

        let vt = obj.vartype_list.as_ref().expect("vartype list");
        assert_eq!(vt.elements.len(), 2);
        assert_eq!(vt.elements[0].lid, 10); // toto
        assert_eq!(vt.elements[0].symbol_crc, 0x9db9_d13b);
        assert_eq!(vt.elements[0].offset_info.relation_id, None);
        assert_eq!(vt.elements[1].lid, 12); // titi
        assert_eq!(vt.elements[1].symbol_crc, 0x2cb8_eed5);

        let vn = obj.varname_list.as_ref().expect("varname list");
        assert_eq!(vn.names, vec!["toto".to_string(), "titi".to_string()]);
    }
}
