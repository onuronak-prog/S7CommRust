// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Write-by-symbol probe: read a tag, write a new value, read back to confirm, then restore
// the original. Validates SetMultiVariables against a live PLC.
//
//   S7_PLC_IP=192.168.0.1 cargo run --example write

use std::time::Duration;

use s7commplus::value::PValue;
use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("write failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> s7commplus::Result<()> {
    let ip = std::env::var("S7_PLC_IP").unwrap_or_else(|_| {
        eprintln!("set S7_PLC_IP (e.g. S7_PLC_IP=192.168.0.1)");
        std::process::exit(2);
    });
    let port: u16 = std::env::var("S7_PLC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(s7commplus::transport::tcp::ISO_TCP_PORT);

    println!("connecting to {ip}:{port} ...");
    let mut conn = Connection::connect((ip.as_str(), port), Duration::from_secs(10))?;
    println!("  session_id = 0x{:08x}", conn.session_id());

    // (symbol, new value to write)
    let cases = [
        ("Data_block_1.titi", PValue::Int(456)),
        ("Data_block_1.toto", PValue::Real(98.76)),
    ];

    for (sym, new_val) in cases {
        let original = conn.read_tag(sym)?;
        println!("\n{sym}: original = {original:?}");

        conn.write_tag(sym, new_val.clone())?;
        let after = conn.read_tag(sym)?;
        let ok = after == new_val;
        println!(
            "  wrote {new_val:?} -> read back {after:?}  [{}]",
            if ok { "MATCH" } else { "MISMATCH" }
        );

        // Restore the original value.
        conn.write_tag(sym, original.clone())?;
        let restored = conn.read_tag(sym)?;
        println!("  restored to {restored:?}");
    }
    Ok(())
}
