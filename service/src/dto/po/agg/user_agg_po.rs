use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::common::aq_const::*;
use ::entity::entity::{user, user_role};
use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtSaveParamObjectTrait;
use tcdt_macro::ParamOjectSave;

/// 系统用户
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[tcdt_po(mod_name = "user")]
#[serde(rename_all = "camelCase")]
pub struct UserAggPO {
    #[tcdt_po(ignore)]
    pub action: i32,
    #[serde(default)]
    #[tcdt_po(po_primary_key)]
    pub id_user: String,
    /// 登录账号 
    #[serde(default)]
    pub account: Option<String>,
    /// 用户密码 
    #[serde(default)]
    pub user_pwd: Option<String>,
    /// 手机号码
    #[serde(default)]
    pub phone: Option<String>,
    /// 邮箱
    #[serde(default)]
    pub email: Option<String>,
    /// 姓名 
    #[serde(default)]
    pub name: Option<String>,
    /// 昵称
    #[serde(default)]
    pub nick_name: Option<String>,
    /// 性别
    #[serde(default)]
    pub gender: Option<String>,
    /// 启用标志
    #[serde(default)]
    pub fg_active: Option<bool>,
    /// 用户角色关系
    #[serde(default)]
    #[tcdt_po(po_children)]
    pub user_roles: Vec<UserRoleAggPO>,
}
/// 用户角色关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[tcdt_po(mod_name = "user_role")]
#[serde(rename_all = "camelCase")]
pub struct UserRoleAggPO {
    #[tcdt_po(ignore)]
    pub action: i32,
    #[serde(default)]
    #[tcdt_po(po_primary_key)]
    pub id_sys_user_role: String,
    /// 角色id
    #[serde(default)]
    pub id_role: Option<String>,
    /// 系统用户id
    #[serde(default)]
    #[tcdt_po(po_parent_key)]
    pub id_user: Option<String>,
}

