use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    project,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_project: String,
    /// 项目编号:
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 系统路径:
    #[serde(default)]
    pub path: Option<String>,
    /// 项目模板编号:
    #[serde(default)]
    pub template_code: Option<String>,
    /// 备注:
    #[serde(default)]
    pub note: Option<String>,
    /// 文件名样式:
    #[serde(default)]
    pub file_name_type: Option<String>,
}
