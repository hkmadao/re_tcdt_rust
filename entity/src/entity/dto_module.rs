use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dto_module")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO模块")]
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