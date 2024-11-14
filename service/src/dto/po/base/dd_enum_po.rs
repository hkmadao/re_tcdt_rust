use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    dd_enum,
    // entity_collection,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct DdEnumPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_enum: String,
    /// 名称:
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型:
    #[serde(default)]
    pub enum_value_type: Option<String>,
    #[serde(default)]
    pub id_entity_collection: Option<String>,
}
