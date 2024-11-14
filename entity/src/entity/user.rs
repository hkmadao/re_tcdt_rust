use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, comment = "系统用户")]
    pub id_user: String,
    /// 登录账号 :
    #[sea_orm(comment = "登录账号 ")]
    pub account: Option<String>,
    /// 用户密码 :
    #[sea_orm(comment = "用户密码 ")]
    pub user_pwd: Option<String>,
    /// 手机号码:
    #[sea_orm(comment = "手机号码")]
    pub phone: Option<String>,
    /// 邮箱:
    #[sea_orm(comment = "邮箱")]
    pub email: Option<String>,
    /// 姓名 :
    #[sea_orm(comment = "姓名 ")]
    pub name: Option<String>,
    /// 昵称:
    #[sea_orm(comment = "昵称")]
    pub nick_name: Option<String>,
    /// 性别:
    #[sea_orm(comment = "性别")]
    pub gender: Option<String>,
    /// 启用标志:
    #[sea_orm(comment = "启用标志")]
    pub fg_active: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// UserRolesLinked
pub struct UserRolesLinked;
impl Linked for UserRolesLinked {
    type FromEntity = Entity;

    type ToEntity = super::user_role::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::user_role::Entity)
            .from(Column::IdUser)
            .to(super::user_role::Column::IdUser)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}