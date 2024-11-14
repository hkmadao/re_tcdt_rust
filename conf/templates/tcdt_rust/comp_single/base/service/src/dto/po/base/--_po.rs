use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    {{ rootInfo.snakeCaseName }},
{%- for upEntiInfo in rootInfo.upEntityInfoList %}
    // {{ upEntiInfo.snakeCaseName }},
{%- endfor %}
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct {{ rootInfo.pascalCaseName }}PO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub {{ rootInfo.pkAttributeInfo.snakeCaseName }}: String,
{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    /// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}>,
{%- endfor %}
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<String>,
{%- endfor %}
}
