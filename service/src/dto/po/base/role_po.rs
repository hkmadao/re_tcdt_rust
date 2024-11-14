use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    role,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct RolePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_role: String,
    /// 名称:
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
}
