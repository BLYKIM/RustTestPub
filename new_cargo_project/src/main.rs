use byte_unit::*;
use psutil::*;
fn main() {
    let number: f32 = 1.25414141;
    let mut status = String::new();

    status.push_str(&format!("Cpu: {:.2}%", &number));
    status.push('%');

    println!("{}", status);

    if let Some(disk_usage) = Some(disk::disk_usage("/").unwrap()) {
        //let disk_total = disk_usage.total();
        let disk_total = Byte::from_bytes(disk_usage.total() as u128)
            .get_adjusted_unit(ByteUnit::GB)
            .get_value() as f32;
        status.push_str(&format!("{:.2}", disk_total));
    };
    //let adbyte = byte.get_adjusted_unit(ByteUnit::GB).get_value();
    //let adbyte = abyte.get_value() as f32;
    println!("{}GB", status);
}
