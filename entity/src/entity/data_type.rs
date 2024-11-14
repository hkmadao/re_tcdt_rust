use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_data_type")]
pub struct Model {
    #[sea_orm(primary_key, comment = "数据类型")]
    pub id_data_type: String,
    /// 数据类型编码:
    #[sea_orm(comment = "数据类型编码")]
    pub code: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 备注:
    #[sea_orm(comment = "备注")]
    pub note: Option<String>,
    /// 序列号:
    #[sea_orm(comment = "序列号")]
    pub sn: Option<i32>,
    /// 长度:
    #[sea_orm(comment = "长度")]
    pub len: Option<i32>,
    /// 精度:
    #[sea_orm(comment = "精度")]
    pub pcs: Option<i32>,
    /// 字段类型:
    #[sea_orm(comment = "字段类型")]
    pub column_type: Option<String>,
    /// 对象类型名称:
    #[sea_orm(comment = "对象类型名称")]
    pub object_type: Option<String>,
    /// 对象类型包名:
    #[sea_orm(comment = "对象类型包名")]
    pub object_type_package: Option<String>,
    /// 扩展属性1:
    #[sea_orm(comment = "扩展属性1")]
    pub ext1: Option<String>,
    /// 扩展属性2:
    #[sea_orm(comment = "扩展属性2")]
    pub ext2: Option<String>,
    /// 扩展属性3:
    #[sea_orm(comment = "扩展属性3")]
    pub ext3: Option<String>,
    /// 扩展属性4:
    #[sea_orm(comment = "扩展属性4")]
    pub ext4: Option<String>,
    /// 扩展属性5:
    #[sea_orm(comment = "扩展属性5")]
    pub ext5: Option<String>,
    /// 扩展属性6:
    #[sea_orm(comment = "扩展属性6")]
    pub ext6: Option<String>,
    /// 默认值:
    #[sea_orm(comment = "默认值")]
    pub default_value: Option<String>,
    /// 必填标志:
    #[sea_orm(comment = "必填标志")]
    pub fg_mandatory: Option<bool>,
    /// TypeScript类型:
    #[sea_orm(comment = "TypeScript类型")]
    pub type_script_type: Option<String>,
    /// HTML5输入框类型:
    #[sea_orm(comment = "HTML5输入框类型")]
    pub web_input_type: Option<String>,
    /// 系统预置数据标识:
    #[sea_orm(comment = "系统预置数据标识")]
    pub fg_preset: bool,
    /// 项目id:
    #[sea_orm(comment = "项目id")]
    pub id_project: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
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
/// DtoEntityAttributesLinked
pub struct DtoEntityAttributesLinked;
impl Linked for DtoEntityAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_attribute::Entity)
            .from(Column::IdDataType)
            .to(super::dto_entity_attribute::Column::IdAttributeType)
            .into()]
    }
}
/// AttributesLinked
pub struct AttributesLinked;
impl Linked for AttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_attribute::Entity)
            .from(Column::IdDataType)
            .to(super::entity_attribute::Column::IdAttributeType)
            .into()]
    }
}
/// DtoComputationAttributesLinked
pub struct DtoComputationAttributesLinked;
impl Linked for DtoComputationAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_computation_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_computation_attribute::Entity)
            .from(Column::IdDataType)
            .to(super::dto_computation_attribute::Column::IdAttributeType)
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
            .from(Column::IdDataType)
            .to(super::common_attribute::Column::IdDataType)
            .into()]
    }
}
/// ComputationAttributesLinked
pub struct ComputationAttributesLinked;
impl Linked for ComputationAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::computation_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::computation_attribute::Entity)
            .from(Column::IdDataType)
            .to(super::computation_attribute::Column::IdAttributeType)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}