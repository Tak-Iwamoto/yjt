use std::{
    fs,
    io::{BufReader, Read},
};

use anyhow::{anyhow, Result};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use toml::Value as TomlValue;

fn read_file(path: String) -> Result<String> {
    let mut content = String::new();

    let mut reader = fs::File::open(path).map(|f| BufReader::new(f))?;
    reader.read_to_string(&mut content)?;

    Ok(content)
}

pub fn read_json_data(filename: String) -> Result<JsonValue> {
    let content = read_file(filename);
    if let Ok(c) = content {
        let v: JsonValue = serde_json::from_str(&c)?;
        Ok(v)
    } else {
        Err(anyhow!("Failed to open file"))
    }
}

pub fn read_yaml_data(filename: String) -> Result<YamlValue> {
    let content = read_file(filename);
    if let Ok(c) = content {
        let v: YamlValue = serde_yaml::from_str(&c)?;
        Ok(v)
    } else {
        Err(anyhow!("Failed to open file"))
    }
}

pub fn read_toml_data(filename: String) -> Result<TomlValue> {
    let content = read_file(filename);
    if let Ok(c) = content {
        let v: TomlValue = toml::from_str(&c)?;
        Ok(v)
    } else {
        Err(anyhow!("Failed to open file"))
    }
}
