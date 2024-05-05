use axum::{
    routing::{delete, get, post},
    Router,
};
use fast_log::plugin::packer::{GZipPacker, LogPacker, ZipPacker};
use log::info;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use utils::config::Config;

pub mod model;
pub mod service;
pub mod utils;

#[tokio::main]
async fn main() {
    let conf = utils::config::get_config().unwrap();

    set_fast_log(&conf);

    let rb = RBatis::new();
    rb.link(MysqlDriver {}, &conf.db).await.unwrap();

    let app = Router::new()
        .route("/picture", post(service::upload_picture))
        .route("/pictures", post(service::upload_pictures))
        .route("/picture/:uuid", get(service::get_picture_by_uuid))
        .route("/pictures", get(service::get_pictures))
        .route("/picture/:uuid", delete(service::delete_picture))
        .with_state(rb);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", conf.ip, conf.port))
        .await
        .unwrap();
    info!("[nose] Server established");
    axum::serve(listener, app).await.unwrap();
}

fn set_fast_log(conf: &Config) {
    match conf.log.packer {
        utils::config::Packer::LogPacker => {
            fast_log::init(
                fast_log::Config::new()
                    .console()
                    .chan_len(Some(conf.log.buffer))
                    .file_split(
                        &conf.log.file_path,
                        conf.log.file_size,
                        conf.log.keep_type,
                        LogPacker {},
                    ),
            )
            .expect("rbatis init fail");
        }
        utils::config::Packer::ZipPacker => {
            fast_log::init(
                fast_log::Config::new()
                    .console()
                    .chan_len(Some(conf.log.buffer))
                    .file_split(
                        &conf.log.file_path,
                        conf.log.file_size,
                        conf.log.keep_type,
                        ZipPacker {},
                    ),
            )
            .expect("rbatis init fail");
        }
        utils::config::Packer::GZipPacker => {
            fast_log::init(
                fast_log::Config::new()
                    .console()
                    .chan_len(Some(conf.log.buffer))
                    .file_split(
                        &conf.log.file_path,
                        conf.log.file_size,
                        conf.log.keep_type,
                        GZipPacker {},
                    ),
            )
            .expect("rbatis init fail");
        }
    }
}
