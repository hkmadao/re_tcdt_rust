use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_sub_project")]
pub struct Model {
    #[sea_orm(primary_key, comment = "子项目")]
    pub id_sub_project: String,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 子系统路径:
    #[sea_orm(comment = "子系统路径")]
    pub path: Option<String>,
    /// 项目id:
    #[sea_orm(comment = "项目id")]
    pub id_project: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ProjectLinked
pub struct ProjectLinked;
impl Linked for ProjectLinked {
    type FromEntity = Entity;

    type ToEntity = super::project::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::project::Entity)
            .from(Column::IdProject)
            .to(super::project::Column::IdProject)
            .into()]
    }
}
/// ComponentModulesLinked
pub struct ComponentModulesLinked;
impl Linked for ComponentModulesLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_module::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_module::Entity)
            .from(Column::IdSubProject)
            .to(super::component_module::Column::IdSubProject)
            .into()]
    }
}
/// DtoModulesLinked
pub struct DtoModulesLinked;
impl Linked for DtoModulesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_module::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_module::Entity)
            .from(Column::IdSubProject)
            .to(super::dto_module::Column::IdSubProject)
            .into()]
    }
}
/// EntityCollectionsLinked
pub struct EntityCollectionsLinked;
impl Linked for EntityCollectionsLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_collection::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_collection::Entity)
            .from(Column::IdSubProject)
            .to(super::entity_collection::Column::IdSubProject)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}