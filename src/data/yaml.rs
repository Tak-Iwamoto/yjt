use anyhow::Result;

use crate::data::data::Data;
use std::collections::HashMap;

impl From<&serde_yaml::Value> for Data {
    fn from(serde_yaml_value: &serde_yaml::Value) -> Data {
        match *serde_yaml_value {
            serde_yaml::Value::Null => Data::Null,
            serde_yaml::Value::String(ref value) => Data::from(value),
            serde_yaml::Value::Number(ref value) => {
                if let Some(value) = value.as_i64() {
                    Data::from(value)
                } else if let Some(value) = value.as_f64() {
                    Data::from(value)
                } else {
                    unreachable!("Invalid number")
                }
            }
            serde_yaml::Value::Bool(value) => Data::from(value),
            serde_yaml::Value::Sequence(ref array) => {
                let mut result = Vec::new();
                for value in array {
                    result.push(Self::from(value))
                }
                Data::from(result)
            }
            serde_yaml::Value::Mapping(ref object) => {
                let mut result = HashMap::new();
                for (k, v) in object {
                    if k.as_str().is_some() {
                        result.insert(String::from(k.as_str().unwrap()), Data::from(v));
                    }
                }
                Data::from(result)
            }
        }
    }
}

pub fn parse_yaml(yaml_str: &str) -> Result<Data> {
    let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str)?;
    Ok(Data::from(&yaml))
}

#[cfg(test)]
mod tests {
    use crate::data::Data;

    #[test]
    fn test_from_serde_yaml() {
        let serde_yaml_str = include_str!("../../tests/test.yaml");
        let serde_yaml_value: serde_yaml::Value = serde_yaml::from_str(&serde_yaml_str).unwrap();
        let parsed_data = Data::from(&serde_yaml_value);
        let is_object = match parsed_data {
            Data::Object(_v) => true,
            _ => false,
        };
        assert_eq!(is_object, true)
    }
}
