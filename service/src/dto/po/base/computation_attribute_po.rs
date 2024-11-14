use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    computation_attribute,
    // component_entity,
    // data_type,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ComputationAttributePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_computation_attribute: String,
    /// 属性名称:
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 数据长度:
    #[serde(default)]
    pub len: Option<i32>,
    /// 是否必填:
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值:
    #[serde(default)]
    pub default_value: Option<String>,
    /// 精度:
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号:
    #[serde(default)]
    pub sn: Option<i32>,
    #[serde(default)]
    pub id_component_entity: Option<String>,
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
