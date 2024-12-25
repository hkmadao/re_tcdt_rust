use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_enum_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "枚举属性")]
    pub id_enum_attribute: String,
    /// 枚举属性显示名称
    #[sea_orm(comment = "枚举属性显示名称")]
    pub display_name: Option<String>,
    /// 枚举属性编码
    #[sea_orm(comment = "枚举属性编码")]
    pub code: Option<String>,
    /// 枚举值
    #[sea_orm(comment = "枚举值")]
    pub enum_value: Option<String>,
    /// 序号
    #[sea_orm(comment = "序号")]
    pub sn: Option<i32>,
    /// 枚举id:
    #[sea_orm(comment = "枚举id")]
    pub id_enum: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// DdEnumLinked
pub struct DdEnumLinked;
impl Linked for DdEnumLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_enum::Entity)
            .from(Column::IdEnum)
            .to(super::dd_enum::Column::IdEnum)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}