// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/DeleteObjectRequest.cs +
// Core/DeleteObjectResponse.cs, LGPL-3.0-or-later.

//! `DeleteObject` — delete a server object by id (used to tear down a subscription/session).

use std::io::Cursor;

use crate::error::{Error, Result};
use crate::proto::header::{RequestHeader, ResponseHeader};
use crate::proto::object::encode_object_qualifier;
use crate::wire::pdu::{self, functioncode, protocol_version};
use crate::wire::{primitives as p, vlq};

const TRANSPORT_FLAGS: u8 = 0x34;

/// Build a framed `DeleteObjectRequest` deleting the object `delete_object_id`.
pub fn build_delete_object_request(
    sequence_number: u16,
    session_id: u32,
    delete_object_id: u32,
    with_integrity: bool,
    integrity_id: u32,
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::DELETE_OBJECT,
        sequence_number,
        session_id,
        transport_flags: TRANSPORT_FLAGS,
    };
    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, delete_object_id)?;
    p::encode_u8(&mut body, 0x00)?;
    encode_object_qualifier(&mut body)?;
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    p::encode_u32(&mut body, 0)?; // fill
    Ok(pdu::frame_single_pdu(protocol_version::V2, &body))
}

/// Parse a `DeleteObjectResponse`, returning its header (check [`ResponseHeader::is_ok`]).
pub fn parse_delete_object_response(buf: &[u8]) -> Result<ResponseHeader> {
    let pdu_header = pdu::parse_header(buf)?;
    let mut cur = Cursor::new(&buf[pdu_header.body_offset..]);
    let header = ResponseHeader::read(&mut cur)?;
    if header.function_code != functioncode::DELETE_OBJECT {
        return Err(Error::protocol(format!(
            "DeleteObjectResponse: expected function 0x{:04x}, got 0x{:04x}",
            functioncode::DELETE_OBJECT,
            header.function_code
        )));
    }
    Ok(header)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::pdu::opcode;

    #[test]
    fn request_layout() {
        let framed = build_delete_object_request(7, 0x0000_0120, 0x7040_0000, true, 3).unwrap();
        assert_eq!(&framed[0..2], &[0x72, protocol_version::V2]);
        assert_eq!(framed[4], opcode::REQUEST);
        assert_eq!(&framed[7..9], &functioncode::DELETE_OBJECT.to_be_bytes());
        assert_eq!(
            &framed[framed.len() - 4..],
            &[0x72, protocol_version::V2, 0x00, 0x00]
        );
    }

    #[test]
    fn parse_response_ok() {
        let mut body = Vec::new();
        body.push(opcode::RESPONSE);
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&functioncode::DELETE_OBJECT.to_be_bytes());
        body.extend_from_slice(&0u16.to_be_bytes());
        body.extend_from_slice(&7u16.to_be_bytes()); // sequence
        body.push(0x34); // transport flags
        vlq::encode_u64(&mut body, 0).unwrap(); // return value ok
        p::encode_u32(&mut body, 0x7040_0000).unwrap(); // deleted object id

        let framed = pdu::frame_single_pdu(protocol_version::V2, &body);
        let h = parse_delete_object_response(&framed).unwrap();
        assert!(h.is_ok());
        assert_eq!(h.sequence_number, 7);
    }
}
