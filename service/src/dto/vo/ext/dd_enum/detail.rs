use ::entity::entity::{dd_enum, enum_attribute};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum: String,
    /// 名称
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型
    #[serde(default)]
    pub enum_value_type: Option<String>,
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub attributes: Vec<EnumAttributeVO>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EnumAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum_attribute: String,
    /// 枚举属性显示名称:枚举属性显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举属性编码:枚举属性编码
    #[serde(default)]
    pub code: Option<String>,
    /// 枚举值:枚举值
    #[serde(default)]
    pub enum_value: Option<String>,
    /// 序号:序号
    #[serde(default)]
    pub sn: Option<i32>,
    #[serde(default)]
    pub id_enum: Option<String>,
}
