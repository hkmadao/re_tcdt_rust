use serde::{Deserialize, Serialize};
/// 属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityAttributePO {
    #[serde(default)]
    pub action: i32,
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
    /// 实体id
    #[serde(default)]
    pub id_entity: Option<String>,
    /// 数据类型id:数据类型id
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/// 枚举实体
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DdEnumPO {
    #[serde(default)]
    pub action: i32,
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
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    /// 枚举属性
    #[serde(default)]
    pub attributes: Vec<EnumAttributePO>,
}
/// 实体集
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityCollectionPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_entity_collection: String,
    /// 名称
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 子项目id
    #[serde(default)]
    pub id_sub_project: Option<String>,
    /// 关系连线
    #[serde(default)]
    pub entity_associates: Vec<EntityAssociatePO>,
    /// ui信息
    #[serde(default)]
    pub node_uis: Vec<NodeUiPO>,
    /// 实体枚举关系
    #[serde(default)]
    pub enum_associates: Vec<EnumAssociatePO>,
    /// 实体信息
    #[serde(default)]
    pub entities: Vec<DdEntityPO>,
    /// 枚举实体
    #[serde(default)]
    pub enums: Vec<DdEnumPO>,
}
/// 实体枚举关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumAssociatePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号
    #[serde(default)]
    pub group_order: Option<i32>,
    /// 枚举id
    #[serde(default)]
    pub id_enum: Option<String>,
    /// 实体id
    #[serde(default)]
    pub id_entity: Option<String>,
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    /// 属性id
    #[serde(default)]
    pub id_attribute: Option<String>,
}
/// 枚举属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumAttributePO {
    #[serde(default)]
    pub action: i32,
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
/// 实体信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DdEntityPO {
    #[serde(default)]
    pub action: i32,
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
    /// 属性
    #[serde(default)]
    pub attributes: Vec<EntityAttributePO>,
}
/// 关系连线
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityAssociatePO {
    #[serde(default)]
    pub action: i32,
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
    /// 上级实体id
    #[serde(default)]
    pub id_up: Option<String>,
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    /// 下级实体id
    #[serde(default)]
    pub id_down: Option<String>,
    /// 是否系统引用连线
    #[serde(default)]
    pub fg_sys_ref: Option<bool>,
}
/// ui信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeUiPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_node_ui: String,
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
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
}
