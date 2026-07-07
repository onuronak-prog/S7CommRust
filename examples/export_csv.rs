// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// Browse every DB/FB and M/Q/I tag on a CPU, read each live value, and write the whole list to
// a CSV file (UTF-8 with a BOM, so spreadsheet apps render non-ASCII names correctly). A
// headless replacement for the old Slint GUI: the same connect -> browse -> read -> CSV
// pipeline, with no GUI dependency.
//
//   S7_PLC_IP=192.168.0.1 cargo run --example export_csv
//   S7_PLC_IP=192.168.0.1 cargo run --example export_csv -- tags.csv
//   S7_PLC_IP=192.168.0.1 cargo run --example export_csv -- tags.csv --legacy
//   S7_PLC_IP=192.168.0.1 cargo run --example export_csv -- tags.csv --real-plc

use std::time::Duration;

use s7commplus::value::PValue;
use s7commplus::Connection;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("export failed: {e}");
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

    // A positional argument is the output path (default `tags.csv`). Flags pick the transport:
    // `--legacy` (non-TLS PLCSIM family) or `--real-plc` (non-TLS real S7-1200/1500 hardware);
    // the default is the modern TLS path.
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(String::as_str)
        .unwrap_or("tags.csv")
        .to_string();

    let addr = (ip.as_str(), port);
    let timeout = Duration::from_secs(10);
    let (mut conn, mode) = if args.iter().any(|a| a == "--real-plc") {
        (
            Connection::connect_real_plc(addr, timeout)?,
            "legacy/real-plc",
        )
    } else if args.iter().any(|a| a == "--legacy") {
        (Connection::connect_legacy(addr, timeout)?, "legacy")
    } else {
        (Connection::connect(addr, timeout)?, "TLS")
    };
    eprintln!("connected to {ip}:{port} ({mode}) — browsing symbol tree…");

    let vars = conn.browse_vars()?;
    eprintln!("found {} tags — reading values…", vars.len());
    let values = conn.read_var_values(&vars)?;

    let mut csv = String::from("\u{feff}"); // UTF-8 BOM
    csv.push_str("Symbol,DataType,Value\r\n");
    for (var, value) in vars.iter().zip(values) {
        csv.push_str(&csv_field(&var.name));
        csv.push(',');
        csv.push_str(&csv_field(&sdt_name(var.softdatatype)));
        csv.push(',');
        csv.push_str(&csv_field(&fmt_value(&value, var.softdatatype)));
        csv.push_str("\r\n");
    }
    std::fs::write(&path, csv)?;
    eprintln!("wrote {} rows -> {path}", vars.len());
    Ok(())
}

/// Quote a CSV field per RFC 4180 when it contains a comma, quote, or newline.
fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Format an optional read value for the CSV. S7 `String`s (softdatatype 19) arrive as a USInt
/// array and are decoded to text; date/time types render via the value layer's helpers.
fn fmt_value(value: &Option<PValue>, softdatatype: u8) -> String {
    match value {
        None => "(not readable)".to_string(),
        Some(PValue::USIntArray(bytes)) if softdatatype == 19 => decode_s7_string(bytes),
        Some(v) => {
            s7commplus::value::datetime::format(softdatatype, v).unwrap_or_else(|| fmt_scalar(v))
        }
    }
}

/// Render a scalar `PValue` compactly (bit-string types also show hex).
fn fmt_scalar(v: &PValue) -> String {
    use PValue::*;
    match v {
        Bool(b) => b.to_string(),
        USInt(n) => n.to_string(),
        UInt(n) => n.to_string(),
        UDInt(n) => n.to_string(),
        ULInt(n) => n.to_string(),
        SInt(n) => n.to_string(),
        Int(n) => n.to_string(),
        DInt(n) => n.to_string(),
        LInt(n) => n.to_string(),
        Byte(n) => format!("{n} (0x{n:02x})"),
        Word(n) => format!("{n} (0x{n:04x})"),
        DWord(n) => format!("{n} (0x{n:08x})"),
        LWord(n) => format!("{n} (0x{n:016x})"),
        Real(x) => x.to_string(),
        LReal(x) => x.to_string(),
        WString(s) => s.clone(),
        USIntArray(b) => decode_s7_string(b),
        other => format!("{other:?}"),
    }
}

/// Decode an S7 `STRING` from its USInt-array form `[max_len, actual_len, chars…]` (Latin-1).
fn decode_s7_string(bytes: &[u8]) -> String {
    if bytes.len() < 2 {
        return String::new();
    }
    let actual = bytes[1] as usize;
    let end = (2 + actual).min(bytes.len());
    bytes[2..end].iter().map(|&b| b as char).collect()
}

/// Map a Siemens "softdatatype" id to a display name (unknowns become `sdtN`).
fn sdt_name(sdt: u8) -> String {
    let name = match sdt {
        1 => "Bool",
        2 => "Byte",
        3 => "Char",
        4 => "Word",
        5 => "Int",
        6 => "DWord",
        7 => "DInt",
        8 => "Real",
        9 => "Date",
        11 => "Time",
        17 => "Struct",
        19 => "String",
        48 => "LReal",
        49 => "ULInt",
        50 => "LInt",
        51 => "LWord",
        52 => "USInt",
        53 => "UInt",
        54 => "UDInt",
        55 => "SInt",
        62 => "WString",
        67 => "DTL",
        _ => return format!("sdt{sdt}"),
    };
    name.to_string()
}
