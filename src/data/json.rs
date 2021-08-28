use std::collections::HashMap;

use super::Data;

impl From<&serde_json::Value> for Data {
    fn from(json_value: &serde_json::Value) -> Data {
        match *json_value {
            serde_json::Value::Null => Data::Null,
            serde_json::Value::String(ref value) => Data::from(value),
            serde_json::Value::Number(ref value) => {
                if let Some(value) = value.as_i64() {
                    Data::from(value)
                } else if let Some(value) = value.as_f64() {
                    Data::from(value)
                } else {
                    unreachable!("Invalid number")
                }
            }
            serde_json::Value::Bool(value) => Data::from(value),
            serde_json::Value::Array(ref array) => {
                let mut result = Vec::new();
                for value in array {
                    result.push(Self::from(value))
                }
                Data::from(result)
            }
            serde_json::Value::Object(ref object) => {
                let mut result = HashMap::new();
                for (k, v) in object {
                    result.insert(k.into(), Data::from(v));
                }
                Data::from(result)
            }
        }
    }
}
