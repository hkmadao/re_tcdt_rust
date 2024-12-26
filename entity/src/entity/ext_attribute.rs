use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_ext_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "扩展属性id")]
    pub id_ext_attribute: String,
    /// 扩展字段1
    #[sea_orm(comment = "扩展字段1")]
    pub ext1: Option<String>,
    /// 排序:
    #[sea_orm(comment = "排序")]
    pub sn: Option<i32>,
    /// 组件实体id:
    #[sea_orm(comment = "组件实体id")]
    pub id_component_entity: Option<String>,
    /// 属性id:
    #[sea_orm(comment = "属性id")]
    pub id_attribute: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ComponentEntityLinked
pub struct ComponentEntityLinked;
impl Linked for ComponentEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_entity::Entity)
            .from(Column::IdComponentEntity)
            .to(super::component_entity::Column::IdComponentEntity)
            .into()]
    }
}
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

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(ext_attribute_model: Model) -> ActiveModel {
    ActiveModel {
        id_ext_attribute: Set(ext_attribute_model.id_ext_attribute.clone()),
        ext1: Set(ext_attribute_model.ext1.clone()),
        sn: Set(ext_attribute_model.sn.clone()),
        id_component_entity: Set(ext_attribute_model.id_component_entity.clone()),
        id_attribute: Set(ext_attribute_model.id_attribute.clone()),
    }
}