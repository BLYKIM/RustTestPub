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

// #[allow(clippy::wildcard_imports)]
// use project::advent::*;
use crate::settings::Settings;
use anyhow::Result;
use ec::test_3;
use num_enum::IntoPrimitive;
use std::{env, process::exit};

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

    println!("*** day_start ***");
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

    test_3();
    #[cfg(debug_assertions)]
    file_test::toml();

    Ok(())
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

#[derive(Debug, IntoPrimitive)]
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

#[cfg(test)]
mod tests {

    #[tokio::test]

    async fn some_fn() {}
}
