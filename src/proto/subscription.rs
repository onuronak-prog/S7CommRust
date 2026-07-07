// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Subscriptions/Subscription.cs +
// Core/CreateObjectRequest.cs, LGPL-3.0-or-later.

//! Subscriptions: build the `CreateObject` request that creates a subscription object on
//! the PLC and encodes the list of variables to monitor. The PLC then pushes
//! [`Notification`](crate::proto::notification::Notification) PDUs.

use crate::error::Result;
use crate::proto::header::RequestHeader;
use crate::proto::item_address::ItemAddress;
use crate::proto::object::{encode_object_qualifier, PObject};
use crate::value::datatype::{flags, tag};
use crate::value::PValue;
use crate::wire::pdu::{self, functioncode, ids, protocol_version};
use crate::wire::{primitives as p, vlq};

// Subscription object class + attribute ids (reference `Ids.cs`).
const CLASS_SUBSCRIPTION: u32 = 1001;
const OBJECT_VARIABLE_TYPE_NAME: u32 = 233;
const SUB_MISSED_SENDINGS: u32 = 1002;
const SUB_SUBSYSTEM_ERROR: u32 = 1003;
const SUB_ROUTE_MODE: u32 = 1040;
const SUB_ACTIVE: u32 = 1041;
const SUB_REFERENCE_LIST: u32 = 1048;
const SUB_CYCLE_TIME: u32 = 1049;
const SUB_DISABLED: u32 = 1051;
const SUB_COUNT: u32 = 1052;
const SUB_CREDIT_LIMIT: u32 = 1053;
const SUB_TICKS: u32 = 1054;
const SUB_FUNCTION_CLASS_ID: u32 = 1082;
/// Reference `1055` — "unknown", works when set to 0.
const SUB_UNKNOWN_1055: u32 = 1055;

/// `CreateObjectRequest` transport flags used for a subscription (reference `0x34`).
const CREATE_OBJECT_TRANSPORT_FLAGS: u8 = 0x34;
/// Reference start value for the subscription relation id (`m_SubscriptionRelationId`).
const DEFAULT_RELATION_ID: u32 = 0x7fff_c001;

// Alarm-subscription ids (reference `AlarmsHandler`/`Ids.cs`).
const SUB_DELAY_TIME: u32 = 1050;
const SUB_REF_TRIGGER_TRANSMIT_MODE: u32 = 1005;
const ALARM_SUB_REF_CLASS: u32 = 2662;
const ALARM_SUB_REF_ALARM_DOMAIN: u32 = 2659;
const ALARM_SUB_REF_ITS_SUBSYSTEM: u32 = 2660;
const ALARM_SUB_REF_ALARM_DOMAIN2: u32 = 7731;
const ALARM_SUB_REF_TEXT_LANGUAGES: u32 = 8181;
const ALARM_SUB_REF_SEND_TEXTS: u32 = 8173;
const ALARM_SUB_REF_RELATION_ID: u32 = 0x5101_0001;
/// Alarm subsystem object id (reference `AddRelation(itsAlarmSubsystem, 8)`).
const ALARM_SUBSYSTEM_ID: u32 = 0x0000_0008;

/// Default route mode. `0x14` with an unlimited credit limit delivers all values on create,
/// then changed values plus an (empty) notification each cycle — a steady flow that needs no
/// credit refresh.
pub const DEFAULT_ROUTE_MODE: u8 = 0x14;
/// Default credit limit. `-1` = unlimited (no periodic credit top-up required).
pub const DEFAULT_CREDIT_LIMIT: i16 = -1;

/// One variable to subscribe: an [`ItemAddress`] plus the client-chosen reference id that the
/// PLC echoes back with each value in a notification (so you know which value is which).
#[derive(Debug, Clone)]
pub struct SubscriptionItem {
    /// Client-chosen reference id the PLC echoes with each value in a notification.
    pub reference_id: u32,
    /// The address of the variable to subscribe.
    pub address: ItemAddress,
}

/// Build the `SubscriptionReferenceList` value: a UDInt **address-array** (`GetSubscriptionListArray`).
fn reference_list(change_counter: u8, items: &[SubscriptionItem]) -> PValue {
    let mut la: Vec<PValue> = Vec::new();
    // 0x8?ssxxxx: 8 = CreateNew flag, ss = subscription change counter, xxxx = unknown.
    la.push(PValue::UDInt(
        0x8000_0000 | (u32::from(change_counter) << 16),
    ));
    la.push(PValue::UDInt(0)); // number of items to unsubscribe
    la.push(PValue::UDInt(items.len() as u32)); // number to subscribe
    for item in items {
        // 0x8004_0000 | (1 + LID count): field count of the 2nd part (sub-area + each LID).
        let head = 0x8004_0000 | (1 + item.address.lid.len() as u32);
        la.push(PValue::UDInt(head));
        la.push(PValue::UDInt(item.reference_id));
        la.push(PValue::UDInt(0)); // unknown 1
        la.push(PValue::UDInt(item.address.access_area));
        la.push(PValue::UDInt(item.address.symbol_crc));
        la.push(PValue::UDInt(item.address.access_sub_area));
        for li in &item.address.lid {
            la.push(PValue::UDInt(*li));
        }
    }
    PValue::Array {
        element_type: tag::UDINT,
        flags: flags::ADDRESS_ARRAY, // 0x20
        items: la,
    }
}

/// Build the subscription object (class 1001), mirroring `SubscriptionCreate`'s attribute set.
fn subscription_object(
    relation_id: u32,
    change_counter: u8,
    route_mode: u8,
    cycle_time_ms: u16,
    credit_limit: i16,
    items: &[SubscriptionItem],
) -> PObject {
    let mut obj = PObject::new(relation_id, CLASS_SUBSCRIPTION, ids::NONE);
    obj.add_attribute(
        OBJECT_VARIABLE_TYPE_NAME,
        PValue::WString(format!("Subscription_{relation_id}")),
    );
    obj.add_attribute(SUB_FUNCTION_CLASS_ID, PValue::USInt(0));
    obj.add_attribute(SUB_MISSED_SENDINGS, PValue::UInt(0));
    obj.add_attribute(SUB_SUBSYSTEM_ERROR, PValue::LInt(0));
    obj.add_attribute(SUB_ROUTE_MODE, PValue::USInt(route_mode));
    obj.add_attribute(SUB_ACTIVE, PValue::Bool(true));
    obj.add_attribute(SUB_REFERENCE_LIST, reference_list(change_counter, items));
    obj.add_attribute(SUB_CYCLE_TIME, PValue::UDInt(u32::from(cycle_time_ms)));
    obj.add_attribute(SUB_DISABLED, PValue::USInt(0));
    obj.add_attribute(SUB_COUNT, PValue::USInt(0));
    obj.add_attribute(SUB_CREDIT_LIMIT, PValue::Int(credit_limit));
    obj.add_attribute(SUB_TICKS, PValue::UInt(65535));
    obj.add_attribute(SUB_UNKNOWN_1055, PValue::USInt(0));
    obj
}

/// Build a framed subscription `CreateObject` request (`ProtocolVersion.V2`, transport flags
/// `0x34`, `RequestId` = `session_id2`). Mirrors `CreateObjectRequest.Serialize`.
#[allow(clippy::too_many_arguments)]
pub fn build_subscription_create_request(
    sequence_number: u16,
    session_id: u32,
    session_id2: u32,
    with_integrity: bool,
    integrity_id: u32,
    change_counter: u8,
    route_mode: u8,
    cycle_time_ms: u16,
    credit_limit: i16,
    items: &[SubscriptionItem],
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::CREATE_OBJECT,
        sequence_number,
        session_id,
        transport_flags: CREATE_OBJECT_TRANSPORT_FLAGS,
    };
    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, session_id2)?; // RequestId
    PValue::UDInt(0).serialize(&mut body)?; // RequestValue
    p::encode_u32(&mut body, 0)?; // unknown / fill
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    subscription_object(
        DEFAULT_RELATION_ID,
        change_counter,
        route_mode,
        cycle_time_ms,
        credit_limit,
        items,
    )
    .serialize(&mut body)?;
    p::encode_u32(&mut body, 0)?; // final fill
    Ok(pdu::frame_single_pdu(protocol_version::V2, &body))
}

/// Build a framed alarm-subscription `CreateObject` request, mirroring `AlarmSubscriptionCreate`.
/// Unlike a variable subscription this carries an empty reference list and an `AlarmSubscriptionRef`
/// child object that selects the alarm domain(s), languages, and text delivery. `credit_limit`
/// `-1` = unlimited.
pub fn build_alarm_subscription_create_request(
    sequence_number: u16,
    session_id: u32,
    session_id2: u32,
    with_integrity: bool,
    integrity_id: u32,
    credit_limit: i16,
) -> Result<Vec<u8>> {
    // The subscription object (class 1001), function class 2 = alarming, route mode 2.
    let mut subs = PObject::new(DEFAULT_RELATION_ID, CLASS_SUBSCRIPTION, ids::NONE);
    subs.add_attribute(
        OBJECT_VARIABLE_TYPE_NAME,
        PValue::WString(format!("Subscription_{DEFAULT_RELATION_ID}")),
    );
    subs.add_attribute(SUB_FUNCTION_CLASS_ID, PValue::USInt(2));
    subs.add_attribute(SUB_MISSED_SENDINGS, PValue::UInt(0));
    subs.add_attribute(SUB_SUBSYSTEM_ERROR, PValue::LInt(0));
    subs.add_attribute(SUB_ROUTE_MODE, PValue::USInt(2));
    subs.add_attribute(SUB_ACTIVE, PValue::Bool(true));
    // Minimal reference list: header (change counter 1) + 0 unsubscribe + 0 subscribe.
    subs.add_attribute(
        SUB_REFERENCE_LIST,
        PValue::Array {
            element_type: tag::UDINT,
            flags: flags::ADDRESS_ARRAY,
            items: vec![
                PValue::UDInt(0x8001_0000),
                PValue::UDInt(0),
                PValue::UDInt(0),
            ],
        },
    );
    subs.add_attribute(SUB_CYCLE_TIME, PValue::UDInt(0));
    subs.add_attribute(SUB_DELAY_TIME, PValue::UDInt(0));
    subs.add_attribute(SUB_DISABLED, PValue::USInt(0));
    subs.add_attribute(SUB_COUNT, PValue::USInt(0));
    subs.add_attribute(SUB_CREDIT_LIMIT, PValue::Int(credit_limit));
    subs.add_attribute(SUB_TICKS, PValue::UInt(65535));

    // The AlarmSubscriptionRef child object (class 2662): what alarms to receive.
    let mut aref = PObject::new(ALARM_SUB_REF_RELATION_ID, ALARM_SUB_REF_CLASS, ids::NONE);
    aref.add_attribute(
        OBJECT_VARIABLE_TYPE_NAME,
        PValue::WString("S7pDriver_Alarming".to_string()),
    );
    aref.add_attribute(SUB_REF_TRIGGER_TRANSMIT_MODE, PValue::USInt(3));
    // AlarmDomain: regular array of 10 zero u16s (no explicit domain filter).
    aref.add_attribute(
        ALARM_SUB_REF_ALARM_DOMAIN,
        PValue::Array {
            element_type: tag::UINT,
            flags: flags::ARRAY,
            items: vec![PValue::UInt(0); 10],
        },
    );
    // AlarmDomain2: address array [65535] = "all domains".
    aref.add_attribute(
        ALARM_SUB_REF_ALARM_DOMAIN2,
        PValue::Array {
            element_type: tag::UINT,
            flags: flags::ADDRESS_ARRAY,
            items: vec![PValue::UInt(65535)],
        },
    );
    // Text languages: empty address array = all languages.
    aref.add_attribute(
        ALARM_SUB_REF_TEXT_LANGUAGES,
        PValue::Array {
            element_type: tag::UDINT,
            flags: flags::ADDRESS_ARRAY,
            items: Vec::new(),
        },
    );
    aref.add_attribute(ALARM_SUB_REF_SEND_TEXTS, PValue::Bool(true));
    aref.add_relation(ALARM_SUB_REF_ITS_SUBSYSTEM, ALARM_SUBSYSTEM_ID);
    subs.add_object(aref);

    let header = RequestHeader {
        function_code: functioncode::CREATE_OBJECT,
        sequence_number,
        session_id,
        transport_flags: CREATE_OBJECT_TRANSPORT_FLAGS,
    };
    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, session_id2)?; // RequestId
    PValue::UDInt(0).serialize(&mut body)?; // RequestValue
    p::encode_u32(&mut body, 0)?; // unknown / fill
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    subs.serialize(&mut body)?;
    p::encode_u32(&mut body, 0)?; // final fill
    Ok(pdu::frame_single_pdu(protocol_version::V2, &body))
}

/// Build a `SetVariable` request that raises the subscription's credit limit (a *no-response*
/// request, transport flags `0x74`), mirroring `SubscriptionSetCreditLimit`. Used to keep a
/// finite-credit subscription flowing. The caller must NOT wait for a reply.
pub fn build_credit_limit_request(
    sequence_number: u16,
    session_id: u32,
    subscription_object_id: u32,
    with_integrity: bool,
    integrity_id: u32,
    limit: i16,
) -> Result<Vec<u8>> {
    let header = RequestHeader {
        function_code: functioncode::SET_VARIABLE,
        sequence_number,
        session_id,
        transport_flags: 0x74, // "no response needed"
    };
    let mut body = Vec::new();
    header.serialize(&mut body)?;
    p::encode_u32(&mut body, subscription_object_id)?; // InObjectId
    vlq::encode_u32(&mut body, 1)?; // always 1
    vlq::encode_u32(&mut body, SUB_CREDIT_LIMIT)?; // address = 1053
    PValue::Int(limit).serialize(&mut body)?;
    encode_object_qualifier(&mut body)?;
    p::encode_u8(&mut body, 0x00)?;
    if with_integrity {
        vlq::encode_u32(&mut body, integrity_id)?;
    }
    p::encode_u32(&mut body, 0)?; // fill
    Ok(pdu::frame_single_pdu(protocol_version::V2, &body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::pdu::opcode;

    #[test]
    fn reference_list_layout() {
        // One item: area=0x50, sub-area=0x9ba, one LID [7]. Change counter 1.
        let items = [SubscriptionItem {
            reference_id: 1,
            address: ItemAddress {
                symbol_crc: 0,
                access_area: 0x50,
                access_sub_area: 0x9ba,
                lid: vec![7],
            },
        }];
        let PValue::Array {
            element_type,
            flags,
            items: la,
        } = reference_list(1, &items)
        else {
            panic!("expected array");
        };
        assert_eq!(element_type, tag::UDINT);
        assert_eq!(flags, 0x20); // address array
                                 // header(0x80010000) + unsub(0) + sub(1) + [head(0x80040002), ref(1), 0, area, crc, sub, lid]
        assert_eq!(la[0], PValue::UDInt(0x8001_0000));
        assert_eq!(la[1], PValue::UDInt(0));
        assert_eq!(la[2], PValue::UDInt(1));
        assert_eq!(la[3], PValue::UDInt(0x8004_0002)); // 1 + lid.len()==2
        assert_eq!(la[4], PValue::UDInt(1)); // reference id
        assert_eq!(la[6], PValue::UDInt(0x50)); // access area
        assert_eq!(la[8], PValue::UDInt(0x9ba)); // access sub area
        assert_eq!(la[9], PValue::UDInt(7)); // lid
    }

    #[test]
    fn create_request_header_is_v2() {
        let items = [SubscriptionItem {
            reference_id: 1,
            address: ItemAddress::default(),
        }];
        let framed =
            build_subscription_create_request(5, 0x120, 0x121, false, 0, 1, 0x14, 100, -1, &items)
                .unwrap();
        // 72 02 <len..> then opcode REQUEST, then reserved/function CreateObject.
        assert_eq!(framed[0], pdu::PROTOCOL_ID);
        assert_eq!(framed[1], protocol_version::V2);
        assert_eq!(framed[4], opcode::REQUEST);
        assert_eq!(&framed[7..9], &functioncode::CREATE_OBJECT.to_be_bytes());
        // transport flags 0x34 sits after reserved(2)+func(2)+reserved(2)+seq(2)+session(4).
        assert_eq!(
            framed[4 + 1 + 2 + 2 + 2 + 2 + 4],
            CREATE_OBJECT_TRANSPORT_FLAGS
        );
    }
}
