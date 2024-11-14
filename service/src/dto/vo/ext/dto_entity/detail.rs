use ::entity::entity::{
    dd_entity, dd_enum, dto_computation_attribute, dto_entity, dto_entity_attribute, dto_enum,
    dto_enum_attribute, entity_attribute,
};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;
/**枚举实体 */
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
}
/**DTO枚举属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
/**实体信息 */
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
}
/**属性 */
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
}
/**DTO枚举实体 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
    /// 引用id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// 引用枚举
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_enum: Option<DdEnumVO>,
    /// DTO枚举属性
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub dto_enum_attributes: Vec<DtoEnumAttributeVO>,
}
/**DTO实体属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    /// 引用属性id
    #[serde(default)]
    pub id_ref_attribute: Option<String>,
    /// 引用属性
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_attribute: Option<EntityAttributeVO>,
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/**DTO实体信息 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
    /// 引用实体
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_entity: Option<DdEntityVO>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// DTO实体属性
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub de_attributes: Vec<DtoEntityAttributeVO>,
    /// DTO计算属性
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub dc_attributes: Vec<DtoComputationAttributeVO>,
}
/**DTO计算属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoComputationAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
