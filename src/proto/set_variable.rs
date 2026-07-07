// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/SetVariableRequest.cs +
// Core/SetVariableResponse.cs, LGPL-3.0-or-later.

//! `SetVariable` — write a single object attribute by address. Used by legitimation to
//! submit the encrypted challenge response.

use std::io::Cursor;

use crate::error::{Error, Result};
use crate::proto::header::{RequestHeader, ResponseHeader};
use crate::proto::object::encode_object_qualifier;
use crate::value::PValue;
use crate::wire::pdu::{self, functioncode};
use crate::wire::{primitives as p, vlq};

const TRANSPORT_FLAGS: u8 = 0x34;

/// Build a framed `SetVariableRequest` writing `value` to the attribute `address`.
#[allow(clippy::too_many_arguments)]
pub fn build_set_variable_request(
    protocol_version: u8,
    sequence_number: u16,
    session_id: u32,
    in_object_id: u32,
    address: u32,
    value: &PValue,
    with_integrity: bool,
    integrity_id: u32,
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::SET_VARIABLE,
        sequence_number,
        session_id,
        transport_flags: TRANSPORT_FLAGS,
    };

    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, in_object_id)?; // InObjectId (the object we write to)
    vlq::encode_u32(&mut body, 1)?; // always 1
    vlq::encode_u32(&mut body, address)?;
    value.serialize(&mut body)?;
    encode_object_qualifier(&mut body)?;
    p::encode_u8(&mut body, 0x00)?; // unknown byte
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    p::encode_u32(&mut body, 0)?; // fill

    Ok(pdu::frame_single_pdu(protocol_version, &body))
}

/// A parsed `SetVariableResponse`.
#[derive(Debug, Clone, PartialEq)]
pub struct SetVariableResponse {
    /// The response header.
    pub header: ResponseHeader,
    /// Integrity id for the next request in the session.
    pub integrity_id: u32,
}

/// Parse a `SetVariableResponse` telegram.
pub fn parse_set_variable_response(buf: &[u8]) -> Result<SetVariableResponse> {
    let pdu_header = pdu::parse_header(buf)?;
    let mut cur = Cursor::new(&buf[pdu_header.body_offset..]);

    let header = ResponseHeader::read(&mut cur)?;
    if header.function_code != functioncode::SET_VARIABLE {
        return Err(Error::protocol(format!(
            "SetVariableResponse: expected function 0x{:04x}, got 0x{:04x}",
            functioncode::SET_VARIABLE,
            header.function_code
        )));
    }
    let integrity_id = vlq::decode_u32(&mut cur).unwrap_or(0);
    Ok(SetVariableResponse {
        header,
        integrity_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::pdu::{opcode, protocol_version};

    #[test]
    fn request_round_shape() {
        let framed = build_set_variable_request(
            protocol_version::V2,
            6,
            0x0000_0120,
            0x0000_0120,
            1846,
            &PValue::Blob {
                root_id: 0,
                data: vec![0x11, 0x22],
            },
            true,
            1,
        )
        .unwrap();
        // Sanity: framed PDU starts with V2 header and ends with V2 trailer.
        assert_eq!(&framed[0..2], &[0x72, 0x02]);
        assert_eq!(&framed[framed.len() - 4..], &[0x72, 0x02, 0x00, 0x00]);
    }

    #[test]
    fn parse_response() {
        let mut body = Vec::new();
        body.push(opcode::RESPONSE);
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&functioncode::SET_VARIABLE.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&6u16.to_be_bytes());
        body.push(0x34);
        vlq::encode_u64(&mut body, 0).unwrap();
        vlq::encode_u32(&mut body, 2).unwrap(); // integrity id

        let framed = pdu::frame_single_pdu(protocol_version::V2, &body);
        let resp = parse_set_variable_response(&framed).unwrap();
        assert!(resp.header.is_ok());
        assert_eq!(resp.integrity_id, 2);
    }
}
