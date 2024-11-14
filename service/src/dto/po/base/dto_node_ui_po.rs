use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    dto_node_ui,
    // dto_entity_collection,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct DtoNodeUiPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_dto_node_ui: String,
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
    pub id_dto_entity_collection: Option<String>,
}
