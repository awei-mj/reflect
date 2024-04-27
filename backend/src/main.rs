use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router
};
use model::Picture;
use rbatis::{rbdc::{DateTime, Uuid}, RBatis};
use rbdc_mysql::MysqlDriver;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod service;
pub mod model;

#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    let rb = RBatis::new();
    rb.link(MysqlDriver {}, "mysql://awei:123456@localhost:3306/nose_dev").await.unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/test", post(test))
        .with_state(rb);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn test(State(rb): State<RBatis>) -> (){
    let pic = Picture {
        uuid: Uuid::new(),
        name: "test".into(),
        size: 114514,
        width: 114,
        height: 514,
        upload_time: DateTime::now(),
        content: vec![3u8].into(),
    };
    Picture::insert(&rb, &pic).await.unwrap();
    println!("insert = {}", json!(pic));
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
