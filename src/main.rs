use anyhow::Result;
use data::{parse_json, parse_toml, parse_yaml, Data};
use file::FileType;
use std::{
    env,
    fs::File,
    io::{self, Read},
};

use crate::cli::build_cli;

mod cli;
mod data;
mod event;
mod file;
mod key;
mod query;

fn parse_input(input: &str, filetype: FileType) -> Result<Data> {
    let data = match filetype {
        FileType::Json => parse_json(input)?,
        FileType::Yaml => parse_yaml(input)?,
        FileType::Toml => parse_toml(input)?,
    };
    Ok(data)
}

fn read_file(input_path: &str) -> Result<String> {
    let mut buf = String::new();
    let mut file = File::open(input_path)?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<()> {
    let matches = build_cli().get_matches_from(env::args_os());

    let filetype = match matches.value_of("filetype") {
        Some(v) => FileType::from(v),
        None => FileType::Json,
    };

    match matches.value_of("input_path") {
        Some(input) => {
            let content = read_file(input);
            let data = parse_input(&content.unwrap(), filetype);
            println!("{:?}", data.unwrap());
        }

        None => {
            let mut buffer = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buffer)?;
            let data = parse_input(&buffer, filetype);
            println!("{:?}", data.unwrap());
        }
    }
    Ok(())
}
