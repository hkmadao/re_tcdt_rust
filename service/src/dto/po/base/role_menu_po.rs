use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    role_menu,
    // menu,
    // role,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct RoleMenuPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_role_menu: String,
    #[serde(default)]
    pub id_menu: Option<String>,
    #[serde(default)]
    pub id_role: Option<String>,
}
