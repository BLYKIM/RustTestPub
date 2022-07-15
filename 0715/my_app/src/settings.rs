use std::{fs, io::Write};

use config::Config;
use tracing::info; // {error, info, Level};

// #[derive(Debug)]
// struct Database {
//     url: String,
//     name: String,
//     password: String,
//     port: String,
//     addr: String,
//     dbname: String,
// }

// #[derive(Debug)]
// pub struct Settings {
//     database: Database,
// }
#[derive(Debug)]
pub enum Kind {
    Url,
    Name,
    Password,
    Port,
    Addr,
    Dbname,
}

// impl Settings {
//     fn something(&self) {}
// }
#[must_use]
pub fn cmd_kind(cmd: &str) -> &'static str {
    match cmd {
        "url" => "database.url",
        "name" => "database.name",
        "password" => "database.password",
        "port" => "database.port",
        "addr" => "database.addr",
        "dbname" => "database.dbname",
        &_ => "database",
    }
}
///read config.toml and get info
#[must_use]
pub fn config(cmd: &str) -> Option<String> {
    let mut somevar = String::new();
    //builder config.toml
    if let Ok(settings) = Config::builder()
        .add_source(config::File::with_name(
            "/Users/jgkim/Desktop/Dev/Project/my_app/src/conf/config",
        ))
        .build()
    {
        info!("{:?}", &settings);

        if let Ok(dbinfo) = settings.get::<String>(cmd_kind(cmd)) {
            somevar.push_str(&dbinfo);
        };
        info!("{}", &somevar);
        if somevar.is_empty() {
            return None;
        }
        return Some(somevar);
    };
    None
}

pub fn trace_info(msg: &str) {
    if let Ok(mut writer) = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("my_app.log")
    {
        let _log = writeln!(writer, "{}", msg);
    }
}
