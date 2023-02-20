use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
// use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
// use data_encoding::BASE64;
// use pdu::{Ethernet, EthernetPdu};

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

    let serial_num = 1;
    let mut pathbuf = PathBuf::from("value");
    let time = Utc.timestamp_nanos(zerosec).format("%F").to_string();
    let serial = format!("{:01$}", serial_num, 5);
    let merged = format!("{time}-{serial}");
    pathbuf.push(merged);
    println!("{pathbuf:?}, {}", pathbuf.exists());

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
