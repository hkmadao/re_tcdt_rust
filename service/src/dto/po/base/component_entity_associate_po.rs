use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    component_entity_associate,
    // entity_associate,
    // component,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityAssociatePO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_component_entity_associate: String,
    /// 下级实体包名:
    #[serde(default)]
    pub down_package_name: Option<String>,
    /// 上级实体包名:
    #[serde(default)]
    pub up_package_name: Option<String>,
    /// 是否agg关系连线:
    #[serde(default)]
    pub fg_agg_asso: Option<bool>,
    #[serde(default)]
    pub id_entity_associate: Option<String>,
    #[serde(default)]
    pub id_component: Option<String>,
}
