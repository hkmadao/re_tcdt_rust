use ::entity::entity::{
    {{ rootInfo.snakeCaseName }},
{%- for childEnit in rootInfo.compInfo.childEntityInfoList %}
    {{ childEnit.snakeCaseName }},
{%- endfor %}
{%- for upEnti in rootInfo.compInfo.upEntityInfoList %}
    {{ upEnti.snakeCaseName }},
{%- endfor %}
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use sea_orm::prelude::Json;
use sea_orm::ModelTrait;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::{DbConn, DbErr, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct {{ rootInfo.pascalCaseName }}VO {
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
{%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
    #[tcdt_vo(vo_array)]
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Vec<{{ attributeInfo.objectType }}VO>,
{%- endfor %}
{%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
    #[tcdt_vo(vo_array_single)]
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
{%- endfor %}
}

{%- for childEnit in rootInfo.compInfo.childEntityInfoList %}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct {{ childEnit.pascalCaseName }}VO {
    #[tcdt_vo(po_primary_key)]
    #[serde(default)]
    pub {{ childEnit.pkAttributeInfo.snakeCaseName }}: String,
  {%- for attributeInfo in childEnit.baseAttributeInfoList %}
    /// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
  {%- endfor %}
  {%- for attributeInfo in childEnit.aggFkAttributeInfoList %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<String>,
  {%- endfor %}
  {%- for attributeInfo in childEnit.fkAttributeInfoList %}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<String>,
  {%- endfor %}
  {%- for attributeInfo in childEnit.upAttributeInfoList %}
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
  {%- endfor %}
  {%- for attributeInfo in childEnit.upSingleAttributeInfoList %}
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}VO>,
  {%- endfor %}
}
{%- endfor %}

{%- for upEntiInfo in rootInfo.compInfo.upEntityInfoList %}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct {{ upEntiInfo.pascalCaseName }}VO {
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
