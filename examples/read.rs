// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Read-by-symbol probe: connect, then read PLC tags by name. The driver browses the data
// blocks, fetches type info, and resolves each symbol to its access LID(s) automatically.
//
//   S7_PLC_IP=192.168.0.1 cargo run --example read
//   S7_PLC_IP=192.168.0.1 cargo run --example read -- Data_block_1.toto Data_block_1.titi

use std::time::Duration;

use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("read failed: {e}");
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

    let mut symbols: Vec<String> = std::env::args().skip(1).collect();
    if symbols.is_empty() {
        symbols = vec![
            "Data_block_1.toto".to_string(),
            "Data_block_1.titi".to_string(),
        ];
    }

    println!("connecting to {ip}:{port} ...");
    let mut conn = Connection::connect((ip.as_str(), port), Duration::from_secs(10))?;
    println!("  session_id = 0x{:08x}", conn.session_id());

    let dbs = conn.datablock_list()?;
    println!("data blocks discovered:");
    for db in &dbs {
        println!(
            "  {} (DB{}, relid 0x{:08x}, ti_relid 0x{:08x})",
            db.name, db.number, db.relid, db.ti_relid
        );
    }

    println!("\nreading by symbol name:");
    for sym in &symbols {
        match conn.read_tag(sym) {
            Ok(value) => {
                let addr = conn.resolve_symbol(sym)?;
                println!(
                    "  {sym} = {value:?}   [AccessArea=0x{:08x} LID={:?}]",
                    addr.access_area, addr.lid
                );
            }
            Err(e) => println!("  {sym} -> ERROR: {e}"),
        }
    }
    Ok(())
}
