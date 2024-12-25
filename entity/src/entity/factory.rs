use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "ui_factory")]
pub struct Model {
    #[sea_orm(primary_key, comment = "UI工厂")]
    pub id_factory: String,
    /// 配置内容
    #[sea_orm(comment = "配置内容")]
    pub content: Option<String>,
    /// 名称:
    #[sea_orm(comment = "名称")]
    pub name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 项目id:
    #[sea_orm(comment = "项目id")]
    pub id_project: Option<String>,
    /// 项目名称:
    #[sea_orm(comment = "项目名称")]
    pub project_name: Option<String>,
    /// 子项目id:
    #[sea_orm(comment = "子项目id")]
    pub id_sub_project: Option<String>,
    /// 子项目名称:
    #[sea_orm(comment = "子项目名称")]
    pub sub_project_name: Option<String>,
    /// 引用组件id内容:
    #[sea_orm(comment = "引用组件id内容")]
    pub ref_id_content: Option<String>,
    /// 组件模块id:
    #[sea_orm(comment = "组件模块id")]
    pub id_component_module: Option<String>,
    /// 组件模块名称:
    #[sea_orm(comment = "组件模块名称")]
    pub component_module_name: Option<String>,
    /// 组件id:
    #[sea_orm(comment = "组件id")]
    pub id_component: Option<String>,
    /// 组件名称:
    #[sea_orm(comment = "组件名称")]
    pub component_name: Option<String>,
    /// 模板标志:
    #[sea_orm(comment = "模板标志")]
    pub fg_template: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}