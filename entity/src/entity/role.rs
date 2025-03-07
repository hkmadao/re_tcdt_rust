use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "sys_role")]
pub struct Model {
    #[sea_orm(primary_key, comment = "角色id")]
    pub id_role: String,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// RoleMenusLinked
pub struct RoleMenusLinked;
impl Linked for RoleMenusLinked {
    type FromEntity = Entity;

    type ToEntity = super::role_menu::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::role_menu::Entity)
            .from(Column::IdRole)
            .to(super::role_menu::Column::IdRole)
            .into()]
    }
}
/// UserRolesLinked
pub struct UserRolesLinked;
impl Linked for UserRolesLinked {
    type FromEntity = Entity;

    type ToEntity = super::user_role::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::user_role::Entity)
            .from(Column::IdRole)
            .to(super::user_role::Column::IdRole)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(role_model: Model) -> ActiveModel {
    ActiveModel {
        id_role: Set(role_model.id_role.clone()),
        name: Set(role_model.name.clone()),
        display_name: Set(role_model.display_name.clone()),
    }
}