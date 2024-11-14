use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    entity_associate,
    // entity_collection,
    // dd_entity,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct EntityAssociatePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
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
}
