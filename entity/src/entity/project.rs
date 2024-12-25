use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_project")]
pub struct Model {
    #[sea_orm(primary_key, comment = "项目")]
    pub id_project: String,
    /// 项目编号:
    #[sea_orm(comment = "项目编号")]
    pub code: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 系统路径:
    #[sea_orm(comment = "系统路径")]
    pub path: Option<String>,
    /// 项目模板编号:
    #[sea_orm(comment = "项目模板编号")]
    pub template_code: Option<String>,
    /// 备注:
    #[sea_orm(comment = "备注")]
    pub note: Option<String>,
    /// 文件名样式:
    #[sea_orm(comment = "文件名样式")]
    pub file_name_type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// SubProjectsLinked
pub struct SubProjectsLinked;
impl Linked for SubProjectsLinked {
    type FromEntity = Entity;

    type ToEntity = super::sub_project::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::sub_project::Entity)
            .from(Column::IdProject)
            .to(super::sub_project::Column::IdProject)
            .into()]
    }
}
/// CommonAttributesLinked
pub struct CommonAttributesLinked;
impl Linked for CommonAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::common_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::common_attribute::Entity)
            .from(Column::IdProject)
            .to(super::common_attribute::Column::IdProject)
            .into()]
    }
}
/// DataTypesLinked
pub struct DataTypesLinked;
impl Linked for DataTypesLinked {
    type FromEntity = Entity;

    type ToEntity = super::data_type::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::data_type::Entity)
            .from(Column::IdProject)
            .to(super::data_type::Column::IdProject)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}