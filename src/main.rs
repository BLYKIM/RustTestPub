#[allow(clippy::wildcard_imports)]
use project::advent::*;
use std::{env, process::exit};

use anyhow::Result;

use crate::settings::Settings;

mod settings;

const USAGE: &str = "\
USAGE:
    project [CONFIG]

FLAGS:
    -h, --help      Prints help information
    -V, --version   Prints version information

ARG:
    <CONFIG>    A TOML config file
";

fn main() -> Result<()> {
    println!("rust-test");
    let settings = if let Some(config_filename) = parse() {
        Settings::from_file(&config_filename)?
    } else {
        Settings::new()?
    };

    println!("host name: {}", settings.host_name);
    // day_one()?;
    // day_two()?;
    // day_three()?;
    // day_four()?;
    // day_five()?;
    // day_six()?;
    day_seven()?;

    Ok(())
}

fn parse() -> Option<String> {
    let mut args = env::args();
    args.next()?;
    let arg = args.next()?;
    if args.next().is_some() {
        eprintln!("Error: too many arguments");
        exit(1);
    }

    if arg == "--help" || arg == "-h" {
        println!("{}", version());
        println!();
        print!("{}", USAGE);
        exit(0);
    }
    if arg == "--version" || arg == "-V" {
        println!("{}", version());
        exit(0);
    }
    if arg.starts_with('-') {
        eprintln!("Error: unknown option: {}", arg);
        eprintln!("\n{}", USAGE);
        exit(1);
    }

    Some(arg)
}

fn version() -> String {
    format!("giganto {}", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]

mod tests {

    #[tokio::test]

    async fn some_fn() {}
}
