use anyhow::Result;
use clap::Parser;
use projector_rust::{config::Config, opts::Opts};
fn main() -> Result<()> {
    let opts = Opts::parse();
    let config: Config = Opts::parse().try_into()?;

    println!("Opts: {:?}", opts);
    println!("Config: {:?}", config);
    return Ok(());
}
