use serde::{Deserialize, Serialize};
/// 描述性信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionInfo {
    #[serde(default)]
    pub id: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 字段名称
    #[serde(default)]
    pub column_name: Option<String>,
    /// 是否主键
    #[serde(default)]
    pub fg_primary_key: Option<bool>,
    /// 是否必填
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 数据长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 分类
    #[serde(default)]
    pub category: Option<String>,
    /// 扩展属性1
    #[serde(default)]
    pub ext1: Option<String>,
    /// 扩展属性2
    #[serde(default)]
    pub ext2: Option<String>,
    /// 扩展属性3
    #[serde(default)]
    pub ext3: Option<String>,
    /// 扩展属性4
    #[serde(default)]
    pub ext4: Option<String>,
    /// 扩展属性5
    #[serde(default)]
    pub ext5: Option<String>,
    /// 扩展属性6
    #[serde(default)]
    pub ext6: Option<String>,
    /// 字段类型
    #[serde(default)]
    pub column_type: Option<String>,
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
    /// 计算属性标志
    #[serde(default)]
    pub fg_computation: Option<bool>,
    /// 是否同一个组件下的实体
    #[serde(default)]
    pub fg_partner: Option<bool>,
    // /// 引用类型
    // #[serde(default)]
    // pub ref_type: Option<String>,
    /// 前端输入框类型
    #[serde(default)]
    pub web_input_type: Option<String>,
    /// 前端TypeScript数据类型
    #[serde(default)]
    pub type_script_type: Option<String>,
    /// 全属性名称
    #[serde(default)]
    pub full_attribute_name: Option<String>,
    /// 属性类型id
    #[serde(default)]
    pub id_attribute_type: Option<String>,
    /// 属性类型编码
    #[serde(default)]
    pub attribute_type_code: Option<String>,
    /// 属性类型显示名称
    #[serde(default)]
    pub attribute_type_display_name: Option<String>,
    /// 类型所在包
    #[serde(default)]
    pub object_type_package: Option<String>,
    /// 代码数据类型
    #[serde(default)]
    pub object_type: Option<String>,
    /// 实体信息
    #[serde(default)]
    pub entity_info: Option<EntityInfo>,
    /// 枚举信息
    #[serde(default)]
    pub enum_info: Option<EnumInfo>,
    /// 外部关联属性信息（子实体的外键属性）
    #[serde(default)]
    pub outer_fk_info: Option<Box<DescriptionInfo>>,
    /// 外部关联属性信息（比如子属性关联的父属性或父实体关联的子属性）
    #[serde(default)]
    pub outer_info: Option<Box<DescriptionInfo>>,
    /// 内部关联属性信息（引用属性关联的外键属性信息或者外键属性关联的引用属性信息）
    #[serde(default)]
    pub inner_info: Option<Box<DescriptionInfo>>,
    /// 子描述性信息
    #[serde(default)]
    pub children: Vec<DescriptionInfo>,
}
/// 单实体组件信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfo {
    #[serde(default)]
    pub id_component: String,
    /// 基础路径
    #[serde(default)]
    pub base_path: Option<String>,
    /// 代码包名
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
}
/// 实体信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityInfo {
    #[serde(default)]
    pub id_component_entity: String,
    /// 实体id
    #[serde(default)]
    pub id_entity: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 类名
    #[serde(default)]
    pub class_name: Option<String>,
    /// 表名
    #[serde(default)]
    pub table_name: Option<String>,
    /// 主属性code
    #[serde(default)]
    pub pk_attribute_code: Option<String>,
    /// 主属性名称
    #[serde(default)]
    pub pk_attribute_name: Option<String>,
    /// 主属性类型名称
    #[serde(default)]
    pub pk_attribute_type_name: Option<String>,
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
    /// 基础路径
    #[serde(default)]
    pub base_path: Option<String>,
    /// 代码包名
    #[serde(default)]
    pub package_name: Option<String>,
    /// 组件信息
    #[serde(default)]
    pub component: Option<ComponentInfo>,
    /// 属性信息
    #[serde(default)]
    pub attributes: Vec<DescriptionInfo>,
    /// 主属性信息
    #[serde(default)]
    pub pk_attribute_info: Option<Box<DescriptionInfo>>,
}
/// 枚举属性信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnumAttributeInfo {
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
pub struct EnumInfo {
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
    pub attributes: Vec<EnumAttributeInfo>,
}
