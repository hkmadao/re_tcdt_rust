use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_data_type")]
pub struct Model {
    #[sea_orm(primary_key, comment = "数据类型id")]
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
    pub fg_preset: Option<bool>,
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

pub fn convert_model_to_active_model(data_type_model: Model) -> ActiveModel {
    ActiveModel {
        id_data_type: Set(data_type_model.id_data_type.clone()),
        code: Set(data_type_model.code.clone()),
        display_name: Set(data_type_model.display_name.clone()),
        note: Set(data_type_model.note.clone()),
        sn: Set(data_type_model.sn.clone()),
        len: Set(data_type_model.len.clone()),
        pcs: Set(data_type_model.pcs.clone()),
        column_type: Set(data_type_model.column_type.clone()),
        object_type: Set(data_type_model.object_type.clone()),
        object_type_package: Set(data_type_model.object_type_package.clone()),
        ext1: Set(data_type_model.ext1.clone()),
        ext2: Set(data_type_model.ext2.clone()),
        ext3: Set(data_type_model.ext3.clone()),
        ext4: Set(data_type_model.ext4.clone()),
        ext5: Set(data_type_model.ext5.clone()),
        ext6: Set(data_type_model.ext6.clone()),
        default_value: Set(data_type_model.default_value.clone()),
        fg_mandatory: Set(data_type_model.fg_mandatory.clone()),
        type_script_type: Set(data_type_model.type_script_type.clone()),
        web_input_type: Set(data_type_model.web_input_type.clone()),
        fg_preset: Set(data_type_model.fg_preset.clone()),
        id_project: Set(data_type_model.id_project.clone()),
    }
}