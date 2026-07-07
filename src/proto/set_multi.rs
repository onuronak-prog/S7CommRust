// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/SetMultiVariablesRequest.cs +
// Core/SetMultiVariablesResponse.cs, LGPL-3.0-or-later.

//! `SetMultiVariables` — write one or more symbolic variables in a single request.
//!
//! Request body (after the shared [`RequestHeader`], transport flags `0x34`): the in-object
//! id (`0` for address-based writes), the value count (VLQ), then — for `InObjectId == 0` —
//! the total field count (VLQ) and each [`ItemAddress`], then the values as `(index, value)`
//! pairs (1-based), a single `0x00` fill byte, the object qualifier, an optional VLQ
//! integrity id, and a fill `u32`.
//!
//! Response body (after the shared response header): an error list of `(item_no,
//! return_value)` pairs terminated by `item_no == 0`, then a trailing integrity id.

use std::io::Cursor;

use crate::error::{Error, Result};
use crate::proto::header::{RequestHeader, ResponseHeader};
use crate::proto::item_address::ItemAddress;
use crate::proto::object::encode_object_qualifier;
use crate::value::PValue;
use crate::wire::pdu::{self, functioncode, ids, protocol_version};
use crate::wire::{primitives as p, vlq};

/// Transport flags used by the reference `SetMultiVariablesRequest`.
const TRANSPORT_FLAGS: u8 = 0x34;

/// Build a framed `SetMultiVariablesRequest` writing `values` to `addresses` (paired by
/// position). Uses address-based access (`InObjectId == 0`).
pub fn build_set_multi_request(
    sequence_number: u16,
    session_id: u32,
    addresses: &[ItemAddress],
    values: &[PValue],
    with_integrity: bool,
    integrity_id: u32,
) -> Result<Vec<u8>> {
    if addresses.len() != values.len() {
        return Err(Error::protocol(format!(
            "SetMultiVariables: address count ({}) != value count ({})",
            addresses.len(),
            values.len()
        )));
    }

    let header = RequestHeader {
        function_code: functioncode::SET_MULTI_VARIABLES,
        sequence_number,
        session_id,
        transport_flags: TRANSPORT_FLAGS,
    };

    let field_count: u32 = addresses.iter().map(ItemAddress::field_count).sum();

    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, 0)?; // InObjectId (0 => address-based)
    vlq::encode_u32(&mut body, values.len() as u32)?; // value count
    vlq::encode_u32(&mut body, field_count)?; // total field count (InObjectId == 0 branch)
    for addr in addresses {
        addr.serialize(&mut body)?;
    }
    for (i, value) in values.iter().enumerate() {
        vlq::encode_u32(&mut body, i as u32 + 1)?; // 1-based item number
        value.serialize(&mut body)?;
    }
    p::encode_u8(&mut body, 0x00)?; // fill byte
    encode_object_qualifier(&mut body)?;
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    p::encode_u32(&mut body, 0)?; // fill

    Ok(pdu::frame_single_pdu(protocol_version::V2, &body))
}

/// Build a framed session-setup `SetMultiVariablesRequest` (`SetSessionSetupData`).
///
/// Writes the `ServerSessionVersion` attribute (306) back to the session object
/// (`InObjectId = session_id`), echoing the Struct the PLC returned in the CreateObject
/// response. This is step 4 of the connect sequence; the PLC rejects later requests
/// (Explore, reads) until it completes. Sent with `ProtocolVersion.V2` and **without** an
/// integrity id (matching the reference `SetSessionSetupData`).
pub fn build_session_setup_request(
    sequence_number: u16,
    session_id: u32,
    server_session_version: &PValue,
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::SET_MULTI_VARIABLES,
        sequence_number,
        session_id,
        transport_flags: TRANSPORT_FLAGS,
    };

    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, session_id)?; // InObjectId = session id (> 0 branch)
    vlq::encode_u32(&mut body, 1)?; // value count
    vlq::encode_u32(&mut body, 1)?; // address (attribute id) count
    vlq::encode_u32(&mut body, ids::SERVER_SESSION_VERSION)?; // attribute 306
    vlq::encode_u32(&mut body, 1)?; // 1-based item number
    server_session_version.serialize(&mut body)?;
    p::encode_u8(&mut body, 0x00)?; // fill byte
    encode_object_qualifier(&mut body)?;
    // WithIntegrityId == false: no integrity id here.
    p::encode_u32(&mut body, 0)?; // fill

    Ok(pdu::frame_single_pdu(protocol_version::V2, &body))
}

/// A parsed `SetMultiVariablesResponse`.
#[derive(Debug, Clone, PartialEq)]
pub struct SetMultiVariablesResponse {
    /// The response header.
    pub header: ResponseHeader,
    /// Per-item error `(item_no, return_value)` pairs (empty on full success).
    pub errors: Vec<(u32, u64)>,
    /// Trailing integrity id (0 if absent).
    pub integrity_id: u32,
}

/// Parse a `SetMultiVariablesResponse` telegram.
pub fn parse_set_multi_response(buf: &[u8]) -> Result<SetMultiVariablesResponse> {
    let pdu_header = pdu::parse_header(buf)?;
    let mut cur = Cursor::new(&buf[pdu_header.body_offset..]);

    let header = ResponseHeader::read(&mut cur)?;
    if header.function_code != functioncode::SET_MULTI_VARIABLES {
        return Err(Error::protocol(format!(
            "SetMultiVariablesResponse: expected function 0x{:04x}, got 0x{:04x}",
            functioncode::SET_MULTI_VARIABLES,
            header.function_code
        )));
    }

    let mut errors = Vec::new();
    let mut item_no = vlq::decode_u32(&mut cur)?;
    while item_no > 0 {
        let retval = vlq::decode_u64(&mut cur)?;
        errors.push((item_no, retval));
        item_no = vlq::decode_u32(&mut cur)?;
    }

    let integrity_id = vlq::decode_u32(&mut cur).unwrap_or(0);

    Ok(SetMultiVariablesResponse {
        header,
        errors,
        integrity_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::pdu::opcode;

    #[test]
    fn request_is_byte_exact() {
        let addr = ItemAddress {
            symbol_crc: 0,
            access_area: 3,
            access_sub_area: 0,
            lid: vec![],
        };
        let framed =
            build_set_multi_request(4, 0x0000_0120, &[addr], &[PValue::UInt(0x1234)], true, 1)
                .unwrap();
        let expected = vec![
            // PDU header: id, V2, len = 0x003a (58)
            0x72, 0x02, 0x00, 0x3a, //
            // request header (function 0x0542, seq 4, session 0x120, flags 0x34)
            0x31, 0x00, 0x00, 0x05, 0x42, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x01, 0x20, 0x34,
            // InObjectId
            0x00, 0x00, 0x00, 0x00, //
            0x01, // value count
            0x04, // total field count (4 + 0 LIDs)
            // ItemAddress
            0x00, 0x03, 0x01, 0x00, //
            // value list: item 1 -> UInt(0x1234) (datatype 0x03)
            0x01, 0x00, 0x03, 0x12, 0x34, //
            0x00, // fill byte
            // object qualifier
            0x00, 0x00, 0x04, 0xe8, 0x89, 0x69, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x89, 0x6a,
            0x00, 0x13, 0x00, 0x89, 0x6b, 0x00, 0x04, 0x00, 0x00, //
            0x01, // integrity id
            0x00, 0x00, 0x00, 0x00, // fill
            // PDU trailer
            0x72, 0x02, 0x00, 0x00,
        ];
        assert_eq!(framed, expected);
    }

    #[test]
    fn length_mismatch_is_rejected() {
        let addr = ItemAddress::default();
        assert!(build_set_multi_request(1, 0, &[addr], &[], false, 0).is_err());
    }

    #[test]
    fn parse_response() {
        let mut body = Vec::new();
        body.push(opcode::RESPONSE);
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&functioncode::SET_MULTI_VARIABLES.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&4u16.to_be_bytes());
        body.push(0x00);
        vlq::encode_u64(&mut body, 0).unwrap(); // return value ok
        vlq::encode_u32(&mut body, 0).unwrap(); // empty error list
        vlq::encode_u32(&mut body, 1).unwrap(); // integrity id

        let framed = pdu::frame_single_pdu(protocol_version::V1, &body);
        let resp = parse_set_multi_response(&framed).unwrap();
        assert!(resp.header.is_ok());
        assert!(resp.errors.is_empty());
        assert_eq!(resp.integrity_id, 1);
    }
}
