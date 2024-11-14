use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dto_enum_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO枚举属性")]
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