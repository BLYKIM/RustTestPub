use crate::settings::{self, trace_info};
use anyhow::{anyhow, Result};
use roxy::{run_roxy, Node, NodeRequest, SubCommand};
use std::{thread, time::Duration};

pub fn call_roxy() {
    let host = "hostname_A";
    let process = "Roxy";
    let blocktime = Duration::from_secs(30);

    if let Some(thing) = settings::config("url") {
        if let Ok(req) =
            NodeRequest::new::<String>(host, process, Node::PostgresQL(SubCommand::Update), thing)
        {
            trace_info("I'm in here!!");
            loop {
                match send_request::<String>(&req) {
                    Ok(r) => println!("Response: {:?}", r),
                    Err(e) => eprintln!("Error: {}", e),
                }
                println!("waiting 30s");
                thread::sleep(blocktime);
            }
        }
    }
}

fn send_request<T>(request: &NodeRequest) -> Result<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let host = "hostname_A";
    if request.host == *host && request.process == "Roxy" {
        run_roxy::<T>(request.to_task())
    } else if request.host == *host && request.process == "Hog" {
        // if I am Hog
        unimplemented!();
    } else {
        // forward this request to the destination host
        Err(anyhow!("not me"))
    }
}
