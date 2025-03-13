use ::entity::entity::{dd_entity, dd_enum, entity_associate, entity_collection, enum_associate};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;
/// 实体集
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EntityCollectionVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity_collection: String,
    /// 名称
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 实体枚举关系
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub enum_associates: Vec<EnumAssociateVO>,
    /// 关系连线
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub entity_associates: Vec<EntityAssociateVO>,
    /// 枚举实体
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "id_enum asc")]
    pub enums: Vec<DdEnumVO>,
    /// 实体信息
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub entities: Vec<DdEntityVO>,
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
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
}
/// 实体枚举关系
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EnumAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号
    #[serde(default)]
    pub group_order: Option<i32>,
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    /// 枚举id
    #[serde(default)]
    pub id_enum: Option<String>,
    /// 属性id
    #[serde(default)]
    pub id_attribute: Option<String>,
    /// 实体id
    #[serde(default)]
    pub id_entity: Option<String>,
}

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
    /// 上级实体id
    #[serde(default)]
    pub id_up: Option<String>,
    /// 下级实体id
    #[serde(default)]
    pub id_down: Option<String>,
    /// 实体集id
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    /// 是否系统引用连线
    #[serde(default)]
    pub fg_sys_ref: Option<bool>,
}
