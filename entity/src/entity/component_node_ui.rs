use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_component_node_ui")]
pub struct Model {
    #[sea_orm(primary_key, comment = "ui信息")]
    pub id_component_node_ui: String,
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

impl ActiveModelBehavior for ActiveModel {}