use entity::entity::{user, role,user_role};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use sea_orm::prelude::Json;
use sea_orm::ModelTrait;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::{DbConn, DbErr, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;

/// 系统用户
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct UserVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
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
    #[tcdt_vo(vo_array)]
    pub user_roles: Vec<UserRoleVO>,
}
/// 角色
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct RoleVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_role: String,
    /// 名称
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
}
/// 用户角色关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_sys_user_role: String,
    /// 角色id
    #[serde(default)]
    pub id_role: Option<String>,
    /// 系统用户id
    #[serde(default)]
    pub id_user: Option<String>,
    /// 角色
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub role: Option<RoleVO>,
}

