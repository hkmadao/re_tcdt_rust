use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sys_role_menu")]
pub struct Model {
    #[sea_orm(primary_key, comment = "角色与菜单")]
    pub id_role_menu: String,
    /// 系统菜单id:
    #[sea_orm(comment = "系统菜单id")]
    pub id_menu: Option<String>,
    /// 角色id:
    #[sea_orm(comment = "角色id")]
    pub id_role: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// MenuLinked
pub struct MenuLinked;
impl Linked for MenuLinked {
    type FromEntity = Entity;

    type ToEntity = super::menu::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::menu::Entity)
            .from(Column::IdMenu)
            .to(super::menu::Column::IdMenu)
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