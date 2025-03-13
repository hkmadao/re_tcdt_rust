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
    /// 直接引用的外部实体
    #[serde(default)]
    pub up_entity_info_list: Vec<EntityInfoPO>,
    /// 下级直接引用的外部实体
    #[serde(default)]
    pub down_entity_info_list: Vec<EntityInfoPO>,
    /// 主实体信息
    #[serde(default)]
    pub main_entity_info: EntityInfoPO,
    /// 基础数据引用包
    #[serde(default)]
    pub out_base_package_list: Vec<BasePackageInfo>,
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
    pub enum_attribute_info_list: Vec<EnumAttributeInfoPO>,
}
/// 属性信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributeInfoPO {
    #[serde(default)]
    pub id_ext_attribute: String,
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
    /// 属性类型编码
    #[serde(default)]
    pub attribute_type_code: Option<String>,
    /// 字段类型
    #[serde(default)]
    pub column_type: Option<String>,
    /// 代码数据类型
    #[serde(default)]
    pub object_type: Option<String>,
    /// 类型所在包
    #[serde(default)]
    pub object_type_package: Option<String>,
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
    /// 外部关联属性信息（比如子属性关联的父属性或父实体关联的子属性）
    #[serde(default)]
    pub outer_info: Option<Box<AttributeInfoPO>>,
    /// 属性关联实体信息
    #[serde(default)]
    pub out_entity_info: Option<Box<EntityInfoPO>>,
    /// 外部关联属性信息（子实体的外键属性）
    #[serde(default)]
    pub outer_fk_info: Option<Box<AttributeInfoPO>>,
    /// 内部关联属性信息（引用属性关联的外键属性信息或者外键属性关联的引用属性信息）
    #[serde(default)]
    pub inner_info: Option<Box<AttributeInfoPO>>,
    /// 枚举信息
    #[serde(default)]
    pub enum_info: Option<EnumInfoPO>,
    /// 数据类型id:数据类型id
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/// 实体信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityInfoPO {
    #[serde(default)]
    pub id_component_entity: String,
    #[serde(default)]
    pub id_entity: String,
    /// 主实体标志
    #[serde(default)]
    pub fg_main: bool,
    /// 主实体基础信息
    #[serde(default)]
    pub main_entity_info: Option<Box<EntityInfoPO>>,
    /// 调试用参数
    #[serde(default)]
    pub param_json: Option<String>,
    /// 基础路径
    #[serde(default)]
    pub base_path: Option<String>,
    /// 模块名称
    #[serde(default)]
    pub package_name: Option<String>,
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
    /// 组件信息
    #[serde(default)]
    pub comp_info: Option<Box<ComponentInfoPO>>,
    /// 外键属性信息
    #[serde(default)]
    pub fk_attribute_info_list: Vec<AttributeInfoPO>,
    /// 所有属性信息
    #[serde(default)]
    pub attribute_info_list: Vec<AttributeInfoPO>,
    /// 主键属性描述
    #[serde(default)]
    pub pk_attribute_info: Option<AttributeInfoPO>,
    /// 下级Single属性信息
    #[serde(default)]
    pub down_single_attribute_info_list: Vec<AttributeInfoPO>,
    /// 下级直接引用的外部实体
    #[serde(default)]
    pub down_entity_info_list: Vec<EntityInfoPO>,
    /// 引用Single类型属性信息
    #[serde(default)]
    pub up_single_attribute_info_list: Vec<AttributeInfoPO>,
    /// 引用类型属性信息
    #[serde(default)]
    pub up_attribute_info_list: Vec<AttributeInfoPO>,
    /// 上级直接引用的外部实体
    #[serde(default)]
    pub up_entity_info_list: Vec<EntityInfoPO>,
    /// 子实体
    #[serde(default)]
    pub child_entity_info_list: Vec<EntityInfoPO>,
    /// 下级属性信息
    #[serde(default)]
    pub down_attribute_info_list: Vec<AttributeInfoPO>,
    /// 基本类型属性信息（不包含主属性）
    #[serde(default)]
    pub base_attribute_info_list: Vec<AttributeInfoPO>,
    /// 基础数据引用包
    #[serde(default)]
    pub out_base_package_list: Vec<BasePackageInfo>,
}

/// 基础包信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BasePackageInfo {
    /// 基础路径
    #[serde(default)]
    pub object_type_package: String,
    /// 类型名称
    #[serde(default)]
    pub object_type: String,
}