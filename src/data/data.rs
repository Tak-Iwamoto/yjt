use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

pub type Array = Vec<Data>;
pub type Object = HashMap<String, Data>;

#[derive(Debug, Clone)]
pub enum Data {
    Null,
    Bool(bool),
    String(String),
    Integer(i64),
    Float(f64),
    Array(Box<Array>),
    Object(Box<Object>),
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Bool(value)
    }
}

impl From<&String> for Data {
    fn from(value: &String) -> Self {
        Data::String(*value)
    }
}

impl<'a> From<&'a str> for Data {
    fn from(value: &'a str) -> Self {
        Data::String(value.into())
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Integer(value)
    }
}

impl From<f64> for Data {
    fn from(value: f64) -> Self {
        Data::Float(value)
    }
}

impl From<Array> for Data {
    fn from(array: Array) -> Self {
        Data::Array(Box::new(array))
    }
}

impl From<Object> for Data {
    fn from(object: Object) -> Self {
        Data::Object(Box::new(object))
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Data::Null => write!(f, "null"),
            Data::String(ref value) => write!(f, "{}", value),
            Data::Integer(ref value) => write!(f, "{}", value),
            Data::Float(ref value) => write!(f, "{}", value),
            Data::Bool(ref value) => write!(f, "{}", value),
            Data::Object(ref object) => write!(f, "{{ {} }}", {
                object
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<String>()
            }),
            Data::Array(ref array) => write!(f, "{:?}", {
                array.iter().map(|e| format!("{}, ", e)).collect::<String>()
            }),
        }
    }
}

// impl From<&JsonValue> for Data {
//     fn from(json: &JsonValue) -> Data {
//         match json {
//            JsonValue::Object(ref object)  => {
//                let mut m = HashMap::new();
//                object.into_iter().for_each(|o| m.insert(o.0, o.1));
//            }
//         }

//     }

// }
