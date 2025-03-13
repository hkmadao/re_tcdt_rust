use serde::{Deserialize, Serialize};
/// DTO关系连线
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityAssociatePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_entity_associate: String,
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
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    ///  下级DTO实体信息id
    #[serde(default)]
    pub id_down: Option<String>,
    ///  上级DTO实体信息id
    #[serde(default)]
    pub id_up: Option<String>,
    /// 是否系统引用连线
    #[serde(default)]
    pub fg_sys_ref: Option<bool>,
}
/// DTO实体属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityAttributePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_entity_attribute: String,
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
    /// 类型
    #[serde(default)]
    pub category: Option<String>,
    /// 数据类型id
    #[serde(default)]
    pub id_attribute_type: Option<String>,
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    /// 引用属性id
    #[serde(default)]
    pub id_ref_attribute: Option<String>,
}
/// DTO计算属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoComputationAttributePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_computation_attribute: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
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
    pub pcs: Option<String>,
    /// 序号
    #[serde(default)]
    pub sn: Option<String>,
    /// 数据类型id
    #[serde(default)]
    pub id_attribute_type: Option<String>,
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
}
/// DTO枚举实体
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_enum: String,
    /// 名称
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型
    #[serde(default)]
    pub enum_value_type: Option<String>,
    /// 引用枚举id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// DTO枚举属性
    #[serde(default)]
    pub dto_enum_attributes: Vec<DtoEnumAttributePO>,
}
/// DTO实体集ui信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoNodeUiPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_node_ui: String,
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
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
}
/// DTO枚举属性
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAttributePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_enum_attribute: String,
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
    /// 引用id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// DTO枚举id
    #[serde(default)]
    pub id_dto_enum: Option<String>,
}
/// DTO实体信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_entity: String,
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
    /// 引用实体id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// DTO计算属性
    #[serde(default)]
    pub dc_attributes: Vec<DtoComputationAttributePO>,
    /// DTO实体属性
    #[serde(default)]
    pub de_attributes: Vec<DtoEntityAttributePO>,
}
/// DTO实体枚举关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAssociatePO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号
    #[serde(default)]
    pub group_order: Option<i32>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// DTO枚举id
    #[serde(default)]
    pub id_dto_enum: Option<String>,
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    /// DTO实体属性id
    #[serde(default)]
    pub id_dto_entity_attribute: Option<String>,
}
/// DTO实体集
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityCollectionPO {
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub id_dto_entity_collection: String,
    /// 名称
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 主DTO实体集id
    #[serde(default)]
    pub id_main_dto_entity: Option<String>,
    /// DTO模块id
    #[serde(default)]
    pub id_dto_module: Option<String>,
    /// DTO关系连线
    #[serde(default)]
    pub de_associates: Vec<DtoEntityAssociatePO>,
    /// DTO实体枚举关系
    #[serde(default)]
    pub dto_enum_associates: Vec<DtoEnumAssociatePO>,
    /// DTO枚举实体
    #[serde(default)]
    pub dto_enums: Vec<DtoEnumPO>,
    /// DTO实体集ui信息
    #[serde(default)]
    pub dto_node_uis: Vec<DtoNodeUiPO>,
    /// DTO实体信息
    #[serde(default)]
    pub dto_entities: Vec<DtoEntityPO>,
}
