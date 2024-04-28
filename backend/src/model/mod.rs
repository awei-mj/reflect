use rbatis::{impl_select, rbdc::{Bytes, DateTime, Uuid}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Picture {
    pub uuid: Uuid,
    pub name: String,
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub upload_time: DateTime,
    pub content: Bytes
}
rbatis::crud!(Picture {}, "pics");
impl_select!(Picture {select_by_id(uuid: &Uuid) -> Option => "`where uuid = #{uuid} limit 1`"}, "pics");
