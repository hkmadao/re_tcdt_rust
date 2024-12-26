use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_component_module")]
pub struct Model {
    #[sea_orm(primary_key, comment = "组件模块id")]
    pub id_component_module: String,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 组件模块空间路径:
    #[sea_orm(comment = "组件模块空间路径")]
    pub path: Option<String>,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub name: Option<String>,
    /// 子项目id:
    #[sea_orm(comment = "子项目id")]
    pub id_sub_project: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// SubProjectLinked
pub struct SubProjectLinked;
impl Linked for SubProjectLinked {
    type FromEntity = Entity;

    type ToEntity = super::sub_project::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::sub_project::Entity)
            .from(Column::IdSubProject)
            .to(super::sub_project::Column::IdSubProject)
            .into()]
    }
}
/// ComponentsLinked
pub struct ComponentsLinked;
impl Linked for ComponentsLinked {
    type FromEntity = Entity;

    type ToEntity = super::component::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component::Entity)
            .from(Column::IdComponentModule)
            .to(super::component::Column::IdComponentModule)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(component_module_model: Model) -> ActiveModel {
    ActiveModel {
        id_component_module: Set(component_module_model.id_component_module.clone()),
        display_name: Set(component_module_model.display_name.clone()),
        path: Set(component_module_model.path.clone()),
        name: Set(component_module_model.name.clone()),
        id_sub_project: Set(component_module_model.id_sub_project.clone()),
    }
}