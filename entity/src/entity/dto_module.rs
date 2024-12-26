use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dto_module")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO模块id")]
    pub id_dto_module: String,
    /// DTO模块名称:
    #[sea_orm(comment = "DTO模块名称")]
    pub name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// DTO模块空间路径:
    #[sea_orm(comment = "DTO模块空间路径")]
    pub path: Option<String>,
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
/// DeCollectionsLinked
pub struct DeCollectionsLinked;
impl Linked for DeCollectionsLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_collection::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_collection::Entity)
            .from(Column::IdDtoModule)
            .to(super::dto_entity_collection::Column::IdDtoModule)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(dto_module_model: Model) -> ActiveModel {
    ActiveModel {
        id_dto_module: Set(dto_module_model.id_dto_module.clone()),
        name: Set(dto_module_model.name.clone()),
        display_name: Set(dto_module_model.display_name.clone()),
        path: Set(dto_module_model.path.clone()),
        id_sub_project: Set(dto_module_model.id_sub_project.clone()),
    }
}