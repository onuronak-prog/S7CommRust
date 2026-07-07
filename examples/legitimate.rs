// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Legitimation (auth) probe. Reads the effective protection level, legitimates with a
// password (and optional username), then proves access with a write round-trip.
//
//   S7_PLC_IP=192.168.0.1 S7_PLC_PASSWORD=secret cargo run --example legitimate
//   ... S7_PLC_USER=admin S7_PLC_PASSWORD=secret ...   (user-management login)

use std::time::Duration;

use s7commplus::value::PValue;
use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("legitimate failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> s7commplus::Result<()> {
    let ip = std::env::var("S7_PLC_IP").unwrap_or_else(|_| {
        eprintln!("set S7_PLC_IP");
        std::process::exit(2);
    });
    let port: u16 = std::env::var("S7_PLC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(s7commplus::transport::tcp::ISO_TCP_PORT);
    // Variable used to prove access (override with S7_WRITE_TAG).
    let probe = std::env::var("S7_WRITE_TAG").unwrap_or_else(|_| "Data_block_1.titi".to_string());

    println!("connecting to {ip}:{port} ...");
    let mut conn = Connection::connect((ip.as_str(), port), Duration::from_secs(10))?;
    println!("  session_id = 0x{:08x}", conn.session_id());

    let before = conn.effective_protection_level()?;
    println!("effective protection level (before) = {before}  (1 = full access)");

    match std::env::var("S7_PLC_PASSWORD") {
        Ok(_) if before <= 1 => {
            println!(
                "PLC grants full access without auth (level {before}) — legitimation is not\n\
                 required and the PLC rejects it. Lower the *default* access level in TIA\n\
                 (Protection & Security) and keep the full-access password to require it."
            );
        }
        Ok(password) => {
            let user = std::env::var("S7_PLC_USER").unwrap_or_default();
            println!("legitimating (user={user:?}) ...");
            conn.legitimate(&user, &password)?;
            println!("  legitimation ACCEPTED by PLC");
            let after = conn.effective_protection_level()?;
            println!("effective protection level (after)  = {after}");
        }
        Err(_) => println!("(set S7_PLC_PASSWORD to legitimate; S7_PLC_USER optional)"),
    }

    // Prove access with a write round-trip.
    println!("\naccess test: write+read-back {probe}");
    let original = conn.read_tag(&probe)?;
    let new_val = match &original {
        PValue::Int(_) => PValue::Int(4242),
        PValue::DInt(_) => PValue::DInt(424242),
        PValue::Real(_) => PValue::Real(42.42),
        PValue::Bool(b) => PValue::Bool(!b),
        other => other.clone(),
    };
    match conn.write_tag(&probe, new_val.clone()) {
        Ok(()) => {
            let after = conn.read_tag(&probe)?;
            println!(
                "  {original:?} -> wrote {new_val:?} -> read {after:?}  [{}]",
                if after == new_val {
                    "MATCH"
                } else {
                    "MISMATCH"
                }
            );
            conn.write_tag(&probe, original)?; // restore
        }
        Err(e) => println!("  write denied: {e}"),
    }
    Ok(())
}
