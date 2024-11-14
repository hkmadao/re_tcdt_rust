use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    enum_associate,
    // entity_attribute,
    // entity_collection,
    // dd_enum,
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
pub struct EnumAssociatePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
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
}
