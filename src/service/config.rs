use anyhow::{Context, Result};
use config::File;
use serde::Deserialize;
use std::{env, net::SocketAddr, path::PathBuf};

const DEFAULT_ADDRESS: &str = "[::]:3030";
const DEFAULT_DATABASE_URL: &str = "postgres://server@localhost/server";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub cert: PathBuf,
    pub key: PathBuf,
    pub roots: Vec<PathBuf>,
    pub server_address: SocketAddr,
    pub database_url: String,
}

impl Config {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let builder = config::Config::builder()
            .set_default("cert", env::current_dir()?.join("cert.pem").to_str())
            .context("cannot set the default certificate file name")?
            .set_default("key", env::current_dir()?.join("key.pem").to_str())
            .context("cannot set the default private key file name")?
            .set_default("server_address", DEFAULT_ADDRESS)
            .context("cannot set the default server address")?
            .set_default("database_url", DEFAULT_DATABASE_URL)
            .context("cannot set the default database URL")?;

        let config: Config = if let Some(path) = path {
            builder.add_source(File::with_name(path))
        } else {
            builder
        }
        .build()
        .context("cannot build the config")?
        .try_deserialize()?;

        Ok(Self {
            cert: config.cert,
            key: config.key,
            roots: config.roots,
            server_address: config.server_address,
            database_url: config.database_url,
        })
    }
}
