use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_data_type_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DataType".to_owned(),
        display_name: "数据类型".to_owned(),
        class_name: "DataType".to_owned(),
        table_name: "dd_data_type".to_owned(),
        base_path: "entity::data_type".to_owned(),
    };
    let id_data_type_attribute_info = AttributeInfo {
        column_name: "id_data_type".to_owned(),
        name: "idDataType".to_owned(),
        display_name: "数据类型id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let code_attribute_info = AttributeInfo {
        column_name: "code".to_owned(),
        name: "code".to_owned(),
        display_name: "数据类型编码".to_owned(),
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
    let sn_attribute_info = AttributeInfo {
        column_name: "sn".to_owned(),
        name: "sn".to_owned(),
        display_name: "序列号".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let len_attribute_info = AttributeInfo {
        column_name: "len".to_owned(),
        name: "len".to_owned(),
        display_name: "长度".to_owned(),
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
    let column_type_attribute_info = AttributeInfo {
        column_name: "column_type".to_owned(),
        name: "columnType".to_owned(),
        display_name: "字段类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let object_type_attribute_info = AttributeInfo {
        column_name: "object_type".to_owned(),
        name: "objectType".to_owned(),
        display_name: "对象类型名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let object_type_package_attribute_info = AttributeInfo {
        column_name: "object_type_package".to_owned(),
        name: "objectTypePackage".to_owned(),
        display_name: "对象类型包名".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ext1_attribute_info = AttributeInfo {
        column_name: "ext1".to_owned(),
        name: "ext1".to_owned(),
        display_name: "扩展属性1".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ext2_attribute_info = AttributeInfo {
        column_name: "ext2".to_owned(),
        name: "ext2".to_owned(),
        display_name: "扩展属性2".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ext3_attribute_info = AttributeInfo {
        column_name: "ext3".to_owned(),
        name: "ext3".to_owned(),
        display_name: "扩展属性3".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ext4_attribute_info = AttributeInfo {
        column_name: "ext4".to_owned(),
        name: "ext4".to_owned(),
        display_name: "扩展属性4".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ext5_attribute_info = AttributeInfo {
        column_name: "ext5".to_owned(),
        name: "ext5".to_owned(),
        display_name: "扩展属性5".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ext6_attribute_info = AttributeInfo {
        column_name: "ext6".to_owned(),
        name: "ext6".to_owned(),
        display_name: "扩展属性6".to_owned(),
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
        display_name: "必填标志".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let type_script_type_attribute_info = AttributeInfo {
        column_name: "type_script_type".to_owned(),
        name: "typeScriptType".to_owned(),
        display_name: "TypeScript类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let web_input_type_attribute_info = AttributeInfo {
        column_name: "web_input_type".to_owned(),
        name: "webInputType".to_owned(),
        display_name: "HTML5输入框类型".to_owned(),
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
    let id_project_attribute_info = AttributeInfo {
        column_name: "id_project".to_owned(),
        name: "idProject".to_owned(),
        display_name: "项目id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "project".to_owned(),
        out_entity_name: "Project".to_owned(),
        out_entity_pk_attribute_name: "idProject".to_owned(),
        out_entity_reversal_attribute_name: "dataTypes".to_owned(),
        ..Default::default()
    };
    let project_attribute_info = AttributeInfo {
        column_name: "project".to_owned(),
        name: "project".to_owned(),
        display_name: "项目".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idProject".to_owned(),
        out_entity_name: "Project".to_owned(),
        out_entity_pk_attribute_name: "idProject".to_owned(),
        out_entity_reversal_attribute_name: "dataTypes".to_owned(),
        ..Default::default()
    };
    let dto_entity_attributes_attribute_info = AttributeInfo {
        column_name: "dto_entity_attributes".to_owned(),
        name: "dtoEntityAttributes".to_owned(),
        display_name: "DTO实体属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAttribute".to_owned(),
        out_entity_reversal_attribute_name: "attributeType".to_owned(),
        out_entity_id_reversal_attribute_name: "idAttributeType".to_owned(),
        ..Default::default()
    };
    let attributes_attribute_info = AttributeInfo {
        column_name: "attributes".to_owned(),
        name: "attributes".to_owned(),
        display_name: "属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "attributeType".to_owned(),
        out_entity_id_reversal_attribute_name: "idAttributeType".to_owned(),
        ..Default::default()
    };
    let dto_computation_attributes_attribute_info = AttributeInfo {
        column_name: "dto_computation_attributes".to_owned(),
        name: "dtoComputationAttributes".to_owned(),
        display_name: "DTO计算属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoComputationAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoComputationAttribute".to_owned(),
        out_entity_reversal_attribute_name: "attributeType".to_owned(),
        out_entity_id_reversal_attribute_name: "idAttributeType".to_owned(),
        ..Default::default()
    };
    let common_attributes_attribute_info = AttributeInfo {
        column_name: "common_attributes".to_owned(),
        name: "commonAttributes".to_owned(),
        display_name: "公共属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "CommonAttribute".to_owned(),
        out_entity_pk_attribute_name: "idCommonAttribute".to_owned(),
        out_entity_reversal_attribute_name: "dataType".to_owned(),
        out_entity_id_reversal_attribute_name: "idDataType".to_owned(),
        ..Default::default()
    };
    let computation_attributes_attribute_info = AttributeInfo {
        column_name: "computation_attributes".to_owned(),
        name: "computationAttributes".to_owned(),
        display_name: "计算属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComputationAttribute".to_owned(),
        out_entity_pk_attribute_name: "idComputationAttribute".to_owned(),
        out_entity_reversal_attribute_name: "attributeType".to_owned(),
        out_entity_id_reversal_attribute_name: "idAttributeType".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_data_type_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_project_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          project_attribute_info.clone(),
      ],
      normal_children: vec![
          dto_entity_attributes_attribute_info.clone(),
          attributes_attribute_info.clone(),
          dto_computation_attributes_attribute_info.clone(),
          common_attributes_attribute_info.clone(),
          computation_attributes_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDataType".to_owned(), id_data_type_attribute_info),
          ("code".to_owned(), code_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("note".to_owned(), note_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("len".to_owned(), len_attribute_info),
          ("pcs".to_owned(), pcs_attribute_info),
          ("columnType".to_owned(), column_type_attribute_info),
          ("objectType".to_owned(), object_type_attribute_info),
          ("objectTypePackage".to_owned(), object_type_package_attribute_info),
          ("ext1".to_owned(), ext1_attribute_info),
          ("ext2".to_owned(), ext2_attribute_info),
          ("ext3".to_owned(), ext3_attribute_info),
          ("ext4".to_owned(), ext4_attribute_info),
          ("ext5".to_owned(), ext5_attribute_info),
          ("ext6".to_owned(), ext6_attribute_info),
          ("defaultValue".to_owned(), default_value_attribute_info),
          ("fgMandatory".to_owned(), fg_mandatory_attribute_info),
          ("typeScriptType".to_owned(), type_script_type_attribute_info),
          ("webInputType".to_owned(), web_input_type_attribute_info),
          ("fgPreset".to_owned(), fg_preset_attribute_info),
          ("idProject".to_owned(), id_project_attribute_info),
          ("project".to_owned(), project_attribute_info),
          ("dtoEntityAttributes".to_owned(), dto_entity_attributes_attribute_info),
          ("attributes".to_owned(), attributes_attribute_info),
          ("dtoComputationAttributes".to_owned(), dto_computation_attributes_attribute_info),
          ("commonAttributes".to_owned(), common_attributes_attribute_info),
          ("computationAttributes".to_owned(), computation_attributes_attribute_info),
      ]),
    };

    entity_desc
}
