// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Core/PValue.cs + Core/Datatype.cs,
// LGPL-3.0-or-later.

//! The value/type system: datatype tags, the [`PValue`] enum, and S7 date/time decoding.

pub mod datatype;
pub mod datetime;
pub mod pvalue;

pub use datetime::{S7DateTime, S7Duration, S7TimeOfDay};
pub use pvalue::PValue;
