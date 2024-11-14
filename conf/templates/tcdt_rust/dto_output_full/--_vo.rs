use ::entity::entity::{
{%- for entityInfo in rootInfo.entities %}
  {{ entityInfo.snakeCaseName }},
{%- endfor %}
};
// use sea_orm::prelude::Json;
use sea_orm::DbConn;
use sea_orm::DbErr;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;

{%- for entityInfo in rootInfo.entities %}
/// {{ entityInfo.displayName }}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct {{ entityInfo.pascalCaseName }}VO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub {{ entityInfo.pkAttributeInfo.snakeCaseName }}: String,
    {%- for attributeInfo in entityInfo.baseAttributeInfoList %}
        {%- if attributeInfo.attributeTypeCode is matching("xxxxxxx") %}
        {%- else %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}>,
        {%- endif %}
    {%- endfor %}
    {%- for attributeInfo in entityInfo.fkAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<String>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.upAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.upSingleAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.downAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    #[tcdt_vo(vo_array, ignore)]
    pub {{ attributeInfo.snakeCaseName }}: Vec<{{ attributeInfo.objectType }}VO>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.downSingleAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    #[tcdt_vo(vo_array_single, ignore)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
    {%- endfor %}
}

{%- endfor %}
