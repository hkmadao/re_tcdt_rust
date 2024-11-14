use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_common_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "公共属性")]
    pub id_common_attribute: String,
    /// 属性名称:
    #[sea_orm(comment = "属性名称")]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 字段名称:
    #[sea_orm(comment = "字段名称")]
    pub column_name: Option<String>,
    /// 默认值:
    #[sea_orm(comment = "默认值")]
    pub default_value: Option<String>,
    /// 是否必填:
    #[sea_orm(comment = "是否必填")]
    pub fg_mandatory: Option<bool>,
    /// 数据长度:
    #[sea_orm(comment = "数据长度")]
    pub len: Option<i32>,
    /// 精度:
    #[sea_orm(comment = "精度")]
    pub pcs: Option<i32>,
    /// 序号:
    #[sea_orm(comment = "序号")]
    pub sn: Option<i32>,
    /// 引用属性名称:
    #[sea_orm(comment = "引用属性名称")]
    pub ref_attribute_name: Option<String>,
    /// 引用属性显示名称:
    #[sea_orm(comment = "引用属性显示名称")]
    pub ref_display_name: Option<String>,
    /// 属性类别:
    #[sea_orm(comment = "属性类别")]
    pub category: Option<String>,
    /// 系统预置数据标识:
    #[sea_orm(comment = "系统预置数据标识")]
    pub fg_preset: bool,
    /// 上级实体id:
    #[sea_orm(comment = "上级实体id")]
    pub id_ref_entity: Option<String>,
    /// 数据类型id:
    #[sea_orm(comment = "数据类型id")]
    pub id_data_type: Option<String>,
    /// 项目id:
    #[sea_orm(comment = "项目id")]
    pub id_project: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// RefEntityLinked
pub struct RefEntityLinked;
impl Linked for RefEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdRefEntity)
            .to(super::dd_entity::Column::IdEntity)
            .into()]
    }
}
/// DataTypeLinked
pub struct DataTypeLinked;
impl Linked for DataTypeLinked {
    type FromEntity = Entity;

    type ToEntity = super::data_type::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::data_type::Entity)
            .from(Column::IdDataType)
            .to(super::data_type::Column::IdDataType)
            .into()]
    }
}
/// ProjectLinked
pub struct ProjectLinked;
impl Linked for ProjectLinked {
    type FromEntity = Entity;

    type ToEntity = super::project::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::project::Entity)
            .from(Column::IdProject)
            .to(super::project::Column::IdProject)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}