// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Read/write round-trip validation across types (bool, int, dint, real, string), array
// elements, and nested-struct members — using only the Rust driver. For each: read the
// original, write a new value, read back to confirm, then restore the original.
//
//   S7_PLC_IP=192.168.0.1 cargo run --example rw

use std::time::Duration;

use s7commplus::value::PValue;
use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("rw failed: {e}");
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

    // (symbol, new value) — covers bool, int, dint, real, array element, nested-struct member.
    let cases = [
        ("Data_block_1.totoB", PValue::Bool(false)),
        ("Data_block_1.titi", PValue::Int(-999)),
        ("Data_block_1.toto", PValue::Real(76.5)),
        ("Data_block_1.totoStruct.titi", PValue::DInt(1_000_000)),
        ("Data_block_1.totoStruct.toto", PValue::Real(-2.5)),
        ("Data_block_1.totoAr[2]", PValue::Int(555)),
    ];

    let mut all_ok = true;
    for (sym, new_val) in cases {
        let original = conn.read_tag(sym)?;
        conn.write_tag(sym, new_val.clone())?;
        let after = conn.read_tag(sym)?;
        let ok = after == new_val;
        all_ok &= ok;
        println!(
            "{sym}\n  {:?} -> wrote {:?} -> read {:?}  [{}]",
            original,
            new_val,
            after,
            if ok { "MATCH" } else { "MISMATCH" }
        );
        conn.write_tag(sym, original)?; // restore
    }

    // String round-trip.
    let sym = "Data_block_1.totoS";
    let original = conn.read_string(sym)?;
    let new_str = "written by the rust driver";
    conn.write_string(sym, new_str)?;
    let after = conn.read_string(sym)?;
    let ok = after == new_str;
    all_ok &= ok;
    println!(
        "{sym}\n  {original:?} -> wrote {new_str:?} -> read {after:?}  [{}]",
        if ok { "MATCH" } else { "MISMATCH" }
    );
    conn.write_string(sym, &original)?; // restore

    println!(
        "\n{}",
        if all_ok {
            "ALL ROUND-TRIPS MATCHED"
        } else {
            "SOME MISMATCHED"
        }
    );
    Ok(())
}
