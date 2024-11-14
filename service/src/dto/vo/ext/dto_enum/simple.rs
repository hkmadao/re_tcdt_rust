use ::entity::entity::dto_enum;
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_enum: String,
    /// 名称
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型
    #[serde(default)]
    pub enum_value_type: Option<String>,
    #[serde(default)]
    pub id_ref: Option<String>,
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
}
