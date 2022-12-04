use crate::error::Error;
use crate::task::*;
use std::fs;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "client", about = "Utility for client")]
pub enum Cmd {
    /// Run client
    Start(StartCmd),
}

#[derive(Debug, StructOpt)]
pub struct Config {
    /// relaychain websocket server endpoint
    #[structopt(short, long, default_value = "ws")]
    pub ws: String,
    /// AssetId defined in pallet_asset
    #[structopt(short, long, default_value = "0")]
    pub asset_id: usize,
    /// the time between sending two tx
    #[structopt(long, default_value = "60000")]
    pub millisecs_per_block: u64,
    /// the path to csv data
    #[structopt(long)]
    pub json_path: String,
    #[structopt(long, default_value = ".local")]
    pub output_dir: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "start", about = "Start service")]
pub struct StartCmd {
    #[structopt(flatten)]
    pub config: Config,

    /// the keystore for signing
    #[structopt(short, long)]
    pub key_store: Option<String>,

    /// the password of keystore
    #[structopt(short, long)]
    pub password: Option<String>,
}

impl StartCmd {
    fn run(&self) -> Result<(), Error> {
        subscan::parse_for_subscan_url(&self.config.json_path, &self.config.output_dir);
        Ok(())
    }
}

pub fn run() -> Result<(), Error> {
    match Cmd::from_args() {
        Cmd::Start(cmd) => cmd.run(),
    };
    Ok(())
}
