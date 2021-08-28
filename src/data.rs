use std::iter::Map;

#[derive(Debug, Clone)]
pub struct Value {
    origin: Option<String>,
    current: Option<String>,
}

pub type Array = Vec<Value>;
pub type Table = Map<String, Value>;

#[derive(Debug, Clone)]
pub enum DataType {
    Null,
    Bool(bool),
    String(String),
    Integer(i64),
    Float(f64),
    Array(Array),
    Object(Table),
}
