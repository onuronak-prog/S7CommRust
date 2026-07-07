// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
// Ported from thomas-v2/S7CommPlusDriver Net/* + OpenSSL/*, LGPL-3.0-or-later.

//! Transport layer: blocking TCP + TPKT/COTP framing and the rustls TLS pump.

pub mod tcp;
// The TLS channel and its constants are driven internally by `Connection`; not part of the API.
pub(crate) mod tls;

// The socket/TLS channel types are driven internally by `Connection`; not part of the API.
pub(crate) use tcp::IsoTcp;
pub(crate) use tls::TlsChannel;
