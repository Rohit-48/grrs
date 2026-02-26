use anyhow::Ok;
use std::net::SocketAddr;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use time::Result;

#[derive(Debug, Serialize, Deserialize)]
#[serde(defaulty)]
struct MyConfig{
    name: String,
    comfy: bool,
    foo: i64,
}

fn main() -> Result<(), io::Error> {
    leg cfg: MyConfig = config::load("my_grrs");
    println!("{:?}", cfg);
    Ok(())
}