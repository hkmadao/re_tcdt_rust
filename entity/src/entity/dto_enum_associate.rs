use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dto_enum_associate")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO枚举关系id")]
    pub id_dto_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号:
    #[sea_orm(comment = "两个相同实体和枚举多条连线时，连线的序号")]
    pub group_order: Option<i32>,
    /// DTO枚举id:
    #[sea_orm(comment = "DTO枚举id")]
    pub id_dto_enum: Option<String>,
    /// DTO实体集id:
    #[sea_orm(comment = "DTO实体集id")]
    pub id_dto_entity_collection: Option<String>,
    ///  DTO实体信息id:
    #[sea_orm(comment = " DTO实体信息id")]
    pub id_dto_entity: Option<String>,
    /// DTO实体属性id:
    #[sea_orm(comment = "DTO实体属性id")]
    pub id_dto_entity_attribute: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// DtoEnumLinked
pub struct DtoEnumLinked;
impl Linked for DtoEnumLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum::Entity)
            .from(Column::IdDtoEnum)
            .to(super::dto_enum::Column::IdDtoEnum)
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
/// DtoEntityLinked
pub struct DtoEntityLinked;
impl Linked for DtoEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_entity::Column::IdDtoEntity)
            .into()]
    }
}
/// DtoEntityAttributeLinked
pub struct DtoEntityAttributeLinked;
impl Linked for DtoEntityAttributeLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_attribute::Entity)
            .from(Column::IdDtoEntityAttribute)
            .to(super::dto_entity_attribute::Column::IdDtoEntityAttribute)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(dto_enum_associate_model: Model) -> ActiveModel {
    ActiveModel {
        id_dto_enum_associate: Set(dto_enum_associate_model.id_dto_enum_associate.clone()),
        group_order: Set(dto_enum_associate_model.group_order.clone()),
        id_dto_enum: Set(dto_enum_associate_model.id_dto_enum.clone()),
        id_dto_entity_collection: Set(dto_enum_associate_model.id_dto_entity_collection.clone()),
        id_dto_entity: Set(dto_enum_associate_model.id_dto_entity.clone()),
        id_dto_entity_attribute: Set(dto_enum_associate_model.id_dto_entity_attribute.clone()),
    }
}