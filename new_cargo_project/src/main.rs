// // // //use byte_unit::*;
// // // use conv::*;
// // // //use postgres::{Client, Error, NoTls};
// // // use log::debug;
// // // use psutil::*;
// // // use std::{thread, time::Duration};

// // // const DVGIGAB: u64 = 1_000_000_00; //0.1 자리
// // // const DVPETAB: u64 = 1_000_000_000_00;
// // // const DVTEN: f32 = 10.0;
// // // fn main() {
// // //     let mut sysinf = Vec::new();
// // //     let blocktime = Duration::from_millis(300);
// // //     let cpu_p_c = cpu::CpuPercentCollector::new();

// // //     thread::sleep(blocktime);
// // //     if let Ok(mut cpu_collector) = cpu_p_c {
// // //         if let Ok(cpu_percent) = cpu_collector.cpu_percent() {
// // //             sysinf.push(cpu_percent);
// // //         };
// // //     };

// // //     if let Ok(disk_usage) = disk::disk_usage("/") {
// // //         if let Ok(disk_used) = f32::value_from(disk_usage.used() / DVPETAB) {
// // //             sysinf.push(disk_used);
// // //         };
// // //         if let Ok(disk_total) = f32::value_from(disk_usage.total() / DVPETAB) {
// // //             sysinf.push(disk_total);
// // //         };
// // //     };

// // //     if let Ok(memory_usage) = memory::virtual_memory() {
// // //         if let Ok(memory_used) = f32::value_from(memory_usage.used() / DVGIGAB) {
// // //             sysinf.push(memory_used / DVTEN);
// // //         };
// // //         if let Ok(memory_total) = f32::value_from(memory_usage.total() / DVGIGAB) {
// // //             sysinf.push(memory_total / DVTEN);
// // //         };
// // //     };

// // //     dbg!(&sysinf);
// // //     for i in &sysinf {
// // //         println!("{}", i);
// // //     }

// // //     // let mut status = String::new();
// // //     // let convnum = u128::value_from(number.into()).unwrap();
// // //     // let fnumber = f32::value_from(f6num).unwrap();

// // //     // status.push_str(&format!("Cpu: {:.2}%", &number));
// // //     // status.push('%');

// // //     // println!("{}", status);

// // //     // if let Some(disk_usage) = Some(disk::disk_usage("/").unwrap()) {
// // //     //     //let disk_total = disk_usage.total();
// // //     //     let disk_total = Byte::from_bytes(disk_usage.total() as u128)
// // //     //         .get_adjusted_unit(ByteUnit::GB)
// // //     //         .get_value() as f32;
// // //     //     status.push_str(&format!("{:.2}", disk_total));
// // //     // };
// // //     // //let adbyte = byte.get_adjusted_unit(ByteUnit::GB).get_value();
// // //     // //let adbyte = abyte.get_value() as f32;
// // //     // println!("{}GB", status);
// // // }

// // // // fn main() -> Result<(), Error> {
// // // //     let mut client = Client::connect("postgresql://jgkim:jgkim@172.17.0.2:5432/resourcedb", NoTls)?;

// // // //     client.batch_execute(
// // // //         "
// // // //             id              SERIAL PRIMARY KEY,
// // // //         CREATE TABLE IF NOT EXISTS resourcet (
// // // //             cpu             FLOAT4 NOT NULL,
// // // //             disk_used       FLOAT4 NOT NULL,
// // // //             disk_total      FLOAT4 NOT NULL,
// // // //             memory_used     FLOAT4 NOT NULL,
// // // //             memory_total    FLOAT4 NOT NULL,
// // // //             time            TIMESTAMP NOT NULL
// // // //             )
// // // //     ",
// // // //     )?;

// // // //     // client.batch_execute(
// // // //     //     "
// // // //     //     CREATE TABLE IF NOT EXISTS book  (
// // // //     //         id              SERIAL PRIMARY KEY,
// // // //     //         title           VARCHAR NOT NULL,
// // // //     //         author_id       INTEGER NOT NULL REFERENCES author
// // // //     //         )
// // // //     // ",
// // // //     // )?;

// // // //     Ok(())
// // // // }
// // //
// // use std::fs::File;
// // use std::io::{BufRead, BufReader, Read};
// // use std::thread;
// // use std::time::Duration;
// // fn main() -> std::io::Result<()> {
// //     use std::time::Instant;
// //     let now = Instant::now();

// //     let mut str = String::new();
// //     let f1 = File::open("tutorial.txt")?;
// //     let mut reader = BufReader::new(f1);
// //     //let result = fs::read("tutorial.txt")?;
// //     let handle = thread::spawn(|| {
// //         for i in 1..10 {
// //             println!("{} spawn", i);
// //             thread::sleep(Duration::from_millis(1));
// //         }
// //     });

// //     //println!("{}", result.len());
// //     //let f2 = reader.lines();
// //     // for line in f2.flatten() {
// //     //     println!("{}", line);
// //     // }
// //     reader.read_to_string(&mut str)?;
// //     println!("{}", str);
// //     for i in 1..7 {
// //         println!("{} main", i);
// //         thread::sleep(Duration::from_millis(1));
// //     }
// //     handle.join().unwrap();

// //     let my_vec = vec![
// //         "awrqdff",
// //         "basdasd",
// //         "ccasasdads",
// //         "awrqdff",
// //         "basdasd",
// //         "ccasasdads",
// //         "awrqdff",
// //         "basdasd",
// //         "ccasasdads",
// //     ];
// //     for i in my_vec.chunks(4) {
// //         println!("{:?}", i);
// //     }
// //     let elapsed = now.elapsed();
// //     println!("Elapsed: {:.2?}", elapsed);
// //     Ok(())
// // }

// use lazy_static::lazy_static;
// use regex::Regex;
// use std::{
//     fs::{self, File},
//     io::{self, BufRead, Write},
//     sync::{mpsc, Arc},
//     thread,
// };

// // pub fn parsing<P>(filename: P)
// // where
// //     P: AsRef<Path>,
// // {
// //     lazy_static! {
// //         static ref RE: Regex = Regex::new(
// //             r#"(?x)
// //     (?P<ip>\S+)\s
// //     (?P<minus>\S*)\s
// //     (?P<uid>\S*)\s
// //     (?P<timedate>\[[^\]]+\])\s"
// //     (?P<method>[A-Z]*[^"]*)"\s
// //     (?P<code>[0-9]{3})\s
// //     (?P<size>[0-9]*)\s"
// //     (?P<url>[^"]*)"\s"
// //     (?P<info>[^"]*)"\n?"#,
// //         )
// //         .unwrap();
// //     }

// //     if let Ok(lines) = open_files(filename) {
// //         let v = lines.flatten().collect::<Vec<_>>();
// //         println!("{}", v.len());
// //         let counter = v.len() / 20;

// //         let mut threads = Vec::new();
// //         let (tx, rx) = mpsc::channel();

// //         for chunk in v.chunks(counter) {
// //             let tx = tx.clone();
// //             let th = thread::spawn(move || {
// //                 let response = String::new();
// //                 for i in chunk {
// //                     for x in RE.captures_iter(&i) {}
// //                 }
// //                 tx.send(response).unwrap();
// //             });
// //             threads.push(th);
// //         }
// //         // for line in lines.flatten() {
// // for x in RE.captures_iter(&line) {
// //     println!(
// //         "ip: {:?} | uid: {:?} | time: {:?} | method: {:?} | code: {:?} | url: {:?}",
// //         &x["ip"], &x["uid"], &x["timedate"], &x["method"], &x["code"], &x["url"],
// //     );
// // }
// //         // }

// //         for _ in 0..20 {
// //             println!("got {}", rx.recv().unwrap());
// //         }
// //     }

// //     //let mut threads = Vec::new();

// //     // for t in threads {
// //     //     t.join().expect("thread failed");
// //     // }
// // }

// // fn open_files<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// // where
// //     P: AsRef<Path>,
// // {
// //     let file = File::open(filename)?;
// //     Ok(io::BufReader::new(file).lines())
// // }

// pub fn parsec(filename: String) {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(
//             r#"(?x)
//     (?P<ip>\S+)\s
//     (?P<minus>\S*)\s
//     (?P<uid>\S*)\s
//     (?P<timedate>\[[^\]]+\])\s"
//     (?P<method>[A-Z]*[^"]*)"\s
//     (?P<code>[0-9]{3})\s
//     (?P<size>[0-9]*)\s"
//     (?P<url>[^"]*)"\s"
//     (?P<info>[^"]*)"\n?"#,
//         )
//         .unwrap();
//     }

//     let linesec = file_to_vec(filename).expect("error file_to_vec");
//     let mut index = 0;
//     let len = linesec.len() as u32;
//     let cnt = linesec.len() / 100;

//     let vec_ref = Arc::new(linesec);
//     let (tx, rx) = mpsc::channel();

//     let mut vector = vec![];

//     for _ in 0..100 {
//         vector.push(vec_ref.clone());
//     }

//     let mut children = vec![];
//     for x in vector {
//         let tx = tx.clone();
//         children.push(thread::spawn(move || {
//             for line in &x[index..index + cnt] {
//                 for x in RE.captures_iter(line) {
//                     let response = format!(
//                         "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
//                         &x["ip"],
//                         &x["minus"],
//                         &x["uid"],
//                         &x["timedate"],
//                         &x["method"],
//                         &x["code"],
//                         &x["size"],
//                         &x["url"],
//                         &x["info"],
//                     );
//                     tx.send(response).unwrap();
//                 }
//             }
//         }));
//         index = index + cnt;
//     }

//     let mut new_file = fs::OpenOptions::new()
//         .write(true)
//         .create(true)
//         .append(true)
//         .open("regex_file.csv")
//         .expect("new_file error");

//     new_file
//         .write_all("ip, minus, uid, timedate, method, code, size, url, info\n".as_bytes())
//         .expect("init new_file error");

//     for _ in 0..len {
//         let _wln = writeln!(new_file, "{}", rx.recv().unwrap());
//     }

//     //println!("{}", linesec.len());
//     //let cnt = linesec.len() / 20;
//     //println!("{}", cnt);
//     //let l_slices: Vec<&[String]> = linesec.chunks(cnt).collect();
//     // let mut text = l_slices[0];
//     // for i in text {
//     //     for x in RE.captures_iter(i) {
//     //         println!(
//     //             "ip: {:?} | uid: {:?} | time: {:?} | method: {:?} | code: {:?} | url: {:?}",
//     //             &x["ip"], &x["uid"], &x["timedate"], &x["method"], &x["code"], &x["url"],
//     //         );
//     //     }
//     // }
//     //let mut threads = Vec::new();
//     //let (tx, rx) = mpsc::channel();
//     //let l_slices = l_slices.clone();
//     // for sliced in l_slices {
//     //     //let tx = tx.clone();

//     //     let th = thread::spawn(move || {
//     //         for texts in sliced {
//     //             for x in RE.captures_iter(texts) {
//     //                 println!(
//     //                     "ip: {:?} | uid: {:?} | time: {:?} | method: {:?} | code: {:?} | url: {:?}",
//     //                     &x["ip"], &x["uid"], &x["timedate"], &x["method"], &x["code"], &x["url"],
//     //                 );
//     //             }
//     //         }
//     //         //let response = format!("return {}", i);
//     //         //tx.send(response).unwrap();
//     //     });

//     //     threads.push(th);
//     // }

//     // for _ in 0..20 {
//     //     println!("got {}", rx.recv().unwrap());
//     // }
// }

// fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
//     let file_in = File::open(filename)?;
//     let file_reader = io::BufReader::new(file_in);

//     Ok(file_reader.lines().filter_map(io::Result::ok).collect())
// }

fn main() {}

