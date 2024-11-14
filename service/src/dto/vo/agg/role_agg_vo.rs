use ::entity::entity::{
    role,
    role_menu,
    user_role,
    menu,
    user,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use sea_orm::prelude::Json;
use sea_orm::ModelTrait;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::{DbConn, DbErr, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct RoleVO {
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_role: String,
    /// 名称:
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    #[tcdt_vo(vo_array)]
    #[serde(default)]
    pub role_menus: Vec<RoleMenuVO>,
    #[tcdt_vo(vo_array)]
    #[serde(default)]
    pub user_roles: Vec<UserRoleVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct RoleMenuVO {
    #[tcdt_vo(po_primary_key)]
    #[serde(default)]
    pub id_role_menu: String,
    #[serde(default)]
    pub id_role: Option<String>,
    #[serde(default)]
    pub id_menu: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub menu: Option<MenuVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleVO {
    #[tcdt_vo(po_primary_key)]
    #[serde(default)]
    pub id_sys_user_role: String,
    #[serde(default)]
    pub id_role: Option<String>,
    #[serde(default)]
    pub id_user: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub user: Option<UserVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct MenuVO {
    #[tcdt_vo(vo_primary_key)]
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
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct UserVO {
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_user: String,
    /// 登录账号 :
    #[serde(default)]
    pub account: Option<String>,
    /// 用户密码 :
    #[serde(default)]
    pub user_pwd: Option<String>,
    /// 手机号码:
    #[serde(default)]
    pub phone: Option<String>,
    /// 邮箱:
    #[serde(default)]
    pub email: Option<String>,
    /// 姓名 :
    #[serde(default)]
    pub name: Option<String>,
    /// 昵称:
    #[serde(default)]
    pub nick_name: Option<String>,
    /// 性别:
    #[serde(default)]
    pub gender: Option<String>,
    /// 启用标志:
    #[serde(default)]
    pub fg_active: Option<bool>,
}
