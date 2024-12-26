use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key, comment = "系统菜单id")]
    pub id_menu: String,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 显示标志:
    #[sea_orm(comment = "显示标志")]
    pub fg_show: Option<bool>,
    /// 路由参数:
    #[sea_orm(comment = "路由参数")]
    pub query: Option<String>,
    /// 菜单类型:
    #[sea_orm(comment = "菜单类型")]
    pub menu_type: Option<String>,
    /// 启用标志:
    #[sea_orm(comment = "启用标志")]
    pub fg_active: Option<bool>,
    /// 前端权限标识:
    #[sea_orm(comment = "前端权限标识")]
    pub web_perms: Option<String>,
    /// 后台权限标识:
    #[sea_orm(comment = "后台权限标识")]
    pub service_perms: Option<String>,
    /// 上级系统菜单id:
    #[sea_orm(comment = "上级系统菜单id")]
    pub id_parent: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ParentLinked
pub struct ParentLinked;
impl Linked for ParentLinked {
    type FromEntity = Entity;

    type ToEntity = super::menu::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::menu::Entity)
            .from(Column::IdParent)
            .to(super::menu::Column::IdMenu)
            .into()]
    }
}
/// RoleMenusLinked
pub struct RoleMenusLinked;
impl Linked for RoleMenusLinked {
    type FromEntity = Entity;

    type ToEntity = super::role_menu::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::role_menu::Entity)
            .from(Column::IdMenu)
            .to(super::role_menu::Column::IdMenu)
            .into()]
    }
}
/// ChildrenLinked
pub struct ChildrenLinked;
impl Linked for ChildrenLinked {
    type FromEntity = Entity;

    type ToEntity = super::menu::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::menu::Entity)
            .from(Column::IdMenu)
            .to(super::menu::Column::IdParent)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(menu_model: Model) -> ActiveModel {
    ActiveModel {
        id_menu: Set(menu_model.id_menu.clone()),
        name: Set(menu_model.name.clone()),
        display_name: Set(menu_model.display_name.clone()),
        fg_show: Set(menu_model.fg_show.clone()),
        query: Set(menu_model.query.clone()),
        menu_type: Set(menu_model.menu_type.clone()),
        fg_active: Set(menu_model.fg_active.clone()),
        web_perms: Set(menu_model.web_perms.clone()),
        service_perms: Set(menu_model.service_perms.clone()),
        id_parent: Set(menu_model.id_parent.clone()),
    }
}