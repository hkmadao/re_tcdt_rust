use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    node_ui,
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
pub struct NodeUiVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_node_ui: String,
    /// x坐标:
    #[serde(default)]
    pub x: Option<i32>,
    /// y坐标:
    #[serde(default)]
    pub y: Option<i32>,
    /// 宽度:
    #[serde(default)]
    pub width: Option<i32>,
    /// 高度:
    #[serde(default)]
    pub height: Option<i32>,
    /// 元素id:
    #[serde(default)]
    pub id_element: Option<String>,
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