use std::io::Write;

use crate::{error::AppError, model::picture, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{prelude::Uuid, EntityTrait};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/* TODO: 检查文件格式？ */
fn save_picture(file_path: &str, data: &Vec<u8>) -> Result<(), AppError> {
    let dir = file_path.split("/").take(file_path.split("/").count() - 1).collect::<Vec<&str>>().join("/");
    if !std::path::Path::new(&dir).exists() {
        std::fs::create_dir_all(dir)?;
    }
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(data.as_slice())?;

    info!("[reflect] Saved picture to {}", file_path);

    Ok(())
}

fn delete_picture(file_path: &str) -> Result<(), AppError> {
    std::fs::remove_file(file_path)?;

    info!("[reflect] Deleted picture from {}", file_path);

    Ok(())
}

pub async fn upload_picture(
    State(state): State<AppState>,
    Json(mut picture): Json<picture::Picture>,
) -> Result<(), AppError> {
    info!("[reflect] Post /picture Handling");

    let data: Vec<u8> = picture.data.ok_or(axum::Error::new("Data is None"))?;
    picture.data = None;

    let pic = picture::ActiveModel::from(picture);

    let file_path = format!("{}/{}", state.img_path, pic.file_path.clone().take().ok_or(axum::Error::new("File path is None"))?.to_string());

    save_picture(&file_path, &data)?;

    let res = picture::Entity::insert(pic).exec(&state.db).await?;

    info!("[reflect] Post /picture Done, id: {}", res.last_insert_id);

    Ok(())
}

pub async fn upload_pictures(
    State(state): State<AppState>,
    Json(mut pictures): Json<Vec<picture::Picture>>,
) -> Result<(), AppError> {
    let mut data: Vec<Vec<u8>> = Vec::new();
    info!("[reflect] Post /pictures Handling");

    for pic in pictures.iter_mut() {
        let data_temp = pic.data.take().ok_or(axum::Error::new("Data is None"))?;
        data.push(data_temp);
    }
    let pics = pictures.into_iter().map(|pic| picture::ActiveModel::from(pic)).collect::<Vec<_>>();

    for (i, pic) in pics.iter().enumerate() {
        let file_path = format!("{}/{}", state.img_path, pic.file_path.clone().take().ok_or(axum::Error::new("File path is None"))?);
        save_picture(&file_path, &data[i])?;
    }
    
    let res = picture::Entity::insert_many(pics).exec(&state.db).await?;
    info!("[reflect] Post /pictures Done, last_insert_id: {}", res.last_insert_id);

    Ok(())
}

pub async fn get_picture_by_uuid(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<picture::Picture>>, AppError> {
    info!("[reflect] Get /picture/{} Handling", id);
    let pic = picture::Entity::find_by_id(id).one(&state.db).await?;
    info!("[reflect] Get /picture/{} Done", id);
    match pic {
        Some(pic) => Ok(Json(Some(picture::Picture::from(pic)))),
        None => Ok(Json(None)),
    }
}

/* TODO: 分页、排序 */
pub async fn get_pictures(
    State(state): State<AppState>,
) -> Result<Json<Vec<picture::Picture>>, AppError> {
    info!("[reflect] Get /pictures Handling");
    let pics = picture::Entity::find().all(&state.db).await?;
    info!("[reflect] Get /pictures Done");
    Ok(Json(
        pics.into_iter()
            .map(|pic| picture::Picture::from(pic))
            .collect(),
    ))
}

pub async fn delete_picture_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<(), AppError> {
    info!("[reflect] Delete /picture/{} Handling", uuid);
    let pic = picture::Entity::find_by_id(uuid).one(&state.db).await?;
    match pic {
        Some(pic) => {
            let file_path = format!("{}/{}", state.img_path, pic.file_path);
            delete_picture(&file_path)?;
            let res = picture::Entity::delete_by_id(uuid).exec(&state.db).await?;
            info!("[reflect] Delete /picture/{} Done, rows_affected: {}", uuid, res.rows_affected);
            Ok(())
        },
        None => {
            warn!("[reflect] Delete /picture/{} Not Found", uuid);
            Err(AppError::from(axum::Error::new(format!("Picture uuid: {} Not Found", uuid))))
        }
    }
}

/* 验证TOTP码 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TotpVerifyRequest {
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TotpVerifyResponse {
    pub valid: bool,
}

pub async fn verify_totp(
    State(state): State<AppState>,
    Json(req): Json<TotpVerifyRequest>,
) -> Result<Json<TotpVerifyResponse>, AppError> {
    use totp_rs::TOTP;
    info!("[reflect] Post /totp/verify Handling");

    // 创建TOTP实例
    let result = TOTP::from_url(state.totp_url)?.check_current(&req.code)?;

    info!("[reflect] Post /totp/verify Done");
    Ok(Json(TotpVerifyResponse { valid: result }))
}
