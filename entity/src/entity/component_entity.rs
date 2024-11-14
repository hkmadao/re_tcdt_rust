use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_component_entity")]
pub struct Model {
    #[sea_orm(primary_key, comment = "组件实体")]
    pub id_component_entity: String,
    /// 虚拟实体标志
    #[sea_orm(comment = "虚拟实体标志")]
    pub fg_virtual: Option<bool>,
    /// 实体id:
    #[sea_orm(comment = "实体id")]
    pub id_entity: Option<String>,
    /// 组件id:
    #[sea_orm(comment = "组件id")]
    pub id_component: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ComponentLinked
pub struct ComponentLinked;
impl Linked for ComponentLinked {
    type FromEntity = Entity;

    type ToEntity = super::component::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component::Entity)
            .from(Column::IdComponent)
            .to(super::component::Column::IdComponent)
            .into()]
    }
}
/// DdEntityLinked
pub struct DdEntityLinked;
impl Linked for DdEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdEntity)
            .to(super::dd_entity::Column::IdEntity)
            .into()]
    }
}
/// ComputationAttributesLinked
pub struct ComputationAttributesLinked;
impl Linked for ComputationAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::computation_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::computation_attribute::Entity)
            .from(Column::IdComponentEntity)
            .to(super::computation_attribute::Column::IdComponentEntity)
            .into()]
    }
}
/// ExtAttributesLinked
pub struct ExtAttributesLinked;
impl Linked for ExtAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::ext_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::ext_attribute::Entity)
            .from(Column::IdComponentEntity)
            .to(super::ext_attribute::Column::IdComponentEntity)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}