use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    common_attribute,
    // dd_entity,
    // data_type,
    // project,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct CommonAttributePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_common_attribute: String,
    /// 属性名称:
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 字段名称:
    #[serde(default)]
    pub column_name: Option<String>,
    /// 默认值:
    #[serde(default)]
    pub default_value: Option<String>,
    /// 是否必填:
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 数据长度:
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度:
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号:
    #[serde(default)]
    pub sn: Option<i32>,
    /// 引用属性名称:
    #[serde(default)]
    pub ref_attribute_name: Option<String>,
    /// 引用属性显示名称:
    #[serde(default)]
    pub ref_display_name: Option<String>,
    /// 属性类别:
    #[serde(default)]
    pub category: Option<String>,
    /// 系统预置数据标识:
    #[serde(default)]
    pub fg_preset: Option<bool>,
    #[serde(default)]
    pub id_ref_entity: Option<String>,
    #[serde(default)]
    pub id_data_type: Option<String>,
    #[serde(default)]
    pub id_project: Option<String>,
}
