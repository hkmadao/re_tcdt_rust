use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_entity_collection")]
pub struct Model {
    #[sea_orm(primary_key, comment = "实体集")]
    pub id_entity_collection: String,
    /// 代码包名:
    #[sea_orm(comment = "代码包名")]
    pub package_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 子项目id:
    #[sea_orm(comment = "子项目id")]
    pub id_sub_project: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// SubProjectLinked
pub struct SubProjectLinked;
impl Linked for SubProjectLinked {
    type FromEntity = Entity;

    type ToEntity = super::sub_project::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::sub_project::Entity)
            .from(Column::IdSubProject)
            .to(super::sub_project::Column::IdSubProject)
            .into()]
    }
}
/// EntityAssociatesLinked
pub struct EntityAssociatesLinked;
impl Linked for EntityAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_associate::Entity)
            .from(Column::IdEntityCollection)
            .to(super::entity_associate::Column::IdEntityCollection)
            .into()]
    }
}
/// NodeUisLinked
pub struct NodeUisLinked;
impl Linked for NodeUisLinked {
    type FromEntity = Entity;

    type ToEntity = super::node_ui::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::node_ui::Entity)
            .from(Column::IdEntityCollection)
            .to(super::node_ui::Column::IdEntityCollection)
            .into()]
    }
}
/// EnumAssociatesLinked
pub struct EnumAssociatesLinked;
impl Linked for EnumAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::enum_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::enum_associate::Entity)
            .from(Column::IdEntityCollection)
            .to(super::enum_associate::Column::IdEntityCollection)
            .into()]
    }
}
/// EnumsLinked
pub struct EnumsLinked;
impl Linked for EnumsLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_enum::Entity)
            .from(Column::IdEntityCollection)
            .to(super::dd_enum::Column::IdEntityCollection)
            .into()]
    }
}
/// EntitiesLinked
pub struct EntitiesLinked;
impl Linked for EntitiesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdEntityCollection)
            .to(super::dd_entity::Column::IdEntityCollection)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}