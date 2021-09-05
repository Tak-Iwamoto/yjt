pub enum FileType {
    Json,
    Yaml,
    Toml,
}

impl From<&str> for FileType {
    fn from(value: &str) -> FileType {
        match &value[..] {
            "json" => FileType::Json,
            "yaml" => FileType::Yaml,
            "toml" => FileType::Toml,
            _ => FileType::Json,
        }
    }
}
