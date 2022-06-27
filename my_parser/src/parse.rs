use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{self, BufRead, Write},
    path::Path,
    sync::{mpsc, Arc},
    thread,
};

pub fn parsec(filepath: &Path) {
    // // regular expression
    // lazy_static! {
    //     static ref RE: Regex = Regex::new(
    //         r#"(?x)
    // (?P<ip>\S+)\s
    // (?P<minus>\S*)\s
    // (?P<uid>\S*)\s
    // (?P<timedate>\[[^\]]+\])\s"
    // (?P<method>[A-Z]*[^"]*)"\s
    // (?P<code>[0-9]{3})\s
    // (?P<size>[0-9]*)\s"
    // (?P<url>[^"]*)"\s"
    // (?P<info>[^"]*)"\n?"#,
    //     )
    //     .unwrap();
    // }

    let linesec = file_to_vec(filepath).expect("error file_to_vec"); // Vec<String> log file read , split n thread, n vector
    let mut index = 0;
    let n = 10; // n thread  ,  <- take n_core (n-1)
    let len = linesec.len() as u32; // to take receive
    let cnt = linesec.len() / n; // counter to n thread

    let vec_ref = Arc::new(linesec); // arc
    let (tx, rx) = mpsc::channel();

    let mut vector = vec![]; // log vector clone for thread
    for _ in 0..n {
        vector.push(vec_ref.clone());
    }

    let mut children = vec![]; // manage thread vector

    for x in vector {
        // each arc vector string of n vector
        let tx = tx.clone(); // clone tx
        children.push(thread::spawn(move || {
            // spawn thread
            for line in &x[index..index + cnt] {
                // iter string from index to index+counter
                for x in RE.captures_iter(line) {
                    tx.send(format!(
                        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                        &x["ip"],
                        &x["minus"],
                        &x["uid"],
                        &x["timedate"],
                        &x["method"],
                        &x["code"],
                        &x["size"],
                        &x["url"],
                        &x["info"],
                    ))
                    .unwrap();
                }
            }
        }));
        index = index + cnt; // index++
    }

    //create or append csv file
    let mut new_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("regex_file.csv")
        .expect("new_file error");

    //write csv column
    new_file
        .write_all("ip, minus, uid, timedate, method, code, size, url, info\n".as_bytes())
        .expect("init new_file error");

    for _ in 0..len {
        let _wln = writeln!(new_file, "{}", rx.recv().unwrap()); // write recv to csv loop 0 to vector length
    }
}

fn file_to_vec(filepath: &Path) -> io::Result<Vec<String>> {
    // log file read
    let file_in = File::open(filepath)?;
    let file_reader = io::BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
