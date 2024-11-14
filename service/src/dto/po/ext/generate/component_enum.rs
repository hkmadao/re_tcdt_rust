use serde::{Deserialize, Serialize};

/// 组件信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfoPO {
    #[serde(default)]
    pub id_component: String,
    /// 调试用参数
    #[serde(default)]
    pub param_json: Option<String>,
    /// 名称
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 基础路径
    #[serde(default)]
    pub base_path: Option<String>,
    /// 枚举信息
    #[serde(default)]
    pub enum_info_list: Vec<EnumInfoPO>,
}
/// 枚举属性信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnumAttributeInfoPO {
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
}
/// 枚举信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnumInfoPO {
    /// 调试用参数
    #[serde(default)]
    pub param_json: Option<String>,
    #[serde(default)]
    pub id_enum: String,
    /// 基础路径
    #[serde(default)]
    pub base_path: Option<String>,
    /// 模块名称
    #[serde(default)]
    pub package_name: Option<String>,
    /// 名称
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型
    #[serde(default)]
    pub enum_value_type: Option<String>,
    /// 属性驼峰名称（驼峰命名法）
    #[serde(default)]
    pub camel_case_name: Option<String>,
    /// 属性大写驼峰名称（帕斯卡命名）
    #[serde(default)]
    pub pascal_case_name: Option<String>,
    /// 属性大写下划线名称（蛇式命名法）
    #[serde(default)]
    pub snake_case_name: Option<String>,
    /// 属性大写下划线名称（宏命名法）
    #[serde(default)]
    pub macro_case_name: Option<String>,
    /// 枚举属性信息
    #[serde(default)]
    pub enum_attribute_info_list: Vec<EnumAttributeInfoPO>,
    /// 组件信息
    #[serde(default)]
    pub comp_info: Option<Box<ComponentInfoPO>>,
}
