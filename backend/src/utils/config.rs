use std::{error::Error, fs, io::Read, time::Duration};

use fast_log::{consts::LogSize, plugin::file_split::KeepType};
use log::warn;
use serde::Deserialize;

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
    pub file_size: LogSize,
    pub keep_type: KeepType,
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
        file_size: LogSize::B(0),
        keep_type: KeepType::All,
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
            conf.log.file_size = LogSize::parse(&entry.log.file_size).unwrap_or_else(|err| {
                warn!("log file size parse error: {}", err);
                LogSize::B(0)
            });

            if entry.log.keep_type == "all" {
                conf.log.keep_type = KeepType::All;
            } else if entry.log.keep_type == "day" {
                conf.log.keep_type = KeepType::KeepTime(Duration::from_secs(24 * 3600));
            } else if entry.log.keep_type == "week" {
                conf.log.keep_type = KeepType::KeepTime(Duration::from_secs(7 * 24 * 3600));
            } else {
                conf.log.keep_type = KeepType::KeepNum(entry.log.keep_type.parse().unwrap());
            }

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
