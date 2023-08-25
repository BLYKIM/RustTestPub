#[allow(unused)]
mod advent;
#[allow(unused)]
mod anything;
#[allow(unused)]
mod ec;
#[allow(unused)]
mod file_test;
#[allow(unused)]
mod my_parser;
#[allow(unused)]
// pub mod postgres;
#[allow(unused)]
mod settings;

#[allow(unused)]
mod service;

// #[allow(clippy::wildcard_imports)]
// use project::advent::*;
use crate::settings::Settings;
use anyhow::Result;
// use my_parser::to_timestamp_nano;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{Display, Formatter},
    net::{IpAddr, Ipv4Addr},
    process::exit,
};

const USAGE: &str = "\
USAGE:
    project [CONFIG]

FLAGS:
    -h, --help      Prints help information
    -V, --version   Prints version information

ARG:
    <CONFIG>    A TOML config file
";

#[tokio::main]
async fn main() -> Result<()> {
    println!("rust-test");

    let (settings, _pick) = if let Some((config_filename, pick)) = parse() {
        (Settings::from_file(&config_filename)?, pick)
    } else {
        (Settings::new()?, String::new())
    };

    println!("host name: {}", settings.host_name);

    let my_vec = vec![
        "ABC".to_string(),
        String::new(),
        "cde".to_string(),
        String::new(),
        "efg".to_string(),
        "fgh".to_string(),
    ];
    let my_vec2 = vec![1, 2, 3, 10000];
    let empty_string_vec = vec![String::new(), String::new()];
    println!(
        "{:?} is empty? : {}",
        empty_string_vec,
        empty_string_vec.is_empty()
    );

    let zipp = my_vec.iter().zip(my_vec2.iter());
    for a in zipp {
        println!("{a:?}");
    }
    // println!("*** day_start ***");
    // day_one()?;
    // day_two()?;
    // day_three()?;
    // day_four()?;
    // day_five()?;
    // day_six()?;
    // day_seven()?;
    // day_eight()?;
    // day_nine()?;
    // day_ten()?;
    // day_eleven()?;
    // day_twelve()?;

    // read_file("inputs/test.txt").await;

    // test_one()?;
    // test_two(&pick)?;
    // to_timestamp_nano();
    // test_3();

    // #[cfg(debug_assertions)]
    // file_test::toml();
    let new = DataField {
        source: "aasd".to_string(),
        src_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
        src_port: 10,
        dst_addr: IpAddr::V4(Ipv4Addr::BROADCAST),
        dst_port: 123,
        proto: 8,
    };

    let new_se = bincode::serialize(&new).unwrap();

    println!("{new_se:?}");

    let a = bincode::deserialize::<DataField>(&[
        0, 0, 0, 0, 72, 167, 32, 248, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 248, 225, 99, 0, 0,
        0, 0, 61, 248, 224, 253, 0, 0, 0, 0, 61, 248, 224, 99, 0, 0, 0, 0, 61, 248, 225, 52, 0, 0,
        0, 0, 61, 248, 225, 103, 0, 0, 0, 0, 61, 248, 224, 197, 0, 0, 0, 0, 61, 248, 225, 110, 0,
        0, 0, 0, 61, 248, 225, 81, 0, 0, 0, 0, 61, 248, 224, 237, 0, 0, 0, 0, 61, 248, 224, 227, 0,
        0, 0, 0, 61, 248, 225, 44, 0, 0, 0, 0, 61, 248, 225, 50, 0, 0, 0, 0, 61, 248, 225, 41, 0,
        0, 0, 0, 61, 248, 225, 96, 0, 0, 0, 0, 61, 248, 225, 84, 0, 0, 0, 0, 61, 248, 224, 52, 0,
        0, 0, 0, 61, 248, 225, 113, 27, 0, 0, 0, 0, 0, 0, 0, 50, 48, 50, 51, 45, 48, 56, 45, 48,
        51, 84, 48, 56, 58, 50, 48, 58, 48, 53, 46, 53, 52, 53, 54, 51, 57, 90, 30, 0, 0, 0, 0, 0,
        0, 0, 50, 48, 50, 51, 45, 48, 56, 45, 48, 51, 84, 48, 56, 58, 50, 48, 58, 49, 51, 46, 53,
        51, 53, 51, 52, 54, 48, 48, 51, 90, 6,
    ]);

    println!("{a:?}");
    println!("{:?}", bincode::deserialize::<DataField>(&new_se));

    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct DataField {
    pub source: String,
    pub src_addr: IpAddr,
    pub src_port: u16,
    pub dst_addr: IpAddr,
    pub dst_port: u16,
    pub proto: u8,
}

fn parse() -> Option<(String, String)> {
    let mut args = env::args();
    args.next()?;
    let arg = args.next()?;
    let option = args.next();
    if args.next().is_some() {
        eprintln!("Error: too many arguments");
        exit(1);
    }

    if arg == "--help" || arg == "-h" {
        println!("{}", version());
        println!();
        print!("{USAGE}");
        exit(0);
    }
    if arg == "--version" || arg == "-V" {
        println!("{}", version());
        exit(0);
    }
    if arg.starts_with('-') {
        eprintln!("Error: unknown option: {arg}");
        eprintln!("\n{USAGE}");
        exit(1);
    }

    if let Some(pick) = option {
        return Some((arg, pick));
    }
    Some((arg, String::new()))
}

fn version() -> String {
    format!("project {}", env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, IntoPrimitive, FromPrimitive)]
#[repr(u16)]
pub enum Qtype {
    A = 1,
    Ns,
    Md,
    Mf,
    Cname,
    Soa,
    Mb,
    Mg,
    Mr,
    Null,
    Wks,
    Ptr,
    Hinfo,
    Minfo,
    Mx,
    Txt,
    Rp,
    Afsdb,
    X25,
    Isdn,
    Rt,
    Nsap,
    NsapPtr,
    Sig,
    Key,
    Px,
    Gpos,
    Aaaa,
    Loc,
    Nxt,
    Eid,
    Nimloc,
    Srv,
    Atma,
    Naptr,
    Kx,
    Cert,
    A6,
    Dname,
    Sink,
    Opt,
    Apl,
    Ds,
    Sshfp,
    Ipseckey,
    Rrsig,
    Nsec,
    Dnskey,
    Dhcid,
    Nsec3,
    Nsec3param,
    Tlsa,
    Smimea,
    Hip = 55,
    Ninfo,
    Rkey,
    Talink,
    Cds,
    Cdnskey,
    Openpgpkey,
    Csync,
    Zonemd,
    Svcb,
    Https,
    Spf = 99,
    #[num_enum(default)]
    Unknown,
}

impl Display for Qtype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let upper = match self {
            Self::NsapPtr => "NSAP-PTR".to_string(),
            _ => format!("{self:?}").to_uppercase(),
        };
        write!(f, "{upper}")
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]

    async fn some_fn() {}
}
