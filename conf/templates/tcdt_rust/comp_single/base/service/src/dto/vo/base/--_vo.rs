use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    {{ rootInfo.snakeCaseName }},
{%- for upEntiInfo in rootInfo.upEntityInfoList %}
    {{ upEntiInfo.snakeCaseName }},
{%- endfor %}
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct {{ rootInfo.pascalCaseName }}VO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
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
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
{%- endfor %}
}

{%- for upEntiInfo in rootInfo.compInfo.upEntityInfoList %}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct {{ upEntiInfo.pascalCaseName }}VO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub {{ upEntiInfo.pkAttributeInfo.snakeCaseName }}: String,
  {%- for attributeInfo in upEntiInfo.baseAttributeInfoList %}
    /// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}>,
  {%- endfor %}
}
{%- endfor %}