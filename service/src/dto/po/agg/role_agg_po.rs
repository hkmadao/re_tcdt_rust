use ::entity::entity::{
    role,
    role_menu,
    user_role,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::common::aq_const::*;
use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtSaveParamObjectTrait;
use tcdt_macro::ParamOjectSave;

/// 角色
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[tcdt_po(mod_name = "role")]
#[serde(rename_all = "camelCase")]
pub struct RoleAggPO {
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
    #[serde(default)]
    #[tcdt_po(po_children)]
    pub role_menus: Vec<RoleMenuAggPO>,
    #[serde(default)]
    #[tcdt_po(po_children)]
    pub user_roles: Vec<UserRoleAggPO>,
}
/// 角色与菜单
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[serde(rename_all = "camelCase")]
pub struct RoleMenuAggPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_role_menu: String,
    #[serde(default)]
    pub id_role: Option<String>,
    #[serde(default)]
    pub id_menu: Option<String>,
}
/// 用户角色关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleAggPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_sys_user_role: String,
    #[serde(default)]
    pub id_role: Option<String>,
    #[serde(default)]
    pub id_user: Option<String>,
}
