use crate::data::data::Data;
use std::collections::HashMap;

impl From<&toml::Value> for Data {
    fn from(toml_value: &toml::Value) -> Data {
        match *toml_value {
            toml::Value::String(ref value) => Data::from(value),
            toml::Value::Integer(ref value) => Data::from(*value),
            toml::Value::Float(ref value) => Data::from(*value),
            toml::Value::Boolean(value) => Data::from(value),
            toml::Value::Array(ref array) => {
                let mut result = Vec::new();
                for value in array {
                    result.push(Self::from(value))
                }
                Data::from(result)
            }
            toml::Value::Table(ref object) => {
                let mut result = HashMap::new();
                for (k, v) in object {
                    result.insert(k.into(), Data::from(v));
                }
                Data::from(result)
            }
            toml::Value::Datetime(ref value) => Data::from(&value.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::data::Data;

    #[test]
    fn test_from_toml() {
        let toml_str = include_str!("../../tests/test.toml");
        let toml_value: serde_json::Value = toml::from_str(&toml_str).unwrap();
        let parsed_data = Data::from(&toml_value);
        let is_object = match parsed_data {
            Data::Object(_v) => true,
            _ => false,
        };
        assert_eq!(is_object, true)
    }
}
