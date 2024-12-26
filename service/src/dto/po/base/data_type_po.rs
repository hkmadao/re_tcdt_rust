use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    data_type,
    // project,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct DataTypePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_data_type: String,
    /// 数据类型编码:
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 备注:
    #[serde(default)]
    pub note: Option<String>,
    /// 序列号:
    #[serde(default)]
    pub sn: Option<i32>,
    /// 长度:
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度:
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 字段类型:
    #[serde(default)]
    pub column_type: Option<String>,
    /// 对象类型名称:
    #[serde(default)]
    pub object_type: Option<String>,
    /// 对象类型包名:
    #[serde(default)]
    pub object_type_package: Option<String>,
    /// 扩展属性1:
    #[serde(default)]
    pub ext1: Option<String>,
    /// 扩展属性2:
    #[serde(default)]
    pub ext2: Option<String>,
    /// 扩展属性3:
    #[serde(default)]
    pub ext3: Option<String>,
    /// 扩展属性4:
    #[serde(default)]
    pub ext4: Option<String>,
    /// 扩展属性5:
    #[serde(default)]
    pub ext5: Option<String>,
    /// 扩展属性6:
    #[serde(default)]
    pub ext6: Option<String>,
    /// 默认值:
    #[serde(default)]
    pub default_value: Option<String>,
    /// 必填标志:
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// TypeScript类型:
    #[serde(default)]
    pub type_script_type: Option<String>,
    /// HTML5输入框类型:
    #[serde(default)]
    pub web_input_type: Option<String>,
    /// 系统预置数据标识:
    #[serde(default)]
    pub fg_preset: Option<bool>,
    #[serde(default)]
    pub id_project: Option<String>,
}
