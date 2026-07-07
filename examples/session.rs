// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// End-to-end connect probe: TCP+COTP -> InitSsl -> TLS -> CreateObject (session).
// Run against a TLS-capable S7-1200/1500 or PLCSIM Advanced + NetToPLCSim.
//
//   S7_PLC_IP=192.168.0.1 cargo run --example session

use std::time::Duration;

use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    if let Err(e) = run() {
        eprintln!("connect failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> s7commplus::Result<()> {
    let ip = std::env::var("S7_PLC_IP").unwrap_or_else(|_| {
        eprintln!("set S7_PLC_IP to the PLC address (e.g. S7_PLC_IP=192.168.0.1)");
        std::process::exit(2);
    });
    let port: u16 = std::env::var("S7_PLC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(s7commplus::transport::tcp::ISO_TCP_PORT);

    println!("connecting to {ip}:{port} ...");
    let mut conn = Connection::connect((ip.as_str(), port), Duration::from_secs(10))?;

    println!("session established:");
    println!("  session_id  = 0x{:08x}", conn.session_id());
    println!("  session_id2 = 0x{:08x}", conn.session_id2());

    let secret = conn.export_oms_secret()?;
    let hex: String = secret.iter().map(|b| format!("{b:02x}")).collect();
    println!("  OMS secret  = {hex}");

    // Optional legitimation: set S7_PLC_PASSWORD (and optionally S7_PLC_USER).
    if let Ok(password) = std::env::var("S7_PLC_PASSWORD") {
        let user = std::env::var("S7_PLC_USER").unwrap_or_default();
        println!("legitimating as user {user:?} ...");
        conn.legitimate(&user, &password)?;
        println!("  legitimation OK");
    } else {
        println!("(set S7_PLC_PASSWORD to exercise legitimation)");
    }

    println!();
    println!("Connected. Reads/writes via read_variables/write_variables once you have an");
    println!("ItemAddress (symbol CRC + access area + LIDs); Explore (discovery) is next.");
    Ok(())
}
