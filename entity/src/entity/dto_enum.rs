use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dto_enum")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO枚举实体")]
    pub id_dto_enum: String,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub class_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 枚举值的类型:
    #[sea_orm(comment = "枚举值的类型")]
    pub enum_value_type: Option<String>,
    /// 枚举id:
    #[sea_orm(comment = "枚举id")]
    pub id_ref: Option<String>,
    /// DTO实体集id:
    #[sea_orm(comment = "DTO实体集id")]
    pub id_dto_entity_collection: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// RefEnumLinked
pub struct RefEnumLinked;
impl Linked for RefEnumLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_enum::Entity)
            .from(Column::IdRef)
            .to(super::dd_enum::Column::IdEnum)
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
/// DtoEnumAttributesLinked
pub struct DtoEnumAttributesLinked;
impl Linked for DtoEnumAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum_attribute::Entity)
            .from(Column::IdDtoEnum)
            .to(super::dto_enum_attribute::Column::IdDtoEnum)
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
            .from(Column::IdDtoEnum)
            .to(super::dto_enum_associate::Column::IdDtoEnum)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}