use anyhow::Result;
use clap::Parser;
use projector_rust::{
    config::{Config, Operation},
    opts::Opts,
    projector::Projector,
};
fn main() -> Result<()> {
    // let opts = Opts::parse();
    let config: Config = Opts::parse().try_into()?;
    let mut projector = Projector::from_config(config.config, config.pwd);

    // println!("Opts: {:?}", opts);
    // println!("Config: {:?}", config);
    match config.operation {
        Operation::Print(None) => {
            let value = projector.get_value_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value);
        }
        Operation::Print(Some(key)) => {
            let _value = projector.get_value(&key).map(|x| {
                println!("{}", x);
            });
        }
        Operation::Add(key, value) => {
            projector.set_value(key, value);
            projector.save()?;
        }
        Operation::Remove(key) => {
            projector.remove_value(&key);
            projector.save()?;
        }
    }
    return Ok(());
}
