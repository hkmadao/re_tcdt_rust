use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_computation_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoComputationAttribute".to_owned(),
        display_name: "DTO计算属性".to_owned(),
        class_name: "DtoComputationAttribute".to_owned(),
        table_name: "dto_computation_attribute".to_owned(),
        base_path: "entity::dto_computation_attribute".to_owned(),
    };
    let id_dto_computation_attribute_attribute_info = AttributeInfo {
        column_name: "id_dto_computation_attribute".to_owned(),
        name: "idDtoComputationAttribute".to_owned(),
        display_name: " DTO计算属性id".to_owned(),
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
    let note_attribute_info = AttributeInfo {
        column_name: "note".to_owned(),
        name: "note".to_owned(),
        display_name: "备注".to_owned(),
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
    let pcs_attribute_info = AttributeInfo {
        column_name: "pcs".to_owned(),
        name: "pcs".to_owned(),
        display_name: "精度".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let sn_attribute_info = AttributeInfo {
        column_name: "sn".to_owned(),
        name: "sn".to_owned(),
        display_name: "序号".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_attribute_info = AttributeInfo {
        column_name: "id_dto_entity".to_owned(),
        name: "idDtoEntity".to_owned(),
        display_name: " DTO实体信息id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dto_entity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "dcAttributes".to_owned(),
        ..Default::default()
    };
    let dto_entity_attribute_info = AttributeInfo {
        column_name: "dto_entity".to_owned(),
        name: "dtoEntity".to_owned(),
        display_name: "DTO实体信息".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_dto_entity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "dcAttributes".to_owned(),
        ..Default::default()
    };
    let id_attribute_type_attribute_info = AttributeInfo {
        column_name: "id_attribute_type".to_owned(),
        name: "idAttributeType".to_owned(),
        display_name: "数据类型id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "attribute_type".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "dtoComputationAttributes".to_owned(),
        ..Default::default()
    };
    let attribute_type_attribute_info = AttributeInfo {
        column_name: "attribute_type".to_owned(),
        name: "attributeType".to_owned(),
        display_name: "数据类型".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_attribute_type".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "dtoComputationAttributes".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_computation_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_dto_entity_attribute_info.clone(),
          id_attribute_type_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dto_entity_attribute_info.clone(),
          attribute_type_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoComputationAttribute".to_owned(), id_dto_computation_attribute_attribute_info),
          ("attributeName".to_owned(), attribute_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("note".to_owned(), note_attribute_info),
          ("len".to_owned(), len_attribute_info),
          ("fgMandatory".to_owned(), fg_mandatory_attribute_info),
          ("defaultValue".to_owned(), default_value_attribute_info),
          ("pcs".to_owned(), pcs_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("idDtoEntity".to_owned(), id_dto_entity_attribute_info),
          ("dtoEntity".to_owned(), dto_entity_attribute_info),
          ("idAttributeType".to_owned(), id_attribute_type_attribute_info),
          ("attributeType".to_owned(), attribute_type_attribute_info),
      ]),
    };

    entity_desc
}
