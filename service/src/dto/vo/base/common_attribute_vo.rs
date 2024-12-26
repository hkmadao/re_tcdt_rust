use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    common_attribute,
    dd_entity,
    data_type,
    project,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct CommonAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_entity: Option<DdEntityVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub data_type: Option<DataTypeVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub project: Option<ProjectVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity: String,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 类名:
    #[serde(default)]
    pub class_name: Option<String>,
    /// 表名:
    #[serde(default)]
    pub table_name: Option<String>,
    /// 主属性code:
    #[serde(default)]
    pub pk_attribute_code: Option<String>,
    /// 主属性名称:
    #[serde(default)]
    pub pk_attribute_name: Option<String>,
    /// 主属性类型名称:
    #[serde(default)]
    pub pk_attribute_type_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DataTypeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_project: String,
    /// 项目编号:
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 系统路径:
    #[serde(default)]
    pub path: Option<String>,
    /// 项目模板编号:
    #[serde(default)]
    pub template_code: Option<String>,
    /// 备注:
    #[serde(default)]
    pub note: Option<String>,
    /// 文件名样式:
    #[serde(default)]
    pub file_name_type: Option<String>,
}