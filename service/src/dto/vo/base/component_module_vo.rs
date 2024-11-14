use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    component_module,
    sub_project,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

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
    #[serde(default)]
    pub id_sub_project: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub sub_project: Option<SubProjectVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct SubProjectVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_sub_project: String,
    /// 名称:
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 子系统路径:
    #[serde(default)]
    pub path: Option<String>,
}