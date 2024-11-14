use ::entity::entity::{component_entity, dd_entity, entity_attribute, ext_attribute};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_entity: String,
    /// 虚拟实体标志:组合实体组件下的组件实体都是虚拟虚拟实体
    #[serde(default)]
    pub fg_virtual: Option<bool>,
    #[serde(default)]
    pub id_entity: Option<String>,
    #[serde(default)]
    pub id_component: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_entity: Option<DdEntityVO>,
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub ext_attributes: Vec<ExtAttributeVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ExtAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_ext_attribute: String,
    /// 扩展字段1:扩展字段1
    #[serde(default)]
    pub ext1: Option<String>,
    /// 排序
    #[serde(default)]
    pub sn: Option<i32>,
    #[serde(default)]
    pub id_component_entity: Option<String>,
    #[serde(default)]
    pub id_attribute: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub attribute: Option<EntityAttributeVO>,
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
    #[serde(default)]
    pub id_entity: Option<String>,
}
