use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_enum_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEnum".to_owned(),
        display_name: "DTO枚举实体".to_owned(),
        class_name: "DtoEnum".to_owned(),
        table_name: "dto_enum".to_owned(),
        base_path: "entity::dto_enum".to_owned(),
    };
    let id_dto_enum_attribute_info = AttributeInfo {
        column_name: "id_dto_enum".to_owned(),
        name: "idDtoEnum".to_owned(),
        display_name: "DTO枚举id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let class_name_attribute_info = AttributeInfo {
        column_name: "class_name".to_owned(),
        name: "className".to_owned(),
        display_name: "名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let display_name_attribute_info = AttributeInfo {
        column_name: "display_name".to_owned(),
        name: "displayName".to_owned(),
        display_name: "显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let enum_value_type_attribute_info = AttributeInfo {
        column_name: "enum_value_type".to_owned(),
        name: "enumValueType".to_owned(),
        display_name: "枚举值的类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_ref_attribute_info = AttributeInfo {
        column_name: "id_ref".to_owned(),
        name: "idRef".to_owned(),
        display_name: "枚举id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "refEnum".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnums".to_owned(),
        ..Default::default()
    };
    let ref_enum_attribute_info = AttributeInfo {
        column_name: "ref_enum".to_owned(),
        name: "refEnum".to_owned(),
        display_name: "引用枚举实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idRef".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnums".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_collection".to_owned(),
        name: "idDtoEntityCollection".to_owned(),
        display_name: "DTO实体集id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnums".to_owned(),
        ..Default::default()
    };
    let dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "dto_entity_collection".to_owned(),
        name: "dtoEntityCollection".to_owned(),
        display_name: "DTO实体集".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnums".to_owned(),
        ..Default::default()
    };
    let dto_enum_attributes_attribute_info = AttributeInfo {
        column_name: "dto_enum_attributes".to_owned(),
        name: "dtoEnumAttributes".to_owned(),
        display_name: "DTO枚举属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEnumAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnumAttribute".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnum".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEnum".to_owned(),
        ..Default::default()
    };
    let dto_enum_associates_attribute_info = AttributeInfo {
        column_name: "dto_enum_associates".to_owned(),
        name: "dtoEnumAssociates".to_owned(),
        display_name: "DTO实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnum".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEnum".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_enum_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_ref_attribute_info.clone(),
          id_dto_entity_collection_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          ref_enum_attribute_info.clone(),
          dto_entity_collection_attribute_info.clone(),
      ],
      normal_children: vec![
          dto_enum_attributes_attribute_info.clone(),
          dto_enum_associates_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEnum".to_owned(), id_dto_enum_attribute_info),
          ("className".to_owned(), class_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("enumValueType".to_owned(), enum_value_type_attribute_info),
          ("idRef".to_owned(), id_ref_attribute_info),
          ("refEnum".to_owned(), ref_enum_attribute_info),
          ("idDtoEntityCollection".to_owned(), id_dto_entity_collection_attribute_info),
          ("dtoEntityCollection".to_owned(), dto_entity_collection_attribute_info),
          ("dtoEnumAttributes".to_owned(), dto_enum_attributes_attribute_info),
          ("dtoEnumAssociates".to_owned(), dto_enum_associates_attribute_info),
      ]),
    };

    entity_desc
}
