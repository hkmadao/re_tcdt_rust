use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_enum_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEnumAttribute".to_owned(),
        display_name: "DTO枚举属性".to_owned(),
        class_name: "DtoEnumAttribute".to_owned(),
        table_name: "dto_enum_attribute".to_owned(),
        base_path: "entity::dto_enum_attribute".to_owned(),
    };
    let id_dto_enum_attribute_attribute_info = AttributeInfo {
        column_name: "id_dto_enum_attribute".to_owned(),
        name: "idDtoEnumAttribute".to_owned(),
        display_name: "DTO枚举属性id".to_owned(),
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
    let id_ref_attribute_info = AttributeInfo {
        column_name: "id_ref".to_owned(),
        name: "idRef".to_owned(),
        display_name: "引用id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_dto_enum_attribute_info = AttributeInfo {
        column_name: "id_dto_enum".to_owned(),
        name: "idDtoEnum".to_owned(),
        display_name: "DTO枚举id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEnum".to_owned(),
        out_entity_name: "DtoEnum".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAttributes".to_owned(),
        ..Default::default()
    };
    let dto_enum_attribute_info = AttributeInfo {
        column_name: "dto_enum".to_owned(),
        name: "dtoEnum".to_owned(),
        display_name: "DTO枚举实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoEnum".to_owned(),
        out_entity_name: "DtoEnum".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAttributes".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_enum_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_dto_enum_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dto_enum_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEnumAttribute".to_owned(), id_dto_enum_attribute_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("code".to_owned(), code_attribute_info),
          ("enumValue".to_owned(), enum_value_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("idRef".to_owned(), id_ref_attribute_info),
          ("idDtoEnum".to_owned(), id_dto_enum_attribute_info),
          ("dtoEnum".to_owned(), dto_enum_attribute_info),
      ]),
    };

    entity_desc
}
