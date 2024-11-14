use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    dd_entity,
    entity_collection,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity: String,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 类名:
    #[serde(default)]
    pub class_name: Option<String>,
    /// 表名:
    #[serde(default)]
    pub table_name: Option<String>,
    /// 主属性code:
    #[serde(default)]
    pub pk_attribute_code: Option<String>,
    /// 主属性名称:
    #[serde(default)]
    pub pk_attribute_name: Option<String>,
    /// 主属性类型名称:
    #[serde(default)]
    pub pk_attribute_type_name: Option<String>,
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub entity_collection: Option<EntityCollectionVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EntityCollectionVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity_collection: String,
    /// 代码包名:
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
}