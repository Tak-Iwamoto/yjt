use std::{
    fs,
    io::{BufReader, Read},
};

use anyhow::{anyhow, Result};

fn read_file(path: String) -> Result<String> {
    let mut content = String::new();

    let mut reader = fs::File::open(path).map(|f| BufReader::new(f))?;
    reader.read_to_string(&mut content)?;

    Ok(content)
}

pub fn read_json_data(filename: String) -> Result<serde_json::Value> {
    let content = read_file(filename);
    if let Ok(c) = content {
        let v: serde_json::Value = serde_json::from_str(&c)?;
        Ok(v)
    } else {
        Err(anyhow!("Failed to open file"))
    }
}

pub fn read_yaml_data(filename: String) -> Result<serde_yaml::Value> {
    let content = read_file(filename);
    if let Ok(c) = content {
        let v: serde_yaml::Value = serde_yaml::from_str(&c)?;
        Ok(v)
    } else {
        Err(anyhow!("Failed to open file"))
    }
}

pub fn read_toml_data(filename: String) -> Result<toml::Value> {
    let content = read_file(filename);
    if let Ok(c) = content {
        let v: toml::Value = toml::from_str(&c)?;
        Ok(v)
    } else {
        Err(anyhow!("Failed to open file"))
    }
}
