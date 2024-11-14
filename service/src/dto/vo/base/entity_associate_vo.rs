use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    entity_associate,
    entity_collection,
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
pub struct EntityAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity_associate: String,
    /// 两个实体多条连线时，连线的序号:
    #[serde(default)]
    pub group_order: Option<i32>,
    /// 上级关系:
    #[serde(default)]
    pub up_associate_type: Option<String>,
    /// 下级关系:
    #[serde(default)]
    pub down_associate_type: Option<String>,
    /// 下级实体属性名称:
    #[serde(default)]
    pub down_attribute_name: Option<String>,
    /// 下级实体属性显示名称:
    #[serde(default)]
    pub down_attribute_display_name: Option<String>,
    /// 引用实体属性:
    #[serde(default)]
    pub ref_attribute_name: Option<String>,
    /// 引用实体属性显示名称:
    #[serde(default)]
    pub ref_attribute_display_name: Option<String>,
    /// 外键字段名称:
    #[serde(default)]
    pub fk_column_name: Option<String>,
    /// 外键属性:
    #[serde(default)]
    pub fk_attribute_name: Option<String>,
    /// 外键属性显示名称:
    #[serde(default)]
    pub fk_attribute_display_name: Option<String>,
    /// 是否建立物理外键:
    #[serde(default)]
    pub fg_foreign_key: Option<bool>,
    /// 下级实体排序:
    #[serde(default)]
    pub down_order_str: Option<String>,
    /// 批量获取下级实体数量:
    #[serde(default)]
    pub down_batch_size: Option<i32>,
    #[serde(default)]
    pub id_entity_collection: Option<String>,
    #[serde(default)]
    pub id_up: Option<String>,
    #[serde(default)]
    pub id_down: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub entity_collection: Option<EntityCollectionVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub up_entity: Option<DdEntityVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub down_entity: Option<DdEntityVO>,
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