use crate::data::DataType;

pub struct App {
    pub currentValue: DataType,
    pub originalValue: DataType,
    pub query: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            currentValue: DataType::Null,
            originalValue: DataType::Null,
            query: "".to_string(),
        }
    }
}

impl App {
    pub fn new() -> App {
        App { ..App::default() }
    }
}
