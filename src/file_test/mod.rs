mod wlkdir;

use anyhow::Result;
use chrono::Utc;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};
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
