use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sys_user_role")]
pub struct Model {
    #[sea_orm(primary_key, comment = "用户角色关系")]
    pub id_sys_user_role: String,
    /// 系统用户id:
    #[sea_orm(comment = "系统用户id")]
    pub id_user: Option<String>,
    /// 角色id:
    #[sea_orm(comment = "角色id")]
    pub id_role: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// UserLinked
pub struct UserLinked;
impl Linked for UserLinked {
    type FromEntity = Entity;

    type ToEntity = super::user::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::user::Entity)
            .from(Column::IdUser)
            .to(super::user::Column::IdUser)
            .into()]
    }
}
/// RoleLinked
pub struct RoleLinked;
impl Linked for RoleLinked {
    type FromEntity = Entity;

    type ToEntity = super::role::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::role::Entity)
            .from(Column::IdRole)
            .to(super::role::Column::IdRole)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}