use anyhow::Result;
use std::{
    env,
    io::{self, Read},
};

use crate::cli::build_cli;

mod cli;
mod data;
mod event;
mod input;
mod key;
mod query;

fn read_string(value: &str) {
    println!("{}", value);
}
fn main() -> Result<()> {
    let matches = build_cli().get_matches_from(env::args_os());
    match matches.value_of("input") {
        Some(input) => {
            read_string(input);
        }
        None => {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buffer)?;
            read_string(&buffer);
        }
    }
    Ok(())
}
