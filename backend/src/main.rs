use axum::{
    routing::{get, post}, Router
};
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;

pub mod service;
pub mod model;

#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    let rb = RBatis::new();
    rb.link(MysqlDriver {}, "mysql://awei:123456@localhost:3306/nose_dev").await.unwrap();

    let app = Router::new()
        .route("/picture", post(service::upload_picture))
        .route("/pictures", post(service::upload_pictures))
        .route("/picture/:uuid", get(service::get_picture_by_uuid))
        .with_state(rb);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
