use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_entity_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "EntityAttribute".to_owned(),
        display_name: "属性".to_owned(),
        class_name: "EntityAttribute".to_owned(),
        table_name: "dd_entity_attribute".to_owned(),
        base_path: "entity::entity_attribute".to_owned(),
    };
    let id_attribute_attribute_info = AttributeInfo {
        column_name: "id_attribute".to_owned(),
        name: "idAttribute".to_owned(),
        display_name: "属性id".to_owned(),
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
        display_name: "分类".to_owned(),
        data_type: "String".to_owned(),
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
        out_entity_reversal_attribute_name: "attributes".to_owned(),
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
        out_entity_reversal_attribute_name: "attributes".to_owned(),
        ..Default::default()
    };
    let id_entity_attribute_info = AttributeInfo {
        column_name: "id_entity".to_owned(),
        name: "idEntity".to_owned(),
        display_name: "实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dd_entity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "attributes".to_owned(),
        ..Default::default()
    };
    let dd_entity_attribute_info = AttributeInfo {
        column_name: "dd_entity".to_owned(),
        name: "ddEntity".to_owned(),
        display_name: "实体信息".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_entity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "attributes".to_owned(),
        ..Default::default()
    };
    let enum_associate_attribute_info = AttributeInfo {
        column_name: "enum_associate".to_owned(),
        name: "enumAssociate".to_owned(),
        display_name: "实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "attribute".to_owned(),
        out_entity_id_reversal_attribute_name: "idAttribute".to_owned(),
        ..Default::default()
    };
    let ext_attributes_attribute_info = AttributeInfo {
        column_name: "ext_attributes".to_owned(),
        name: "extAttributes".to_owned(),
        display_name: "组件实体属性".to_owned(),
        data_type: "InternalSingle".to_owned(),
        out_entity_name: "ExtAttribute".to_owned(),
        out_entity_pk_attribute_name: "idExtAttribute".to_owned(),
        out_entity_reversal_attribute_name: "attribute".to_owned(),
        out_entity_id_reversal_attribute_name: "idAttribute".to_owned(),
        ..Default::default()
    };
    let de_attributes_attribute_info = AttributeInfo {
        column_name: "de_attributes".to_owned(),
        name: "deAttributes".to_owned(),
        display_name: "DTO实体属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAttribute".to_owned(),
        out_entity_reversal_attribute_name: "refAttribute".to_owned(),
        out_entity_id_reversal_attribute_name: "idRefAttribute".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_attribute_type_attribute_info.clone(),
          id_entity_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          attribute_type_attribute_info.clone(),
          dd_entity_attribute_info.clone(),
      ],
      normal_children: vec![
          enum_associate_attribute_info.clone(),
          de_attributes_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
          ext_attributes_attribute_info.clone(),
      ],
      attribute_info_map: HashMap::from([
          ("idAttribute".to_owned(), id_attribute_attribute_info),
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
          ("idEntity".to_owned(), id_entity_attribute_info),
          ("ddEntity".to_owned(), dd_entity_attribute_info),
          ("enumAssociate".to_owned(), enum_associate_attribute_info),
          ("extAttributes".to_owned(), ext_attributes_attribute_info),
          ("deAttributes".to_owned(), de_attributes_attribute_info),
      ]),
    };

    entity_desc
}
