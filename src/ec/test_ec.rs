use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use anyhow::{anyhow, bail, Result};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use data_encoding::BASE64;
use pdu::{Ethernet, EthernetPdu};

///
/// # Errors
///
/// what
///
pub fn test_one() -> Result<()> {
    let Ok(rdr) = reader("inputs/input_1.txt") else {
        return Err(anyhow!("Rdr failed"));
    };

    'b: for line in rdr.lines().flatten() {
        let chars = line.chars().collect::<Vec<char>>();
        let len = chars.len();
        for i in 0..len {
            if chars[i] != chars[len - 1 - i] {
                println!("line: {chars:?}, False\n");
                continue 'b;
            }
        }
        println!("line: {chars:?}, True\n");
    }
    Ok(())
}

///
/// # Errors
///
/// f clippy
///
pub fn test_two(_pick: &str) -> Result<()> {
    let Ok(rdr) = reader("inputs/http.log.sample_2") else {
        return Err(anyhow!("Rdr failed"));
    };

    let Ok(mut wrt)= writer("outputs/test_two.log") else {
        return Err(anyhow!("create fail"));
    };

    let mut ips = HashMap::new();

    for line in rdr.lines().flatten() {
        if line.starts_with('#') {
            continue;
        }
        let mut new_line = Vec::new();
        let mut line_vec = line.split('\t').collect::<Vec<&str>>();

        let timestamp = line_vec[0];
        let datetime = if let Some(i) = timestamp.find('.') {
            let secs = timestamp[..i].parse::<i64>()?;
            let micros = timestamp[i + 1..].parse::<u32>()?;
            let Some(time) = NaiveDateTime::from_timestamp_opt(secs, micros * 1000) else {
                    return Err(anyhow!("failed to create NaiveDateTime from timestamp"));
                };
            DateTime::<Utc>::from_utc(time, Utc)
        } else {
            bail!("invalid timestamp: {}", timestamp);
        };
        let dt = format!("{}", datetime.format("%F %T"));
        line_vec[0] = &dt;

        for field in line_vec {
            if field.contains(',') {
                let new_field = field.replace(',', " ");
                new_line.push(new_field);
            } else {
                new_line.push(field.to_string());
            }
        }
        // 1.0.3
        // if new_line.get(2) == Some(&pick) {
        //     writeln!(wrt, "{}", new_line.join(","))?;
        // }

        // 1.0.4

        let ip = new_line.get(2).expect("ip field").clone();

        if let std::collections::hash_map::Entry::Vacant(e) = ips.entry(ip.clone()) {
            e.insert(1u32);
        } else if let Some(value) = ips.get_mut(&ip) {
            *value += 1;
        }

        writeln!(wrt, "{}", new_line.join(","))?;
    }
    // 1.0.4
    writeln!(wrt, "{ips:?}")?;

    // // 1.0.2
    // let Ok(reader) = reader("outputs/test_two.log") else {
    //     bail!("second rdr failed");
    // };

    // let Ok(mut wrt) = writer("outputs/test_two_two.log") else {
    //     bail!("second wrt failed");
    // };

    // for line in reader.lines() {
    //     if let Ok(line) = line {
    //         writeln!(wrt, "{}", line.replace(",", "\t"))?;
    //     }
    // }

    Ok(())
}

pub fn test_3() {
    let zerosec = Utc
        .with_ymd_and_hms(2023, 1, 20, 0, 0, 0)
        .unwrap()
        .timestamp_nanos();
    let onesec = Utc
        .with_ymd_and_hms(2023, 1, 20, 0, 0, 1)
        .unwrap()
        .timestamp_nanos();
    let twosec = Utc
        .with_ymd_and_hms(2023, 1, 27, 10, 30, 0)
        .unwrap()
        .timestamp_nanos();
    println!("{zerosec}, {onesec}, {twosec}");

    println!("{:?}", Utc.timestamp_nanos(zerosec));

    let a: u32 = 1;
    let b: u32 = 50000;
    println!("{a:0>5}");
    println!("{:01$}", a, 5);
    println!("{a:0width$}", width = 5);

    println!("{b:0>5}");
    println!("{:01$}", b, 5);
    println!("{b:0width$}", width = 5);

    let a1 = format!("{a:0width$}", width = 5);
    println!("{a1}");

    let dec = base64_engine.decode("//".as_bytes()).unwrap();
    println!("{:?}", dec);
    let aice = BASE64.decode("MjAyMi0xMi0yOFQwNzozOTozMi42OTE2MDlaIEVSUk9SIFRMUyBhbGVydCByZWNlaXZlZDogQWxlcnRNZXNzYWdlUGF5bG9hZCB7".as_bytes()).unwrap();
    // println!("{:?}", aice);
    let aiceto = std::str::from_utf8(&aice);
    println!("{aiceto:?}");

    let lalala: &[u8] = &[];

    println!("{}", lalala.len());
    // let testpack: &[u8] = &[
    //     0x68, 0x5b, 0x35, 0xc0, 0x61, 0xb6, 0x00, 0x1d, 0x09, 0x94, 0x65, 0x38, 0x08, 0x00, 0x45,
    //     0x00, 0x00, 0x3b, 0x2d, 0xfd, 0x00, 0x00, 0x40, 0x11, 0xbc, 0x43, 0x83, 0xb3, 0xc4, 0x2e,
    //     0x83, 0xb3, 0xc4, 0xdc, 0x18, 0xdb, 0x18, 0xdb, 0x00, 0x27, 0xe0, 0x3e, 0x05, 0x1d, 0x07,
    //     0x15, 0x08, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x08, 0x07, 0x74, 0x65, 0x73,
    //     0x74, 0x41, 0x70, 0x70, 0x08, 0x01, 0x31, 0x0a, 0x04, 0x1e, 0xcc, 0xe2, 0x51,
    // ];
    // println!("{:?}", testpack);

    println!("\n\n");
    // let testpacket = EthernetPdu::new(&testpack).unwrap();
    // println!("{:?}", testpacket.inner());
    let packet = EthernetPdu::new(&dec).unwrap();
    println!("{:?}", packet.inner());
    match packet.inner() {
        Ok(Ethernet::Ipv4(_)) => println!("ipv4"),
        Ok(Ethernet::Ipv6(_)) => println!("ipv6"),
        Ok(other) => println!("unexpected protocol, {other:?}"),
        Err(e) => println!("parser failure: {e:?}"),
    }
    // let src = packet.source_address();
    // let dst = packet.destination_address();
    // let proto = format!("protocol: 0x{:02x}", packet.protocol());

    // println!("{src:?}, {dst:?}, {proto}");

    // let asd = std::str::from_utf8(&dec[..10]);
    // println!("{asd:?}");
}

fn reader(path: &str) -> Result<BufReader<File>> {
    let path = Path::new(path);
    let file = File::open(path)?;

    Ok(BufReader::new(file))
}

fn writer(path: &str) -> Result<File> {
    let writer = File::create(path)?;

    Ok(writer)
}
