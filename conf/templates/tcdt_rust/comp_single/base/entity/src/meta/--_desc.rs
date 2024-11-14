use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_{{ rootInfo.snakeCaseName }}_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "{{ rootInfo.pascalCaseName }}".to_owned(),
        display_name: "{{ rootInfo.displayName }}".to_owned(),
        class_name: "{{ rootInfo.pascalCaseName }}".to_owned(),
        table_name: "{{ rootInfo.tableName }}".to_owned(),
        base_path: "entity::{{ rootInfo.snakeCaseName }}".to_owned(),
    };
    
{%- if rootInfo.attributeInfoList %}
    {%- for attributeInfo in rootInfo.attributeInfoList %}
    let {{ attributeInfo.snakeCaseName }}_attribute_info = AttributeInfo {
        column_name: "{{ attributeInfo.snakeCaseName }}".to_owned(),
        name: "{{ attributeInfo.camelCaseName }}".to_owned(),
        display_name: "{{ attributeInfo.displayName }}".to_owned(),
        data_type: "{{ attributeInfo.attributeTypeCode }}".to_owned(),
      {%- if attributeInfo.innerInfo %}
        inner_attribute_name: "{{ attributeInfo.innerInfo.camelCaseName }}".to_owned(),
      {%- endif %}
      {%- if attributeInfo.outEntityInfo %}
        out_entity_name: "{{ attributeInfo.outEntityInfo.pascalCaseName }}".to_owned(),
        out_entity_pk_attribute_name: "{{ attributeInfo.outEntityInfo.pkAttributeInfo.camelCaseName }}".to_owned(),
      {%- endif %}
      {%- if attributeInfo.outerInfo %}
        out_entity_reversal_attribute_name: "{{ attributeInfo.outerInfo.camelCaseName }}".to_owned(),
      {%- endif %}
      {%- if attributeInfo.outerFkInfo %}
        out_entity_id_reversal_attribute_name: "{{ attributeInfo.outerFkInfo.camelCaseName }}".to_owned(),
      {%- endif %}
        ..Default::default()
    };
    {%- endfor %}
{%- endif %}
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: {{ rootInfo.pkAttributeInfo.snakeCaseName }}_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
          {{ attributeInfo.snakeCaseName }}_attribute_info.clone(),
{%- endfor %}
      ],
      normal_fk_attribute_infos: vec![
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
          {{ attributeInfo.snakeCaseName }}_attribute_info.clone(),
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
          {{ attributeInfo.snakeCaseName }}_attribute_info.clone(),
{%- endfor %}
      ],
      normal_children: vec![
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
          {{ attributeInfo.snakeCaseName }}_attribute_info.clone(),
{%- endfor %}
      ],
      normal_one_2_one_children: vec![
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
          {{ attributeInfo.snakeCaseName }}_attribute_info.clone(),
{%- endfor %}
      ],
      attribute_info_map: HashMap::from([
{%- if rootInfo.attributeInfoList %}
    {%- for attributeInfo in rootInfo.attributeInfoList %}
          ("{{ attributeInfo.camelCaseName }}".to_owned(), {{ attributeInfo.snakeCaseName }}_attribute_info),
    {%- endfor %}
{%- endif %}
      ]),
    };

    entity_desc
}
