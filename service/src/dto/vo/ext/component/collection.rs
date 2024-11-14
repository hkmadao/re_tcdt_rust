use ::entity::entity::{
    component, component_entity, component_entity_associate, component_enum, component_module,
    component_node_ui, computation_attribute, data_type, dd_entity, dd_enum, entity_associate,
    entity_attribute, enum_attribute, ext_attribute, sub_project,
};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;
/// 实体信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity: String,
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
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    /// 实体属性
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub attributes: Vec<EntityAttributeVO>,
}
/// 枚举实体
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
    /// 枚举属性
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub attributes: Vec<EnumAttributeVO>,
}
/// 组件实体属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ExtAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_ext_attribute: String,
    /// 扩展字段1
    #[serde(default)]
    pub ext1: Option<String>,
    /// 排序
    #[serde(default)]
    pub sn: Option<i32>,
    /// 组件实体id
    #[serde(default)]
    pub id_component_entity: Option<String>,
    /// 属性id
    #[serde(default)]
    pub id_attribute: Option<String>,
    /// 属性
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub attribute: Option<EntityAttributeVO>,
}
/// 计算属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComputationAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_computation_attribute: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 数据长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 是否必填
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 组件实体id
    #[serde(default)]
    pub id_component_entity: Option<String>,
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/// 枚举属性
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
    /// 枚举id
    #[serde(default)]
    pub id_enum: Option<String>,
}
/// 属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EntityAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_attribute: String,
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
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/// 组件模块
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentModuleVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_module: String,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 组件模块空间路径
    #[serde(default)]
    pub path: Option<String>,
    /// 名称
    #[serde(default)]
    pub name: Option<String>,
    /// 子项目id
    #[serde(default)]
    pub id_sub_project: Option<String>,
    /// 子项目
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub sub_project: Option<SubProjectVO>,
}
/// 组件
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component: String,
    /// 主实体id
    #[serde(default)]
    pub id_main_component_entity: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 包名
    #[serde(default)]
    pub package_name: Option<String>,
    /// 组件类型
    #[serde(default)]
    pub component_type: Option<String>,
    /// 组件模块id
    #[serde(default)]
    pub id_component_module: Option<String>,
    /// 组件模块
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub component_module: Option<ComponentModuleVO>,
    /// ui信息
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub component_node_uis: Vec<ComponentNodeUiVO>,
    /// 系统数据类型
    #[serde(default)]
    #[tcdt_vo(vo_array, ignore)]
    pub sys_data_types: Vec<DataTypeVO>,
    /// 组件关系
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub component_entity_associates: Vec<ComponentEntityAssociateVO>,
    /// 组件枚举
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub component_enums: Vec<ComponentEnumVO>,
    /// 组件实体
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub component_entities: Vec<ComponentEntityVO>,
}
/// 组件枚举
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_enum: String,
    /// 枚举id
    #[serde(default)]
    pub id_enum: Option<String>,
    /// 枚举实体
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_enum: Option<DdEnumVO>,
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
}
/// 数据类型
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DataTypeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_data_type: String,
    /// 数据类型编码
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 序列号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 字段类型
    #[serde(default)]
    pub column_type: Option<String>,
    /// 对象类型名称
    #[serde(default)]
    pub object_type: Option<String>,
    /// 对象类型包名
    #[serde(default)]
    pub object_type_package: Option<String>,
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
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 必填标志
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// TypeScript类型
    #[serde(default)]
    pub type_script_type: Option<String>,
    /// HTML5输入框类型
    #[serde(default)]
    pub web_input_type: Option<String>,
}
/// 组件关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_entity_associate: String,
    /// 下级实体包名
    #[serde(default)]
    pub down_package_name: Option<String>,
    /// 上级实体包名
    #[serde(default)]
    pub up_package_name: Option<String>,
    /// 是否agg关系连线
    #[serde(default)]
    pub fg_agg_asso: Option<bool>,
    /// 实体连线id
    #[serde(default)]
    pub id_entity_associate: Option<String>,
    /// 关系连线
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub entity_associate: Option<EntityAssociateVO>,
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
}
/// 子项目
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct SubProjectVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_sub_project: String,
    /// 名称
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 子系统路径
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub id_project: Option<String>,
}
/// 关系连线
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EntityAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity_associate: String,
    /// 两个实体多条连线时，连线的序号
    #[serde(default)]
    pub group_order: Option<i32>,
    /// 上级关系
    #[serde(default)]
    pub up_associate_type: Option<String>,
    /// 下级关系
    #[serde(default)]
    pub down_associate_type: Option<String>,
    /// 下级实体属性名称
    #[serde(default)]
    pub down_attribute_name: Option<String>,
    /// 下级实体属性显示名称
    #[serde(default)]
    pub down_attribute_display_name: Option<String>,
    /// 引用实体属性
    #[serde(default)]
    pub ref_attribute_name: Option<String>,
    /// 引用实体属性显示名称
    #[serde(default)]
    pub ref_attribute_display_name: Option<String>,
    /// 外键字段名称
    #[serde(default)]
    pub fk_column_name: Option<String>,
    /// 外键属性
    #[serde(default)]
    pub fk_attribute_name: Option<String>,
    /// 外键属性显示名称
    #[serde(default)]
    pub fk_attribute_display_name: Option<String>,
    /// 是否建立物理外键
    #[serde(default)]
    pub fg_foreign_key: Option<bool>,
    /// 下级实体排序
    #[serde(default)]
    pub down_order_str: Option<String>,
    /// 批量获取下级实体数量
    #[serde(default)]
    pub down_batch_size: Option<i32>,
    /// 父实体id
    #[serde(default)]
    pub id_up: Option<String>,
    /// 父实体
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub up_entity: Option<AssoRefEntityVO>,
    /// 子实体id
    #[serde(default)]
    pub id_down: Option<String>,
    /// 子实体
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub down_entity: Option<AssoRefEntityVO>,
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
}
/// 组件实体
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_entity: String,
    /// 虚拟实体标志
    #[serde(default)]
    pub fg_virtual: Option<bool>,
    /// 实体id
    #[serde(default)]
    pub id_entity: Option<String>,
    /// 实体信息
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_entity: Option<DdEntityVO>,
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
    /// 组件实体属性
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub ext_attributes: Vec<ExtAttributeVO>,
    /// 计算属性
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub computation_attributes: Vec<ComputationAttributeVO>,
}
/// ui信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentNodeUiVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_node_ui: String,
    /// x坐标
    #[serde(default)]
    pub x: Option<i32>,
    /// y坐标
    #[serde(default)]
    pub y: Option<i32>,
    /// 宽度
    #[serde(default)]
    pub width: Option<i32>,
    /// 高度
    #[serde(default)]
    pub height: Option<i32>,
    /// 元素id
    #[serde(default)]
    pub id_element: Option<String>,
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
}
/// 简要实体信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
#[tcdt_vo(mod_name = "dd_entity")]
pub struct AssoRefEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity: String,
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
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
}
