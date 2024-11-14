use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_common_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "CommonAttribute".to_owned(),
        display_name: "公共属性".to_owned(),
        class_name: "CommonAttribute".to_owned(),
        table_name: "dd_common_attribute".to_owned(),
        base_path: "entity::common_attribute".to_owned(),
    };
    let id_common_attribute_attribute_info = AttributeInfo {
        column_name: "id_common_attribute".to_owned(),
        name: "idCommonAttribute".to_owned(),
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
    let default_value_attribute_info = AttributeInfo {
        column_name: "default_value".to_owned(),
        name: "defaultValue".to_owned(),
        display_name: "默认值".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_mandatory_attribute_info = AttributeInfo {
        column_name: "fg_mandatory".to_owned(),
        name: "fgMandatory".to_owned(),
        display_name: "是否必填".to_owned(),
        data_type: "Boolean".to_owned(),
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
    let ref_attribute_name_attribute_info = AttributeInfo {
        column_name: "ref_attribute_name".to_owned(),
        name: "refAttributeName".to_owned(),
        display_name: "引用属性名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ref_display_name_attribute_info = AttributeInfo {
        column_name: "ref_display_name".to_owned(),
        name: "refDisplayName".to_owned(),
        display_name: "引用属性显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let category_attribute_info = AttributeInfo {
        column_name: "category".to_owned(),
        name: "category".to_owned(),
        display_name: "属性类别".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_preset_attribute_info = AttributeInfo {
        column_name: "fg_preset".to_owned(),
        name: "fgPreset".to_owned(),
        display_name: "系统预置数据标识".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let id_ref_entity_attribute_info = AttributeInfo {
        column_name: "id_ref_entity".to_owned(),
        name: "idRefEntity".to_owned(),
        display_name: "上级实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "ref_entity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "commonAttributes".to_owned(),
        ..Default::default()
    };
    let ref_entity_attribute_info = AttributeInfo {
        column_name: "ref_entity".to_owned(),
        name: "refEntity".to_owned(),
        display_name: "上级实体信息".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_ref_entity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "commonAttributes".to_owned(),
        ..Default::default()
    };
    let id_data_type_attribute_info = AttributeInfo {
        column_name: "id_data_type".to_owned(),
        name: "idDataType".to_owned(),
        display_name: "数据类型id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "data_type".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "commonAttributes".to_owned(),
        ..Default::default()
    };
    let data_type_attribute_info = AttributeInfo {
        column_name: "data_type".to_owned(),
        name: "dataType".to_owned(),
        display_name: "数据类型".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_data_type".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "commonAttributes".to_owned(),
        ..Default::default()
    };
    let id_project_attribute_info = AttributeInfo {
        column_name: "id_project".to_owned(),
        name: "idProject".to_owned(),
        display_name: "项目id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "project".to_owned(),
        out_entity_name: "Project".to_owned(),
        out_entity_pk_attribute_name: "idProject".to_owned(),
        out_entity_reversal_attribute_name: "commonAttributes".to_owned(),
        ..Default::default()
    };
    let project_attribute_info = AttributeInfo {
        column_name: "project".to_owned(),
        name: "project".to_owned(),
        display_name: "项目".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_project".to_owned(),
        out_entity_name: "Project".to_owned(),
        out_entity_pk_attribute_name: "idProject".to_owned(),
        out_entity_reversal_attribute_name: "commonAttributes".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_common_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_ref_entity_attribute_info.clone(),
          id_data_type_attribute_info.clone(),
          id_project_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          ref_entity_attribute_info.clone(),
          data_type_attribute_info.clone(),
          project_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idCommonAttribute".to_owned(), id_common_attribute_attribute_info),
          ("attributeName".to_owned(), attribute_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("columnName".to_owned(), column_name_attribute_info),
          ("defaultValue".to_owned(), default_value_attribute_info),
          ("fgMandatory".to_owned(), fg_mandatory_attribute_info),
          ("len".to_owned(), len_attribute_info),
          ("pcs".to_owned(), pcs_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("refAttributeName".to_owned(), ref_attribute_name_attribute_info),
          ("refDisplayName".to_owned(), ref_display_name_attribute_info),
          ("category".to_owned(), category_attribute_info),
          ("fgPreset".to_owned(), fg_preset_attribute_info),
          ("idRefEntity".to_owned(), id_ref_entity_attribute_info),
          ("refEntity".to_owned(), ref_entity_attribute_info),
          ("idDataType".to_owned(), id_data_type_attribute_info),
          ("dataType".to_owned(), data_type_attribute_info),
          ("idProject".to_owned(), id_project_attribute_info),
          ("project".to_owned(), project_attribute_info),
      ]),
    };

    entity_desc
}
