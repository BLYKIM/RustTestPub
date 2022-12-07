use std::{fs::OpenOptions, io::Write};

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
