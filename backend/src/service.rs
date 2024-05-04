use axum::{extract::{Path, State}, Json};
use rbatis::{
    rbdc::{DateTime, Uuid},
    RBatis,
};
use serde_json::json;

use crate::model::Picture;

pub async fn upload_picture(State(rb): State<RBatis>, Json(mut picture): Json<Picture>) -> () {
    picture.uuid = Uuid::new();
    picture.upload_time = DateTime::now();
    println!("insert = {}", json!(picture));
    Picture::insert(&rb, &picture).await.unwrap();
}

pub async fn upload_pictures(State(rb): State<RBatis>, Json(mut pics): Json<Vec<Picture>>) -> () {
    for pic in &mut pics {
        (*pic).uuid = Uuid::new();
        (*pic).upload_time = DateTime::now();
    }
    println!("insert = {}", json!(pics));
    Picture::insert_batch(&rb, pics.as_slice(), pics.len() as u64).await.unwrap();
}

pub async fn get_picture_by_uuid(State(rb): State<RBatis>, Path(uuid): Path<Uuid>) -> Json<Option<Picture>> {
    let pic = Picture::select_by_id(&rb, &uuid).await.unwrap();
    Json(pic)
}
