use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    dto_enum_associate,
    // dto_enum,
    // dto_entity_collection,
    // dto_entity,
    // dto_entity_attribute,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAssociatePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_dto_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号:
    #[serde(default)]
    pub group_order: Option<i32>,
    #[serde(default)]
    pub id_dto_enum: Option<String>,
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    #[serde(default)]
    pub id_dto_entity_attribute: Option<String>,
}
