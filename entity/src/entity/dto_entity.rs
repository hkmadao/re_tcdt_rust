use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dto_entity")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO实体信息")]
    pub id_dto_entity: String,
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
    /// 引用实体id:
    #[sea_orm(comment = "引用实体id")]
    pub id_ref: Option<String>,
    /// DTO实体集id:
    #[sea_orm(comment = "DTO实体集id")]
    pub id_dto_entity_collection: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// RefEntityLinked
pub struct RefEntityLinked;
impl Linked for RefEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdRef)
            .to(super::dd_entity::Column::IdEntity)
            .into()]
    }
}
/// DtoEntityCollectionLinked
pub struct DtoEntityCollectionLinked;
impl Linked for DtoEntityCollectionLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_collection::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_collection::Entity)
            .from(Column::IdDtoEntityCollection)
            .to(super::dto_entity_collection::Column::IdDtoEntityCollection)
            .into()]
    }
}
/// DownAssociatesLinked
pub struct DownAssociatesLinked;
impl Linked for DownAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_associate::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_entity_associate::Column::IdUp)
            .into()]
    }
}
/// DtoEnumAssociatesLinked
pub struct DtoEnumAssociatesLinked;
impl Linked for DtoEnumAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum_associate::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_enum_associate::Column::IdDtoEntity)
            .into()]
    }
}
/// DcAttributesLinked
pub struct DcAttributesLinked;
impl Linked for DcAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_computation_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_computation_attribute::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_computation_attribute::Column::IdDtoEntity)
            .into()]
    }
}
/// UpAssociatesLinked
pub struct UpAssociatesLinked;
impl Linked for UpAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_associate::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_entity_associate::Column::IdDown)
            .into()]
    }
}
/// DeAttributesLinked
pub struct DeAttributesLinked;
impl Linked for DeAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_attribute::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_entity_attribute::Column::IdDtoEntity)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}