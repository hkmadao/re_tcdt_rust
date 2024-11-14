use serde::{Deserialize, Serialize};

{%- for entityInfo in rootInfo.entities %}
/// {{ entityInfo.displayName }}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct {{ entityInfo.pascalCaseName }}PO {
    #[serde(default)]
    pub action: i32,
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
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}PO>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.upSingleAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}PO>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.downAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Vec<{{ attributeInfo.objectType }}PO>,
    {%- endfor %}
    {%- for attributeInfo in entityInfo.downSingleAttributeInfoList %}
    /// {{ attributeInfo.displayName }}
    #[serde(default)]
    pub {{ attributeInfo.snakeCaseName }}: Option<{{ attributeInfo.objectType }}PO>,
    {%- endfor %}
}

{%- endfor %}
