use axum::{
    extract::{Path, State}, response::IntoResponse, Json
};
use crate::model::picture::{self, Entity as Picture};
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait};
use tracing::info;

pub async fn upload_picture(State(db): State<DatabaseConnection>, Json(mut picture): Json<picture::Model>) -> impl IntoResponse {
    info!("[reflect] Post /picture Handling");

    info!("[reflect] Post /picture Done");
}

pub async fn upload_pictures(State(db): State<DatabaseConnection>, Json(mut pics): Json<Vec<picture::Model>>) -> () {
    info!("[reflect] Post /pictures Handling");

    info!("[reflect] Post /pictures Done");
}

pub async fn get_picture_by_uuid(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Json<Option<picture::Model>> {
    info!("[reflect] Get /picture/{} Handling", id);
    let pic = Picture::find_by_id(id).one(&db).await.unwrap();
    info!("[reflect] Get /picture/{} Done", id);
    Json(pic)
}

pub async fn get_pictures(State(db): State<DatabaseConnection>) -> Json<Vec<picture::Model>> {
    info!("[reflect] Get /pictures Handling");
    let pics = Picture::find().all(&db).await.unwrap();
    info!("[reflect] Get /pictures Done");
    Json(pics)
}

pub async fn delete_picture(State(db): State<DatabaseConnection>, Path(uuid): Path<Uuid>) -> () {
    info!("[reflect] Delete /picture/{} Handling", uuid);
    
    info!("[reflect] Delete /picture/{} Done", uuid);
}
