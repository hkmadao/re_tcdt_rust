use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "ui_button_action")]
pub struct Model {
    #[sea_orm(primary_key, comment = "按钮操作")]
    pub id_button_action: String,
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}