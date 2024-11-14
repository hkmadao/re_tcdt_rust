use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    enum_associate,
    entity_attribute,
    entity_collection,
    dd_enum,
    dd_entity,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EnumAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号:
    #[serde(default)]
    pub group_order: Option<i32>,
    #[serde(default)]
    pub id_attribute: Option<String>,
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    #[serde(default)]
    pub id_enum: Option<String>,
    #[serde(default)]
    pub id_entity: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub attribute: Option<EntityAttributeVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub entity_collection: Option<EntityCollectionVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_enum: Option<DdEnumVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_entity: Option<DdEntityVO>,
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
    /// 属性名称:
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 字段名称:
    #[serde(default)]
    pub column_name: Option<String>,
    /// 是否主键:
    #[serde(default)]
    pub fg_primary_key: Option<bool>,
    /// 是否必填:
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值:
    #[serde(default)]
    pub default_value: Option<String>,
    /// 数据长度:
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度:
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号:
    #[serde(default)]
    pub sn: Option<i32>,
    /// 备注:
    #[serde(default)]
    pub note: Option<String>,
    /// 分类:
    #[serde(default)]
    pub category: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EntityCollectionVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity_collection: String,
    /// 代码包名:
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum: String,
    /// 名称:
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型:
    #[serde(default)]
    pub enum_value_type: Option<String>,
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