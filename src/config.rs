use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            pwd,
            config,
        });
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                let err = anyhow!(
                    "add operation expects 2 arguments but got {}",
                    value.len() - 1
                );
                return Err(err);
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("expect to exist"),
                drain.next().expect("expect to exist"),
            ));
        }
        if term == "rmv" {
            if value.len() != 2 {
                let err = anyhow!(
                    "rmv operation expects 1 arguments but got {}",
                    value.len() - 1
                );
                return Err(err);
            }
            let arg = value.pop().expect("expect to exist");

            return Ok(Operation::Remove(arg));
        }
        if value.len() > 1 {
            let err = anyhow!(
                "print operation expects 0 or 1 arguments but got {}",
                value.len() - 1
            );
            return Err(err);
        }
        let arg = value.pop().expect("expect to exist");

        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let loc = std::env::var("HOME").context("unable to get HOME")?;
    let mut loc = PathBuf::from(loc);

    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    let pwd = std::env::current_dir().context("errored getting current dir")?;
    return Ok(pwd);
}
