use serde::{Deserialize, Serialize};
/// 组件关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityAssociatePO {
    #[serde(default)]
    pub action: i32,
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
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
    /// 实体连线id
    #[serde(default)]
    pub id_entity_associate: Option<String>,
    /// 下级组件实体id:下级组件实体id
    #[serde(default)]
    pub id_down_cp_entity: Option<String>,
    /// 上级组件实体id:上级组件实体id
    #[serde(default)]
    pub id_up_cp_entity: Option<String>,
}
/// 组件实体
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_component_entity: String,
    /// 虚拟实体标志
    #[serde(default)]
    pub fg_virtual: Option<bool>,
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
    /// 实体id
    #[serde(default)]
    pub id_entity: Option<String>,
    /// 组件实体属性
    #[serde(default)]
    pub ext_attributes: Vec<ExtAttributePO>,
    /// 计算属性
    #[serde(default)]
    pub computation_attributes: Vec<ComputationAttributePO>,
}
/// ui信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentNodeUiPO {
    #[serde(default)]
    pub action: i32,
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
/// 组件实体属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtAttributePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_ext_attribute: String,
    /// 扩展字段1
    #[serde(default)]
    pub ext1: Option<String>,
    /// 排序
    #[serde(default)]
    pub sn: Option<i32>,
    /// 属性id
    #[serde(default)]
    pub id_attribute: Option<String>,
    /// 组件实体id
    #[serde(default)]
    pub id_component_entity: Option<String>,
}
/// 组件枚举
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEnumPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_component_enum: String,
    /// 枚举id
    #[serde(default)]
    pub id_enum: Option<String>,
    /// 组件id
    #[serde(default)]
    pub id_component: Option<String>,
}
/// 组件
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentPO {
    #[serde(default)]
    pub action: i32,
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
    /// 组件关系
    #[serde(default)]
    pub component_entity_associates: Vec<ComponentEntityAssociatePO>,
    /// 组件枚举
    #[serde(default)]
    pub component_enums: Vec<ComponentEnumPO>,
    /// 组件实体
    #[serde(default)]
    pub component_entities: Vec<ComponentEntityPO>,
    /// ui信息
    #[serde(default)]
    pub component_node_uis: Vec<ComponentNodeUiPO>,
}
/// 计算属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputationAttributePO {
    #[serde(default)]
    pub action: i32,
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
    /// 数据类型id
    #[serde(default)]
    pub id_attribute_type: Option<String>,
    /// 组件实体id
    #[serde(default)]
    pub id_component_entity: Option<String>,
}
