// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/S7p.cs, LGPL-3.0-or-later.

//! Wire codec: the byte-level S7CommPlus encoding primitives.
//!
//! Built and unit-tested against golden byte vectors before any higher-layer logic
//! depends on it.

pub mod pdu;
pub mod primitives;
pub mod vlq;
