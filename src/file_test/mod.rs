mod wlkdir;

use anyhow::Result;
use chrono::Utc;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};
use toml_edit::{value, Document};
pub use wlkdir::test_dir;

///
/// # Panics
/// todo
pub fn file_test() {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("../../../test.txt")
        .unwrap();
    let enable = "0\n";
    //    for line in file.lines() {
    //        println!("{}", line);
    //    }
    file.write_all(enable.as_bytes()).unwrap();
    //    for line in file.lines() {
    //        println!("{}", line);
    //    }
}

pub fn toml() {
    let toml = fs::read_to_string("tests/config.toml").unwrap();
    let mut doc = toml.parse::<Document>().unwrap();

    // println!("\n{}", doc.to_string());
    let name = doc.get("host_name").unwrap().to_string();
    let retention = doc.get("retention").unwrap().to_string();
    let graphql = doc.get("graphql_address").unwrap().to_string();
    let ingestion = doc.get("ingest_address").unwrap().to_string();
    let publish = doc.get("publish_address").unwrap().to_string();
    println!("{name}\n{ingestion}\n{publish}\n{graphql}\n{retention}");

    let strs = "100d".to_string();
    doc["retention"] = value(strs);
    doc["host_name"] = value("BLYKIM");
    doc["graphql_address"] = value("127.0.0.1:8444");
    let name = doc.get("host_name").unwrap().to_string();
    let retention = doc.get("retention").unwrap().to_string();
    let graphql = doc.get("graphql_address").unwrap().to_string();

    println!("{name}\n{ingestion}\n{publish}\n{graphql}\n{retention}");
    println!("========================================");
    let output = doc.to_string();
    let mut toml_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tests/config.toml")
        .unwrap();

    writeln!(toml_file, "{output}");

    println!("{output}");
}

// debug log writer from r
///
/// # Errors
///
/// FUCKCLIPPY
///
pub fn log_debug(msg: &str) -> Result<()> {
    if let Ok(mut writer) = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("roxy.log")
    {
        writeln!(writer, "{:?}: {msg}", Utc::now())?;
    }

    Ok(())
}

///
/// # Panics
///
/// asdasds
///
pub async fn read_file(path: &str) {
    let file = File::open(path).unwrap();
    let rdr = BufReader::new(file);
    let mut lines = rdr.lines();

    loop {
        if let Some(Ok(line)) = lines.next() {
            println!("{line}");
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }
}
