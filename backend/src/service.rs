use std::io::Write;

use crate::{model::picture, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{prelude::Uuid, EntityTrait};
use tracing::info;

/* 检查文件格式？ */
fn save_picture(file_path: &str, data: Vec<u8>) {
    let dir = file_path.split("/").take(file_path.split("/").count() - 1).collect::<Vec<&str>>().join("/");
    if !std::path::Path::new(&dir).exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(data.as_slice()).unwrap();
}

fn delete_picture(file_path: &str) {
    std::fs::remove_file(file_path).unwrap();
}

pub async fn upload_picture(
    State(state): State<AppState>,
    Json(mut picture): Json<picture::Picture>,
) -> () {
    info!("[reflect] Post /picture Handling");

    let data: Vec<u8> = picture.data.unwrap();
    picture.data = None;

    let pic = picture::ActiveModel::from(picture);
    let file_path = format!("{}/{}", state.img_path, pic.file_path.clone().unwrap());

    save_picture(&file_path, data);

    let res = picture::Entity::insert(pic).exec(&state.db).await.unwrap();

    info!("[reflect] Post /picture Done, id: {}", res.last_insert_id);
}

pub async fn upload_pictures(
    State(state): State<AppState>,
    Json(mut pictures): Json<Vec<picture::Picture>>,
) -> () {
    info!("[reflect] Post /pictures Handling");

    let data: Vec<Vec<u8>> = pictures.iter_mut().map(|pic| pic.data.clone().unwrap()).collect();
    pictures.iter_mut().for_each(|pic| pic.data = None);
    let pics = pictures.into_iter().map(|pic| picture::ActiveModel::from(pic)).collect::<Vec<_>>();

    data.into_iter().zip(pics.iter()).for_each(|(data, pic)| {
        let file_path = format!("{}/{}", state.img_path, pic.file_path.clone().unwrap());
        save_picture(&file_path, data);
    });
    
    let res = picture::Entity::insert_many(pics).exec(&state.db).await.unwrap();
    info!("[reflect] Post /pictures Done, ids: {}", res.last_insert_id);
}

pub async fn get_picture_by_uuid(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<Option<picture::Picture>> {
    info!("[reflect] Get /picture/{} Handling", id);
    let pic = picture::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .unwrap();
    info!("[reflect] Get /picture/{} Done", id);
    match pic {
        Some(pic) => Json(Some(picture::Picture::from(pic))),
        None => Json(None),
    }
}

/* TODO: 分页、排序 */
pub async fn get_pictures(
    State(state): State<AppState>,
) -> Json<Vec<picture::Picture>> {
    info!("[reflect] Get /pictures Handling");
    let pics = picture::Entity::find().all(&state.db).await.unwrap();
    info!("[reflect] Get /pictures Done");
    Json(
        pics.into_iter()
            .map(|pic| picture::Picture::from(pic))
            .collect(),
    )
}

pub async fn delete_picture_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> () {
    info!("[reflect] Delete /picture/{} Handling", uuid);
    if let Some(pic) = picture::Entity::find_by_id(uuid).one(&state.db).await.unwrap() {
        let file_path = format!("{}/{}", state.img_path, pic.file_path);
        delete_picture(&file_path);
        let res = picture::Entity::delete_by_id(uuid).exec(&state.db).await.unwrap();
        info!("[reflect] Delete /picture/{} Done, rows_affected: {}", uuid, res.rows_affected);
    } else {
        info!("[reflect] Delete /picture/{} Not Found", uuid);
    }
}
