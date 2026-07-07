// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// M/Q/I-area tag read/write round-trip — tags are addressed by bare name (no DB prefix).
// Driver-only.
//
//   S7_PLC_IP=192.168.0.1 cargo run --example mq

use std::time::Duration;

use s7commplus::value::PValue;
use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("mq failed: {e}");
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

    println!("connecting to {ip}:{port} ...");
    let mut conn = Connection::connect((ip.as_str(), port), Duration::from_secs(10))?;
    println!("  session_id = 0x{:08x}\n", conn.session_id());

    // M-area tags addressed by bare name (no DB prefix).
    let cases = [
        ("flag1", PValue::Bool(true)),
        ("flag2", PValue::Int(12345)),
        ("flag3", PValue::DInt(-70000)),
    ];

    let mut all_ok = true;
    for (sym, new_val) in cases {
        let addr = conn.resolve_symbol(sym)?;
        let original = conn.read_tag(sym)?;
        conn.write_tag(sym, new_val.clone())?;
        let after = conn.read_tag(sym)?;
        let ok = after == new_val;
        all_ok &= ok;
        println!(
            "{sym}  [AccessArea=0x{:02x} sub={} LID={:?}]\n  {:?} -> wrote {:?} -> read {:?}  [{}]",
            addr.access_area,
            addr.access_sub_area,
            addr.lid,
            original,
            new_val,
            after,
            if ok { "MATCH" } else { "MISMATCH" }
        );
        conn.write_tag(sym, original)?; // restore
    }

    println!(
        "\n{}",
        if all_ok {
            "ALL M-AREA ROUND-TRIPS MATCHED"
        } else {
            "SOME MISMATCHED"
        }
    );
    Ok(())
}
