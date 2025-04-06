use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::net::TcpListener;
use tracing::info;

pub mod model;
pub mod service;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub img_path: String,
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let conf = utils::config::get_config().unwrap();

    let mut opt = ConnectOptions::new(&conf.db);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await.unwrap();
    let app_state = AppState {
        db,
        img_path: conf.img_path,
    };

    let app = Router::new()
        .route("/picture", post(service::upload_picture))
        .route(
            "/pictures",
            get(service::get_pictures)
            .post(service::upload_pictures),
        )
        .route(
            "/picture/{uuid}",
            get(service::get_picture_by_uuid)
            .delete(service::delete_picture_by_uuid),
        )
        .with_state(app_state);
    let listener = TcpListener::bind(format!("{}:{}", conf.ip, conf.port))
        .await
        .unwrap();
    info!("[nose] Server established");
    axum::serve(listener, app).await.unwrap();
}
