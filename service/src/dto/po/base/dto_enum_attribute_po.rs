use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    dto_enum_attribute,
    // dto_enum,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAttributePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_dto_enum_attribute: String,
    /// 枚举属性显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举属性编码:
    #[serde(default)]
    pub code: Option<String>,
    /// 枚举值:
    #[serde(default)]
    pub enum_value: Option<String>,
    /// 序号:
    #[serde(default)]
    pub sn: Option<i32>,
    /// 引用id:
    #[serde(default)]
    pub id_ref: Option<String>,
    #[serde(default)]
    pub id_dto_enum: Option<String>,
}
