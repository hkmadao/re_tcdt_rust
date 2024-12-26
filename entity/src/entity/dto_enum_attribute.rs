use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dto_enum_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO枚举属性id")]
    pub id_dto_enum_attribute: String,
    /// 枚举属性显示名称:
    #[sea_orm(comment = "枚举属性显示名称")]
    pub display_name: Option<String>,
    /// 枚举属性编码:
    #[sea_orm(comment = "枚举属性编码")]
    pub code: Option<String>,
    /// 枚举值:
    #[sea_orm(comment = "枚举值")]
    pub enum_value: Option<String>,
    /// 序号:
    #[sea_orm(comment = "序号")]
    pub sn: Option<i32>,
    /// 引用id:
    #[sea_orm(comment = "引用id")]
    pub id_ref: Option<String>,
    /// DTO枚举id:
    #[sea_orm(comment = "DTO枚举id")]
    pub id_dto_enum: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// DtoEnumLinked
pub struct DtoEnumLinked;
impl Linked for DtoEnumLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum::Entity)
            .from(Column::IdDtoEnum)
            .to(super::dto_enum::Column::IdDtoEnum)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(dto_enum_attribute_model: Model) -> ActiveModel {
    ActiveModel {
        id_dto_enum_attribute: Set(dto_enum_attribute_model.id_dto_enum_attribute.clone()),
        display_name: Set(dto_enum_attribute_model.display_name.clone()),
        code: Set(dto_enum_attribute_model.code.clone()),
        enum_value: Set(dto_enum_attribute_model.enum_value.clone()),
        sn: Set(dto_enum_attribute_model.sn.clone()),
        id_ref: Set(dto_enum_attribute_model.id_ref.clone()),
        id_dto_enum: Set(dto_enum_attribute_model.id_dto_enum.clone()),
    }
}