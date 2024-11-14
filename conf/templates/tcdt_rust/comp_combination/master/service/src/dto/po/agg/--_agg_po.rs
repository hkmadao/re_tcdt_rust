use ::entity::entity::{
    {{ rootInfo.snakeCaseName }},
{%- for childEnit in rootInfo.compInfo.childEntityInfoList %}
    {{ childEnit.snakeCaseName }},
{%- endfor %}
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::common::aq_const::*;
use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtSaveParamObjectTrait;
use tcdt_macro::ParamOjectSave;

/// {{ rootInfo.displayName }}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[tcdt_po(mod_name = "{{ rootInfo.snakeCaseName }}")]
#[serde(rename_all = "camelCase")]
pub struct {{ rootInfo.pascalCaseName }}AggPO {
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
{%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
    #[serde(default)]
    #[tcdt_po(po_children)]
    pub {{ attributeInfo.snakeCaseName }}: Vec<{{ attributeInfo.objectType }}AggPO>,
{%- endfor %}
{%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
    #[serde(default)]
    #[tcdt_po(po_children)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}AggPO>,
{%- endfor %}
}

{%- for childEnit in rootInfo.compInfo.childEntityInfoList %}
/// {{ childEnit.displayName }}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamOjectSave)]
#[serde(rename_all = "camelCase")]
pub struct {{ childEnit.pascalCaseName }}AggPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub {{ childEnit.pkAttributeInfo.snakeCaseName }}: String,
  {%- for attributeInfo in childEnit.baseAttributeInfoList %}
    /// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}>,
  {%- endfor %}
  {%- for attributeInfo in childEnit.aggFkAttributeInfoList %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<String>,
  {%- endfor %}
  {%- for attributeInfo in childEnit.fkAttributeInfoList %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<String>,
  {%- endfor %}
}
{%- endfor %}
