use axum::{
    extract::{Path, State},
    Json,
};
use log::info;
use rbatis::{
    rbdc::{DateTime, Uuid},
    RBatis,
};

use crate::model::Picture;

pub async fn upload_picture(State(rb): State<RBatis>, Json(mut picture): Json<Picture>) -> () {
    info!("[nose] Post /picture Handling");
    let fut = rb.acquire_begin();
    picture.uuid = Uuid::new();
    picture.upload_time = DateTime::now();
    let mut tx = fut.await.unwrap();
    Picture::insert(&tx, &picture).await.unwrap();
    tx.commit().await.unwrap();
    info!("[nose] Post /picture Done");
}

pub async fn upload_pictures(State(rb): State<RBatis>, Json(mut pics): Json<Vec<Picture>>) -> () {
    info!("[nose] Post /pictures Handling");
    let fut = rb.acquire_begin();
    for pic in &mut pics {
        (*pic).uuid = Uuid::new();
        (*pic).upload_time = DateTime::now();
    }
    let mut tx = fut.await.unwrap();
    Picture::insert_batch(&tx, pics.as_slice(), pics.len() as u64)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    info!("[nose] Post /pictures Done");
}

pub async fn get_picture_by_uuid(
    State(rb): State<RBatis>,
    Path(uuid): Path<Uuid>,
) -> Json<Option<Picture>> {
    info!("[nose] Get /picture/{} Handling", uuid);
    let pic = Picture::select_by_id(&rb, &uuid).await.unwrap();
    info!("[nose] Get /picture/{} Done", uuid);
    Json(pic)
}

pub async fn get_pictures(State(rb): State<RBatis>) -> Json<Vec<Picture>> {
    info!("[nose] Get /pictures Handling");
    let pics = Picture::select_all(&rb).await.unwrap();
    info!("[nose] Get /pictures Done");
    Json(pics)
}

pub async fn delete_picture(State(rb): State<RBatis>, Path(uuid): Path<Uuid>) -> () {
    info!("[nose] Delete /picture/{} Handling", uuid);
    let mut tx = rb.acquire_begin().await.unwrap();
    Picture::delete_by_id(&tx, &uuid).await.unwrap();
    info!("[nose] Delete /picture/{} Done", uuid);
    tx.commit().await.unwrap();
}
