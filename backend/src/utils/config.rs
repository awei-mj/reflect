use std::{error::Error, fs, io::Read};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub db: String,
    pub img_path: String,
    pub totp_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AppToml {
    active: String,
    configs: Option<Vec<RawConfig>>,
}

pub fn get_config() -> Result<RawConfig, Box<dyn Error>> {
    let mut toml = fs::File::open("./application.toml")?;
    let mut str = String::new();
    toml.read_to_string(&mut str)?;
    let app_config: AppToml = toml::from_str(&str)?;

    let conf = app_config.configs
        .expect("configs not found")
        .into_iter()
        .filter(|entry| app_config.active == entry.name)
        .next()
        .expect(format!("config {} not found", app_config.active).as_str());
    Ok(conf)
}
