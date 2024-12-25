use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_entity")]
pub struct Model {
    #[sea_orm(primary_key, comment = "实体信息")]
    pub id_entity: String,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 类名:
    #[sea_orm(comment = "类名")]
    pub class_name: Option<String>,
    /// 表名:
    #[sea_orm(comment = "表名")]
    pub table_name: Option<String>,
    /// 主属性code:
    #[sea_orm(comment = "主属性code")]
    pub pk_attribute_code: Option<String>,
    /// 主属性名称:
    #[sea_orm(comment = "主属性名称")]
    pub pk_attribute_name: Option<String>,
    /// 主属性类型名称:
    #[sea_orm(comment = "主属性类型名称")]
    pub pk_attribute_type_name: Option<String>,
    /// 实体集id:
    #[sea_orm(comment = "实体集id")]
    pub id_entity_collection: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// EntityCollectionLinked
pub struct EntityCollectionLinked;
impl Linked for EntityCollectionLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_collection::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_collection::Entity)
            .from(Column::IdEntityCollection)
            .to(super::entity_collection::Column::IdEntityCollection)
            .into()]
    }
}
/// CommonAttributesLinked
pub struct CommonAttributesLinked;
impl Linked for CommonAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::common_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::common_attribute::Entity)
            .from(Column::IdEntity)
            .to(super::common_attribute::Column::IdRefEntity)
            .into()]
    }
}
/// DtoEntitysLinked
pub struct DtoEntitysLinked;
impl Linked for DtoEntitysLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity::Entity)
            .from(Column::IdEntity)
            .to(super::dto_entity::Column::IdRef)
            .into()]
    }
}
/// DownAssociatesLinked
pub struct DownAssociatesLinked;
impl Linked for DownAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_associate::Entity)
            .from(Column::IdEntity)
            .to(super::entity_associate::Column::IdUp)
            .into()]
    }
}
/// AttributesLinked
pub struct AttributesLinked;
impl Linked for AttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_attribute::Entity)
            .from(Column::IdEntity)
            .to(super::entity_attribute::Column::IdEntity)
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
            .from(Column::IdEntity)
            .to(super::enum_associate::Column::IdEntity)
            .into()]
    }
}
/// UpAssociatesLinked
pub struct UpAssociatesLinked;
impl Linked for UpAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_associate::Entity)
            .from(Column::IdEntity)
            .to(super::entity_associate::Column::IdDown)
            .into()]
    }
}
/// ComponentEntitiesLinked
pub struct ComponentEntitiesLinked;
impl Linked for ComponentEntitiesLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_entity::Entity)
            .from(Column::IdEntity)
            .to(super::component_entity::Column::IdEntity)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}