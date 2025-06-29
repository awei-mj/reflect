use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::net::TcpListener;
use tracing::{error, info};
use totp_rs::TOTP;

pub mod model;
pub mod service;
pub mod utils;
pub mod error;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub img_path: String,
    pub totp: TOTP,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = utils::config::get_config().inspect_err(|e| error!("[reflect] {}", e.to_string()))?;

    let file_appender = tracing_appender::rolling::daily(&conf.log_dir, &conf.log_prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(non_blocking)
        .init();

    let totp_url = get_or_generate_totp_url(&conf.totp_url).inspect_err(|e| error!("[reflect] {}", e.to_string()))?;

    let mut opt = ConnectOptions::new(&conf.db);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await.inspect_err(|e| error!("[reflect] {}", e.to_string()))?;
    let app_state = AppState {
        db,
        img_path: conf.img_path,
        totp: TOTP::from_url(totp_url)?,
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
        .route("/totp/verify", post(service::verify_totp))
        .with_state(app_state);
    let listener = TcpListener::bind(format!("{}:{}", conf.ip, conf.port)).await.inspect_err(|e| error!("[reflect] {}", e.to_string()))?;
    info!("[reflect] Server established");
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.inspect_err(|e| error!("[reflect] {}", e.to_string()))?;

    Ok(())
}

fn get_or_generate_totp_url(conf: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    use totp_rs::{TOTP, Algorithm, Secret};

    match conf {
        Some(conf_url) => Ok(conf_url.clone()),
        None => {
            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                Secret::generate_secret().to_bytes()?,
                Some("awei".to_string()),
                "awei".to_string(),
            )?;
            let url = totp.get_url();
            info!("[reflect] Generate totp url: {}", url);
            Ok(url)
        }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).expect("failed to install term signal handler").recv().await;
    };

    tokio::select! {
        _ = ctrl_c => {
            info!("[reflect] Ctrl+C received");
        },
        _ = terminate => {
            info!("[reflect] Term signal received");
        },
    }

    info!("[reflect] Shutting down");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    info!("[reflect] Shutdown complete");
}
