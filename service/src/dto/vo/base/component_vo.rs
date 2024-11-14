use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    component,
    component_module,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub component_module: Option<ComponentModuleVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentModuleVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
}