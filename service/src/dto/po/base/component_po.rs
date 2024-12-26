use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    component,
    // component_module,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ComponentPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_component: String,
    /// 主实体id:
    #[serde(default)]
    pub id_main_component_entity: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 包名:
    #[serde(default)]
    pub package_name: Option<String>,
    /// 组件类型:
    #[serde(default)]
    pub component_type: Option<String>,
    #[serde(default)]
    pub id_component_module: Option<String>,
}
