use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    menu,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct MenuPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_menu: String,
    /// 名称:
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 显示标志:
    #[serde(default)]
    pub fg_show: Option<bool>,
    /// 路由参数:
    #[serde(default)]
    pub query: Option<String>,
    /// 菜单类型:
    #[serde(default)]
    pub menu_type: Option<String>,
    /// 启用标志:
    #[serde(default)]
    pub fg_active: Option<bool>,
    /// 前端权限标识:
    #[serde(default)]
    pub web_perms: Option<String>,
    /// 后台权限标识:
    #[serde(default)]
    pub service_perms: Option<String>,
    #[serde(default)]
    pub id_parent: Option<String>,
}