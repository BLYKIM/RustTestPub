use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_file(path: &str) -> Result<Lines<BufReader<File>>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let rdr = BufReader::new(file);
    Ok(rdr.lines())
}
