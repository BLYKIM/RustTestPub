mod config;
mod graphql;
mod postgres;
mod storage;
mod tls;
mod web;

use self::{
    config::Config,
    graphql::schema,
    postgres::Database,
    storage::{DbOptions, Store},
};
use anyhow::Result;
use std::{env, fs, process::exit};

const USAGE: &str = "\
USAGE:
    server [CONFIG]
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
ARG:
    <CONFIG>    A TOML config file
";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new(parse().as_deref())?;
    run(config).await?;
    Ok(())
}

async fn run(config: Config) -> Result<()> {
    let store_path = config.data_dir.join("db");
    let store_options = DbOptions::new(config.max_open_files, config.max_mb_of_level_base);
    let store = Store::open(&store_path, &store_options)?;

    let db = postgres::Database::new(&config.database_url, &config.roots).await?;

    let cert_pem = fs::read(config.cert)?;
    let key_pem = fs::read(config.key)?;

    let schema = schema(db.clone(), store.clone());

    web::serve(schema, config.server_address, cert_pem, key_pem).await;
    Ok(())
}

/// Parses the command line arguments and returns the first argument.
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

    Some(arg)
}

fn version() -> String {
    format!("server {}", env!("CARGO_PKG_VERSION"))
}
