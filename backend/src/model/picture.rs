use chrono::Datelike;
use sea_orm::{entity::prelude::*, sqlx::types::chrono::Local, ActiveValue::Set};
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Picture {
    pub id: Option<Uuid>,
    pub name: String,
    pub file_size: u32,
    pub width: u16,
    pub height: u16,
    pub data: Option<Vec<u8>>,
    pub path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub name: String,
    pub file_size: u32,
    pub width: u16,
    pub height: u16,
    #[sea_orm(indexed)]
    pub upload_time: DateTime,
    pub file_path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Picture {
    fn from(model: Model) -> Self {
        Picture {
            id: Some(model.id),
            name: model.name,
            file_size: model.file_size,
            width: model.width,
            height: model.height,
            data: None,
            path: Some(model.file_path),
        }
    }
}

impl From<Picture> for ActiveModel {
    fn from(picture: Picture) -> Self {
        let now = Local::now().naive_local();
        let uuid = match picture.id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        let name = picture.name.split(".").last().unwrap_or("jpg");

        let file_path = format!("{}/{}/{}.{}", now.year(), now.month(), uuid.to_string(), name.to_string());

        ActiveModel {
            id: Set(uuid),
            name: Set(picture.name.clone()),
            file_size: Set(picture.file_size),
            width: Set(picture.width),
            height: Set(picture.height),
            upload_time: Set(now.clone()),
            file_path: Set(file_path),
        }
    }
}
