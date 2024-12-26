use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_component_enum")]
pub struct Model {
    #[sea_orm(primary_key, comment = "组件枚举id")]
    pub id_component_enum: String,
    /// 枚举id:
    #[sea_orm(comment = "枚举id")]
    pub id_enum: Option<String>,
    /// 组件id:
    #[sea_orm(comment = "组件id")]
    pub id_component: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
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

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(component_enum_model: Model) -> ActiveModel {
    ActiveModel {
        id_component_enum: Set(component_enum_model.id_component_enum.clone()),
        id_enum: Set(component_enum_model.id_enum.clone()),
        id_component: Set(component_enum_model.id_component.clone()),
    }
}