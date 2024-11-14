use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    component_enum,
    // dd_enum,
    // component,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::ConnectionTrait;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEnumPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_component_enum: String,
    #[serde(default)]
    pub id_enum: Option<String>,
    #[serde(default)]
    pub id_component: Option<String>,
}
