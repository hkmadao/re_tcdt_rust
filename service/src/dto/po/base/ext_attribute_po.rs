use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    ext_attribute,
    // component_entity,
    // entity_attribute,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ExtAttributePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_ext_attribute: String,
    /// 扩展字段1
    #[serde(default)]
    pub ext1: Option<String>,
    /// 排序:
    #[serde(default)]
    pub sn: Option<i32>,
    #[serde(default)]
    pub id_component_entity: Option<String>,
    #[serde(default)]
    pub id_attribute: Option<String>,
}
