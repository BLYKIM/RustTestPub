//use byte_unit::*;
use conv::*;
//use postgres::{Client, Error, NoTls};
use log::debug;
use psutil::*;
use std::{thread, time::Duration};

const DVGIGAB: u64 = 1_000_000_00; //0.1 자리
const DVPETAB: u64 = 1_000_000_000_00;
const DVTEN: f32 = 10.0;
fn main() {
    let mut sysinf = Vec::new();
    let blocktime = Duration::from_millis(300);
    let cpu_p_c = cpu::CpuPercentCollector::new();

    thread::sleep(blocktime);
    if let Ok(mut cpu_collector) = cpu_p_c {
        if let Ok(cpu_percent) = cpu_collector.cpu_percent() {
            sysinf.push(cpu_percent);
        };
    };

    if let Ok(disk_usage) = disk::disk_usage("/") {
        if let Ok(disk_used) = f32::value_from(disk_usage.used() / DVPETAB) {
            sysinf.push(disk_used);
        };
        if let Ok(disk_total) = f32::value_from(disk_usage.total() / DVPETAB) {
            sysinf.push(disk_total);
        };
    };

    if let Ok(memory_usage) = memory::virtual_memory() {
        if let Ok(memory_used) = f32::value_from(memory_usage.used() / DVGIGAB) {
            sysinf.push(memory_used / DVTEN);
        };
        if let Ok(memory_total) = f32::value_from(memory_usage.total() / DVGIGAB) {
            sysinf.push(memory_total / DVTEN);
        };
    };

    dbg!(&sysinf);
    for i in &sysinf {
        println!("{}", i);
    }

    // let mut status = String::new();
    // let convnum = u128::value_from(number.into()).unwrap();
    // let fnumber = f32::value_from(f6num).unwrap();

    // status.push_str(&format!("Cpu: {:.2}%", &number));
    // status.push('%');

    // println!("{}", status);

    // if let Some(disk_usage) = Some(disk::disk_usage("/").unwrap()) {
    //     //let disk_total = disk_usage.total();
    //     let disk_total = Byte::from_bytes(disk_usage.total() as u128)
    //         .get_adjusted_unit(ByteUnit::GB)
    //         .get_value() as f32;
    //     status.push_str(&format!("{:.2}", disk_total));
    // };
    // //let adbyte = byte.get_adjusted_unit(ByteUnit::GB).get_value();
    // //let adbyte = abyte.get_value() as f32;
    // println!("{}GB", status);
}

// fn main() -> Result<(), Error> {
//     let mut client = Client::connect("postgresql://jgkim:jgkim@172.17.0.2:5432/resourcedb", NoTls)?;

//     client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS resourcet (
//             id              SERIAL PRIMARY KEY,
//             cpu             FLOAT4 NOT NULL,
//             disk_used       FLOAT4 NOT NULL,
//             disk_total      FLOAT4 NOT NULL,
//             memory_used     FLOAT4 NOT NULL,
//             memory_total    FLOAT4 NOT NULL,
//             time            TIMESTAMP NOT NULL
//             )
//     ",
//     )?;

//     // client.batch_execute(
//     //     "
//     //     CREATE TABLE IF NOT EXISTS book  (
//     //         id              SERIAL PRIMARY KEY,
//     //         title           VARCHAR NOT NULL,
//     //         author_id       INTEGER NOT NULL REFERENCES author
//     //         )
//     // ",
//     // )?;

//     Ok(())
// }
