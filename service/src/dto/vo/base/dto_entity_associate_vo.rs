use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    dto_entity_associate,
    dto_entity_collection,
    dto_entity,
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity_associate: String,
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
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    #[serde(default)]
    pub id_up: Option<String>,
    #[serde(default)]
    pub id_down: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dto_entity_collection: Option<DtoEntityCollectionVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub up_entity: Option<DtoEntityVO>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub down_entity: Option<DtoEntityVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityCollectionVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity_collection: String,
    /// 代码包名:
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称:
    #[serde(default)]
    pub display_name: Option<String>,
    /// 主DTO实体集id:
    #[serde(default)]
    pub id_main_dto_entity: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity: String,
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