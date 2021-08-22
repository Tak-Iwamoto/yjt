use std::{
    env,
};

use anyhow::Result;
mod input;

fn main() {
    let json_path = env::args().nth(1);
    let yaml_path = env::args().nth(2);
    let toml_path = env::args().nth(3);
    let json = input::read_json_data(json_path.unwrap());
    let yaml = input::read_yaml_data(yaml_path.unwrap());
    let toml = input::read_toml_data(toml_path.unwrap());
    println!("{}", json.unwrap());
    println!("{:?}", yaml.unwrap());
    println!("{}", toml.unwrap()["title"]);
}
