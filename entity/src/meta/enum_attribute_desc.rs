use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_enum_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "EnumAttribute".to_owned(),
        display_name: "枚举属性".to_owned(),
        class_name: "EnumAttribute".to_owned(),
        table_name: "dd_enum_attribute".to_owned(),
        base_path: "entity::enum_attribute".to_owned(),
    };
    let id_enum_attribute_attribute_info = AttributeInfo {
        column_name: "id_enum_attribute".to_owned(),
        name: "idEnumAttribute".to_owned(),
        display_name: "枚举属性id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let display_name_attribute_info = AttributeInfo {
        column_name: "display_name".to_owned(),
        name: "displayName".to_owned(),
        display_name: "枚举属性显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let code_attribute_info = AttributeInfo {
        column_name: "code".to_owned(),
        name: "code".to_owned(),
        display_name: "枚举属性编码".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let enum_value_attribute_info = AttributeInfo {
        column_name: "enum_value".to_owned(),
        name: "enumValue".to_owned(),
        display_name: "枚举值".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let sn_attribute_info = AttributeInfo {
        column_name: "sn".to_owned(),
        name: "sn".to_owned(),
        display_name: "序号".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let id_enum_attribute_info = AttributeInfo {
        column_name: "id_enum".to_owned(),
        name: "idEnum".to_owned(),
        display_name: "枚举id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dd_enum".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "attributes".to_owned(),
        ..Default::default()
    };
    let dd_enum_attribute_info = AttributeInfo {
        column_name: "dd_enum".to_owned(),
        name: "ddEnum".to_owned(),
        display_name: "枚举实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_enum".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "attributes".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_enum_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_enum_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dd_enum_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idEnumAttribute".to_owned(), id_enum_attribute_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("code".to_owned(), code_attribute_info),
          ("enumValue".to_owned(), enum_value_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("idEnum".to_owned(), id_enum_attribute_info),
          ("ddEnum".to_owned(), dd_enum_attribute_info),
      ]),
    };

    entity_desc
}
