use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    factory,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct FactoryPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_factory: String,
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
    /// 引用组件id内容:
    #[serde(default)]
    pub ref_id_content: Option<String>,
    /// 组件模块id:
    #[serde(default)]
    pub id_component_module: Option<String>,
    /// 组件模块名称:
    #[serde(default)]
    pub component_module_name: Option<String>,
    /// 组件id:
    #[serde(default)]
    pub id_component: Option<String>,
    /// 组件名称:
    #[serde(default)]
    pub component_name: Option<String>,
    /// 模板标志:
    #[serde(default)]
    pub fg_template: Option<bool>,
}
