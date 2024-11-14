use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    user,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct UserVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
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