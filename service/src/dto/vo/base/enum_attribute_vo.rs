use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    enum_attribute,
    dd_enum,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EnumAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum_attribute: String,
    /// 枚举属性显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举属性编码
    #[serde(default)]
    pub code: Option<String>,
    /// 枚举值
    #[serde(default)]
    pub enum_value: Option<String>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    #[serde(default)]
    pub id_enum: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_enum: Option<DdEnumVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum: String,
    /// 名称:
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型:
    #[serde(default)]
    pub enum_value_type: Option<String>,
}