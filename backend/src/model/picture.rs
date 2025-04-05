use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
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

/* TODO: 插入/更新/删除前后处理文件与检查？ */
impl ActiveModelBehavior for ActiveModel {}
