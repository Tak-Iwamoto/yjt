mod data;
pub use self::data::Data;

mod json;
mod toml;
mod yaml;

pub use self::json::parse_json;
pub use self::yaml::parse_yaml;
pub use self::toml::parse_toml;
