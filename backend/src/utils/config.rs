use std::{error::Error, fs, io::Read};
use serde::Deserialize;
use tracing::warn;

pub enum Packer {
    LogPacker,
    ZipPacker,
    GZipPacker,
}

#[derive(Debug, Deserialize)]
struct RawLog {
    buffer: usize,
    file_path: String,
    file_size: String,
    keep_type: String,
    packer: String,
}

pub struct Log {
    pub buffer: usize,
    pub file_path: String,
    pub file_size: u32,
    pub keep_type: u32,
    pub packer: Packer,
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    name: String,
    ip: String,
    port: u16,
    db: String,
    log: RawLog,
}

pub struct Config {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub db: String,
    pub log: Log,
}

#[derive(Debug, Deserialize)]
struct AppToml {
    active: String,
    configs: Option<Vec<RawConfig>>,
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut toml = fs::File::open("./application.toml")?;
    let mut str = String::new();
    toml.read_to_string(&mut str)?;
    let app_config: AppToml = toml::from_str(&str)?;
    let log = Log {
        buffer: 0,
        file_path: String::new(),
        file_size: 0,
        keep_type: 0,
        packer: Packer::LogPacker,
    };
    let mut conf = Config {
        name: String::new(),
        ip: String::new(),
        port: 0,
        db: String::new(),
        log: log,
    };
    app_config
        .configs
        .expect("configs not found")
        .into_iter()
        .filter(|entry| app_config.active == entry.name)
        .for_each(|entry| {
            conf.log.buffer = entry.log.buffer;
            conf.log.file_path = entry.log.file_path;
            conf.log.file_size = 0;

            conf.log.keep_type = 0;

            if entry.log.packer == "log" {
                conf.log.packer = Packer::LogPacker;
            } else if entry.log.packer == "zip" {
                conf.log.packer = Packer::ZipPacker;
            } else if entry.log.packer == "gzip" {
                conf.log.packer = Packer::GZipPacker;
            } else {
                warn!("log file packer parse error");
                conf.log.packer = Packer::LogPacker;
            }

            conf.name = entry.name;
            conf.ip = entry.ip;
            conf.port = entry.port;
            conf.db = entry.db;
        });
    Ok(conf)
}
