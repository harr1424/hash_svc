use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) secrets: Secrets,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Secrets {
    pub(crate) en_image: String,
    pub(crate) en_image_p: String,
    pub(crate) es_image: String,
    pub(crate) es_image_p: String,
    pub(crate) fr_image: String,
    pub(crate) po_image: String,
    pub(crate) it_image: String,
    pub(crate) de_image: String,
}

impl Config {
    pub(crate) fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = read_to_string(filename)
            .map_err(|err| format!("Unable to read config file: {}", err))?;
        let config: Config = toml::from_str(&config_str)
            .map_err(|err| format!("Unable to parse config file: {}", err))?;
        Ok(config)
    }
}
