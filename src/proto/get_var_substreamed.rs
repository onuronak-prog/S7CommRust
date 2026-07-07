// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/GetVarSubstreamedRequest.cs +
// Core/GetVarSubstreamedResponse.cs, LGPL-3.0-or-later.

//! `GetVarSubstreamed` — read a single object attribute by address. Used by legitimation
//! to fetch the server-session challenge.

use std::io::Cursor;

use crate::error::{Error, Result};
use crate::proto::header::{RequestHeader, ResponseHeader};
use crate::proto::object::encode_object_qualifier;
use crate::value::{datatype::tag, PValue};
use crate::wire::pdu::{self, functioncode};
use crate::wire::{primitives as p, vlq};

const TRANSPORT_FLAGS: u8 = 0x34;

/// Build a framed `GetVarSubstreamedRequest` for the attribute `address`.
#[allow(clippy::too_many_arguments)]
pub fn build_get_var_substreamed_request(
    protocol_version: u8,
    sequence_number: u16,
    session_id: u32,
    in_object_id: u32,
    address: u32,
    with_integrity: bool,
    integrity_id: u32,
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::GET_VAR_SUBSTREAMED,
        sequence_number,
        session_id,
        transport_flags: TRANSPORT_FLAGS,
    };

    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, in_object_id)?; // InObjectId (the object whose attribute we read)
    p::encode_u8(&mut body, 0x20)?; // address descriptor flag
    p::encode_u8(&mut body, tag::UDINT)?; // address datatype
    p::encode_u8(&mut body, 1)?; // address count
    vlq::encode_u32(&mut body, address)?;
    encode_object_qualifier(&mut body)?;
    p::encode_u16(&mut body, 0x0001)?;
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    p::encode_u32(&mut body, 0)?; // fill

    Ok(pdu::frame_single_pdu(protocol_version, &body))
}

/// A parsed `GetVarSubstreamedResponse`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetVarSubstreamedResponse {
    /// The response header.
    pub header: ResponseHeader,
    /// The reassembled value.
    pub value: PValue,
    /// Integrity id for the next request in the session.
    pub integrity_id: u32,
}

/// Parse a `GetVarSubstreamedResponse` telegram.
pub fn parse_get_var_substreamed_response(buf: &[u8]) -> Result<GetVarSubstreamedResponse> {
    let pdu_header = pdu::parse_header(buf)?;
    let mut cur = Cursor::new(&buf[pdu_header.body_offset..]);

    let header = ResponseHeader::read(&mut cur)?;
    if header.function_code != functioncode::GET_VAR_SUBSTREAMED {
        return Err(Error::protocol(format!(
            "GetVarSubstreamedResponse: expected function 0x{:04x}, got 0x{:04x}",
            functioncode::GET_VAR_SUBSTREAMED,
            header.function_code
        )));
    }
    let _unknown = p::decode_u8(&mut cur)?;
    let value = PValue::deserialize(&mut cur)?;
    let integrity_id = vlq::decode_u32(&mut cur).unwrap_or(0);

    Ok(GetVarSubstreamedResponse {
        header,
        value,
        integrity_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::pdu::{opcode, protocol_version};

    #[test]
    fn roundtrip_challenge_response() {
        // Build a synthetic response carrying a USInt-array challenge.
        let mut body = Vec::new();
        body.push(opcode::RESPONSE);
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&functioncode::GET_VAR_SUBSTREAMED.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&5u16.to_be_bytes()); // sequence
        body.push(0x34); // transport flags
        vlq::encode_u64(&mut body, 0).unwrap(); // return value ok
        body.push(0x00); // unknown byte
        PValue::USIntArray(vec![0xaa; 16])
            .serialize(&mut body)
            .unwrap();
        vlq::encode_u32(&mut body, 1).unwrap(); // integrity id

        let framed = pdu::frame_single_pdu(protocol_version::V2, &body);
        let resp = parse_get_var_substreamed_response(&framed).unwrap();
        assert!(resp.header.is_ok());
        assert_eq!(resp.value, PValue::USIntArray(vec![0xaa; 16]));
        assert_eq!(resp.integrity_id, 1);
    }
}
