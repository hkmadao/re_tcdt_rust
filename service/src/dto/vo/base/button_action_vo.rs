use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    button_action,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ButtonActionVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_button_action: String,
    /// 配置内容
    #[serde(default)]
    pub content: Option<String>,
    /// 名称:
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 项目id:
    #[serde(default)]
    pub id_project: Option<String>,
    /// 项目名称:
    #[serde(default)]
    pub project_name: Option<String>,
    /// 子项目id:
    #[serde(default)]
    pub id_sub_project: Option<String>,
    /// 子项目名称:
    #[serde(default)]
    pub sub_project_name: Option<String>,
}