use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dto_computation_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = " DTO计算属性id")]
    pub id_dto_computation_attribute: String,
    /// 属性名称:
    #[sea_orm(comment = "属性名称")]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 备注:
    #[sea_orm(comment = "备注")]
    pub note: Option<String>,
    /// 数据长度:
    #[sea_orm(comment = "数据长度")]
    pub len: Option<i32>,
    /// 是否必填:
    #[sea_orm(comment = "是否必填")]
    pub fg_mandatory: Option<bool>,
    /// 默认值:
    #[sea_orm(comment = "默认值")]
    pub default_value: Option<String>,
    /// 精度:
    #[sea_orm(comment = "精度")]
    pub pcs: Option<String>,
    /// 序号:
    #[sea_orm(comment = "序号")]
    pub sn: Option<String>,
    ///  DTO实体信息id:
    #[sea_orm(comment = " DTO实体信息id")]
    pub id_dto_entity: Option<String>,
    /// 数据类型id:
    #[sea_orm(comment = "数据类型id")]
    pub id_attribute_type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// DtoEntityLinked
pub struct DtoEntityLinked;
impl Linked for DtoEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity::Entity)
            .from(Column::IdDtoEntity)
            .to(super::dto_entity::Column::IdDtoEntity)
            .into()]
    }
}
/// AttributeTypeLinked
pub struct AttributeTypeLinked;
impl Linked for AttributeTypeLinked {
    type FromEntity = Entity;

    type ToEntity = super::data_type::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::data_type::Entity)
            .from(Column::IdAttributeType)
            .to(super::data_type::Column::IdDataType)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(dto_computation_attribute_model: Model) -> ActiveModel {
    ActiveModel {
        id_dto_computation_attribute: Set(dto_computation_attribute_model.id_dto_computation_attribute.clone()),
        attribute_name: Set(dto_computation_attribute_model.attribute_name.clone()),
        display_name: Set(dto_computation_attribute_model.display_name.clone()),
        note: Set(dto_computation_attribute_model.note.clone()),
        len: Set(dto_computation_attribute_model.len.clone()),
        fg_mandatory: Set(dto_computation_attribute_model.fg_mandatory.clone()),
        default_value: Set(dto_computation_attribute_model.default_value.clone()),
        pcs: Set(dto_computation_attribute_model.pcs.clone()),
        sn: Set(dto_computation_attribute_model.sn.clone()),
        id_dto_entity: Set(dto_computation_attribute_model.id_dto_entity.clone()),
        id_attribute_type: Set(dto_computation_attribute_model.id_attribute_type.clone()),
    }
}