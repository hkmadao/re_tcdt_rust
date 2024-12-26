use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dto_entity_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO实体属性id")]
    pub id_dto_entity_attribute: String,
    /// 属性名称:
    #[sea_orm(comment = "属性名称")]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 字段名称:
    #[sea_orm(comment = "字段名称")]
    pub column_name: Option<String>,
    /// 是否主键:
    #[sea_orm(comment = "是否主键")]
    pub fg_primary_key: Option<bool>,
    /// 是否必填:
    #[sea_orm(comment = "是否必填")]
    pub fg_mandatory: Option<bool>,
    /// 默认值:
    #[sea_orm(comment = "默认值")]
    pub default_value: Option<String>,
    /// 数据长度:
    #[sea_orm(comment = "数据长度")]
    pub len: Option<i32>,
    /// 精度:
    #[sea_orm(comment = "精度")]
    pub pcs: Option<i32>,
    /// 序号:
    #[sea_orm(comment = "序号")]
    pub sn: Option<i32>,
    /// 备注:
    #[sea_orm(comment = "备注")]
    pub note: Option<String>,
    /// 类型:
    #[sea_orm(comment = "类型")]
    pub category: Option<String>,
    /// 数据类型id:
    #[sea_orm(comment = "数据类型id")]
    pub id_attribute_type: Option<String>,
    /// 引用属性id:
    #[sea_orm(comment = "引用属性id")]
    pub id_ref_attribute: Option<String>,
    ///  DTO实体信息id:
    #[sea_orm(comment = " DTO实体信息id")]
    pub id_dto_entity: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
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
/// RefAttributeLinked
pub struct RefAttributeLinked;
impl Linked for RefAttributeLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_attribute::Entity)
            .from(Column::IdRefAttribute)
            .to(super::entity_attribute::Column::IdAttribute)
            .into()]
    }
}
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
/// DtoEnumAssociatesLinked
pub struct DtoEnumAssociatesLinked;
impl Linked for DtoEnumAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum_associate::Entity)
            .from(Column::IdDtoEntityAttribute)
            .to(super::dto_enum_associate::Column::IdDtoEntityAttribute)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(dto_entity_attribute_model: Model) -> ActiveModel {
    ActiveModel {
        id_dto_entity_attribute: Set(dto_entity_attribute_model.id_dto_entity_attribute.clone()),
        attribute_name: Set(dto_entity_attribute_model.attribute_name.clone()),
        display_name: Set(dto_entity_attribute_model.display_name.clone()),
        column_name: Set(dto_entity_attribute_model.column_name.clone()),
        fg_primary_key: Set(dto_entity_attribute_model.fg_primary_key.clone()),
        fg_mandatory: Set(dto_entity_attribute_model.fg_mandatory.clone()),
        default_value: Set(dto_entity_attribute_model.default_value.clone()),
        len: Set(dto_entity_attribute_model.len.clone()),
        pcs: Set(dto_entity_attribute_model.pcs.clone()),
        sn: Set(dto_entity_attribute_model.sn.clone()),
        note: Set(dto_entity_attribute_model.note.clone()),
        category: Set(dto_entity_attribute_model.category.clone()),
        id_attribute_type: Set(dto_entity_attribute_model.id_attribute_type.clone()),
        id_ref_attribute: Set(dto_entity_attribute_model.id_ref_attribute.clone()),
        id_dto_entity: Set(dto_entity_attribute_model.id_dto_entity.clone()),
    }
}