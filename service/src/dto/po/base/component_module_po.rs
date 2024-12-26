use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    component_module,
    // sub_project,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ComponentModulePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_component_module: String,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 组件模块空间路径:
    #[serde(default)]
    pub path: Option<String>,
    /// 名称:
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub id_sub_project: Option<String>,
}
