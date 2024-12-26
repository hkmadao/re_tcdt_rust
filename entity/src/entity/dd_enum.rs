use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_enum")]
pub struct Model {
    #[sea_orm(primary_key, comment = "枚举id")]
    pub id_enum: String,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub class_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 枚举值的类型:
    #[sea_orm(comment = "枚举值的类型")]
    pub enum_value_type: Option<String>,
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
/// EnumAssociatesLinked
pub struct EnumAssociatesLinked;
impl Linked for EnumAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::enum_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::enum_associate::Entity)
            .from(Column::IdEnum)
            .to(super::enum_associate::Column::IdEnum)
            .into()]
    }
}
/// ComponentEnumsLinked
pub struct ComponentEnumsLinked;
impl Linked for ComponentEnumsLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_enum::Entity)
            .from(Column::IdEnum)
            .to(super::component_enum::Column::IdEnum)
            .into()]
    }
}
/// AttributesLinked
pub struct AttributesLinked;
impl Linked for AttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::enum_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::enum_attribute::Entity)
            .from(Column::IdEnum)
            .to(super::enum_attribute::Column::IdEnum)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(dd_enum_model: Model) -> ActiveModel {
    ActiveModel {
        id_enum: Set(dd_enum_model.id_enum.clone()),
        class_name: Set(dd_enum_model.class_name.clone()),
        display_name: Set(dd_enum_model.display_name.clone()),
        enum_value_type: Set(dd_enum_model.enum_value_type.clone()),
        id_entity_collection: Set(dd_enum_model.id_entity_collection.clone()),
    }
}