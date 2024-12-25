use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_component")]
pub struct Model {
    #[sea_orm(primary_key, comment = "组件")]
    pub id_component: String,
    /// 主实体id:
    #[sea_orm(comment = "主实体id")]
    pub id_main_component_entity: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 包名:
    #[sea_orm(comment = "包名")]
    pub package_name: Option<String>,
    /// 组件类型:
    #[sea_orm(comment = "组件类型")]
    pub component_type: Option<String>,
    /// 组件模块id:
    #[sea_orm(comment = "组件模块id")]
    pub id_component_module: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ComponentModuleLinked
pub struct ComponentModuleLinked;
impl Linked for ComponentModuleLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_module::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_module::Entity)
            .from(Column::IdComponentModule)
            .to(super::component_module::Column::IdComponentModule)
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
            .from(Column::IdComponent)
            .to(super::component_enum::Column::IdComponent)
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
            .from(Column::IdComponent)
            .to(super::component_entity::Column::IdComponent)
            .into()]
    }
}
/// ComponentEntityAssociatesLinked
pub struct ComponentEntityAssociatesLinked;
impl Linked for ComponentEntityAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_entity_associate::Entity)
            .from(Column::IdComponent)
            .to(super::component_entity_associate::Column::IdComponent)
            .into()]
    }
}
/// ComponentNodeUisLinked
pub struct ComponentNodeUisLinked;
impl Linked for ComponentNodeUisLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_node_ui::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_node_ui::Entity)
            .from(Column::IdComponent)
            .to(super::component_node_ui::Column::IdComponent)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}