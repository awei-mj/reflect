use axum::{extract::{Path, State}, Json};
use rbatis::{
    rbdc::{DateTime, Uuid},
    RBatis,
};

use crate::model::Picture;

pub async fn upload_picture(State(rb): State<RBatis>, Json(mut picture): Json<Picture>) -> () {
    let fut = rb.acquire_begin();
    picture.uuid = Uuid::new();
    picture.upload_time = DateTime::now();
    let mut tx = fut.await.unwrap();
    Picture::insert(&tx, &picture).await.unwrap();
    tx.commit().await.unwrap();
}

pub async fn upload_pictures(State(rb): State<RBatis>, Json(mut pics): Json<Vec<Picture>>) -> () {
    let fut = rb.acquire_begin();
    for pic in &mut pics {
        (*pic).uuid = Uuid::new();
        (*pic).upload_time = DateTime::now();
    }
    let mut tx = fut.await.unwrap();
    Picture::insert_batch(&tx, pics.as_slice(), pics.len() as u64).await.unwrap();
    tx.commit().await.unwrap();
}

pub async fn get_picture_by_uuid(State(rb): State<RBatis>, Path(uuid): Path<Uuid>) -> Json<Option<Picture>> {
    let pic = Picture::select_by_id(&rb, uuid).await.unwrap();
    Json(pic)
}

pub async fn get_pictures(State(rb): State<RBatis>) -> Json<Vec<Picture>> {
    let pics = Picture::select_all(&rb).await.unwrap();
    Json(pics)
}

pub async fn delete_picture(State(rb): State<RBatis>, Path(uuid): Path<Uuid>) -> () {
    let mut tx = rb.acquire_begin().await.unwrap();
    Picture::delete_by_id(&tx, uuid).await.unwrap();
    tx.commit().await.unwrap();
}
