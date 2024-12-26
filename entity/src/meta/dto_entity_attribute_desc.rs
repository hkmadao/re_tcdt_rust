use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_entity_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEntityAttribute".to_owned(),
        display_name: "DTO实体属性".to_owned(),
        class_name: "DtoEntityAttribute".to_owned(),
        table_name: "dto_entity_attribute".to_owned(),
        base_path: "entity::dto_entity_attribute".to_owned(),
    };
    let id_dto_entity_attribute_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_attribute".to_owned(),
        name: "idDtoEntityAttribute".to_owned(),
        display_name: "DTO实体属性id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let attribute_name_attribute_info = AttributeInfo {
        column_name: "attribute_name".to_owned(),
        name: "attributeName".to_owned(),
        display_name: "属性名称".to_owned(),
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
    let column_name_attribute_info = AttributeInfo {
        column_name: "column_name".to_owned(),
        name: "columnName".to_owned(),
        display_name: "字段名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_primary_key_attribute_info = AttributeInfo {
        column_name: "fg_primary_key".to_owned(),
        name: "fgPrimaryKey".to_owned(),
        display_name: "是否主键".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let fg_mandatory_attribute_info = AttributeInfo {
        column_name: "fg_mandatory".to_owned(),
        name: "fgMandatory".to_owned(),
        display_name: "是否必填".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let default_value_attribute_info = AttributeInfo {
        column_name: "default_value".to_owned(),
        name: "defaultValue".to_owned(),
        display_name: "默认值".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let len_attribute_info = AttributeInfo {
        column_name: "len".to_owned(),
        name: "len".to_owned(),
        display_name: "数据长度".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let pcs_attribute_info = AttributeInfo {
        column_name: "pcs".to_owned(),
        name: "pcs".to_owned(),
        display_name: "精度".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let sn_attribute_info = AttributeInfo {
        column_name: "sn".to_owned(),
        name: "sn".to_owned(),
        display_name: "序号".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let note_attribute_info = AttributeInfo {
        column_name: "note".to_owned(),
        name: "note".to_owned(),
        display_name: "备注".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let category_attribute_info = AttributeInfo {
        column_name: "category".to_owned(),
        name: "category".to_owned(),
        display_name: "类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_attribute_type_attribute_info = AttributeInfo {
        column_name: "id_attribute_type".to_owned(),
        name: "idAttributeType".to_owned(),
        display_name: "数据类型id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "attributeType".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityAttributes".to_owned(),
        ..Default::default()
    };
    let attribute_type_attribute_info = AttributeInfo {
        column_name: "attribute_type".to_owned(),
        name: "attributeType".to_owned(),
        display_name: "数据类型".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idAttributeType".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityAttributes".to_owned(),
        ..Default::default()
    };
    let id_ref_attribute_attribute_info = AttributeInfo {
        column_name: "id_ref_attribute".to_owned(),
        name: "idRefAttribute".to_owned(),
        display_name: "引用属性id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "refAttribute".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "deAttributes".to_owned(),
        ..Default::default()
    };
    let ref_attribute_attribute_info = AttributeInfo {
        column_name: "ref_attribute".to_owned(),
        name: "refAttribute".to_owned(),
        display_name: "引用属性".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idRefAttribute".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "deAttributes".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_attribute_info = AttributeInfo {
        column_name: "id_dto_entity".to_owned(),
        name: "idDtoEntity".to_owned(),
        display_name: " DTO实体信息id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEntity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "deAttributes".to_owned(),
        ..Default::default()
    };
    let dto_entity_attribute_info = AttributeInfo {
        column_name: "dto_entity".to_owned(),
        name: "dtoEntity".to_owned(),
        display_name: "DTO实体信息".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoEntity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "deAttributes".to_owned(),
        ..Default::default()
    };
    let dto_enum_associates_attribute_info = AttributeInfo {
        column_name: "dto_enum_associates".to_owned(),
        name: "dtoEnumAssociates".to_owned(),
        display_name: "DTO实体枚举关系".to_owned(),
        data_type: "InternalSingle".to_owned(),
        out_entity_name: "DtoEnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityAttribute".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntityAttribute".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_entity_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_attribute_type_attribute_info.clone(),
          id_ref_attribute_attribute_info.clone(),
          id_dto_entity_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          attribute_type_attribute_info.clone(),
          ref_attribute_attribute_info.clone(),
          dto_entity_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
          dto_enum_associates_attribute_info.clone(),
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEntityAttribute".to_owned(), id_dto_entity_attribute_attribute_info),
          ("attributeName".to_owned(), attribute_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("columnName".to_owned(), column_name_attribute_info),
          ("fgPrimaryKey".to_owned(), fg_primary_key_attribute_info),
          ("fgMandatory".to_owned(), fg_mandatory_attribute_info),
          ("defaultValue".to_owned(), default_value_attribute_info),
          ("len".to_owned(), len_attribute_info),
          ("pcs".to_owned(), pcs_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("note".to_owned(), note_attribute_info),
          ("category".to_owned(), category_attribute_info),
          ("idAttributeType".to_owned(), id_attribute_type_attribute_info),
          ("attributeType".to_owned(), attribute_type_attribute_info),
          ("idRefAttribute".to_owned(), id_ref_attribute_attribute_info),
          ("refAttribute".to_owned(), ref_attribute_attribute_info),
          ("idDtoEntity".to_owned(), id_dto_entity_attribute_info),
          ("dtoEntity".to_owned(), dto_entity_attribute_info),
          ("dtoEnumAssociates".to_owned(), dto_enum_associates_attribute_info),
      ]),
    };

    entity_desc
}
