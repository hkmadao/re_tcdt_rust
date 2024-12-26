use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_enum_associate")]
pub struct Model {
    #[sea_orm(primary_key, comment = "枚举关系id")]
    pub id_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号:
    #[sea_orm(comment = "两个相同实体和枚举多条连线时，连线的序号")]
    pub group_order: Option<i32>,
    /// 属性id:
    #[sea_orm(comment = "属性id")]
    pub id_attribute: Option<String>,
    /// 实体集id:
    #[sea_orm(comment = "实体集id")]
    pub id_entity_collection: Option<String>,
    /// 枚举id:
    #[sea_orm(comment = "枚举id")]
    pub id_enum: Option<String>,
    /// 实体id:
    #[sea_orm(comment = "实体id")]
    pub id_entity: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// AttributeLinked
pub struct AttributeLinked;
impl Linked for AttributeLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_attribute::Entity)
            .from(Column::IdAttribute)
            .to(super::entity_attribute::Column::IdAttribute)
            .into()]
    }
}
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
/// DdEnumLinked
pub struct DdEnumLinked;
impl Linked for DdEnumLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_enum::Entity)
            .from(Column::IdEnum)
            .to(super::dd_enum::Column::IdEnum)
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

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(enum_associate_model: Model) -> ActiveModel {
    ActiveModel {
        id_enum_associate: Set(enum_associate_model.id_enum_associate.clone()),
        group_order: Set(enum_associate_model.group_order.clone()),
        id_attribute: Set(enum_associate_model.id_attribute.clone()),
        id_entity_collection: Set(enum_associate_model.id_entity_collection.clone()),
        id_enum: Set(enum_associate_model.id_enum.clone()),
        id_entity: Set(enum_associate_model.id_entity.clone()),
    }
}