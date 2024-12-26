use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_node_ui")]
pub struct Model {
    #[sea_orm(primary_key, comment = "id")]
    pub id_node_ui: String,
    /// x坐标:
    #[sea_orm(comment = "x坐标")]
    pub x: Option<i32>,
    /// y坐标:
    #[sea_orm(comment = "y坐标")]
    pub y: Option<i32>,
    /// 宽度:
    #[sea_orm(comment = "宽度")]
    pub width: Option<i32>,
    /// 高度:
    #[sea_orm(comment = "高度")]
    pub height: Option<i32>,
    /// 元素id:
    #[sea_orm(comment = "元素id")]
    pub id_element: Option<String>,
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

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(node_ui_model: Model) -> ActiveModel {
    ActiveModel {
        id_node_ui: Set(node_ui_model.id_node_ui.clone()),
        x: Set(node_ui_model.x.clone()),
        y: Set(node_ui_model.y.clone()),
        width: Set(node_ui_model.width.clone()),
        height: Set(node_ui_model.height.clone()),
        id_element: Set(node_ui_model.id_element.clone()),
        id_entity_collection: Set(node_ui_model.id_entity_collection.clone()),
    }
}