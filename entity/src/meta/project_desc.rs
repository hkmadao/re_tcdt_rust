use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_project_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "Project".to_owned(),
        display_name: "项目".to_owned(),
        class_name: "Project".to_owned(),
        table_name: "dd_project".to_owned(),
        base_path: "entity::project".to_owned(),
    };
    let id_project_attribute_info = AttributeInfo {
        column_name: "id_project".to_owned(),
        name: "idProject".to_owned(),
        display_name: "项目id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let code_attribute_info = AttributeInfo {
        column_name: "code".to_owned(),
        name: "code".to_owned(),
        display_name: "项目编号".to_owned(),
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
    let path_attribute_info = AttributeInfo {
        column_name: "path".to_owned(),
        name: "path".to_owned(),
        display_name: "系统路径".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let template_code_attribute_info = AttributeInfo {
        column_name: "template_code".to_owned(),
        name: "templateCode".to_owned(),
        display_name: "后台项目模板编号".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let web_template_code_attribute_info = AttributeInfo {
        column_name: "web_template_code".to_owned(),
        name: "webTemplateCode".to_owned(),
        display_name: "前端项目模板编号".to_owned(),
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
    let file_name_type_attribute_info = AttributeInfo {
        column_name: "file_name_type".to_owned(),
        name: "fileNameType".to_owned(),
        display_name: "文件名样式".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let sub_projects_attribute_info = AttributeInfo {
        column_name: "sub_projects".to_owned(),
        name: "subProjects".to_owned(),
        display_name: "模块".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "SubProject".to_owned(),
        out_entity_pk_attribute_name: "idSubProject".to_owned(),
        out_entity_reversal_attribute_name: "project".to_owned(),
        out_entity_id_reversal_attribute_name: "idProject".to_owned(),
        ..Default::default()
    };
    let common_attributes_attribute_info = AttributeInfo {
        column_name: "common_attributes".to_owned(),
        name: "commonAttributes".to_owned(),
        display_name: "公共属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "CommonAttribute".to_owned(),
        out_entity_pk_attribute_name: "idCommonAttribute".to_owned(),
        out_entity_reversal_attribute_name: "project".to_owned(),
        out_entity_id_reversal_attribute_name: "idProject".to_owned(),
        ..Default::default()
    };
    let data_types_attribute_info = AttributeInfo {
        column_name: "data_types".to_owned(),
        name: "dataTypes".to_owned(),
        display_name: "数据类型".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DataType".to_owned(),
        out_entity_pk_attribute_name: "idDataType".to_owned(),
        out_entity_reversal_attribute_name: "project".to_owned(),
        out_entity_id_reversal_attribute_name: "idProject".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_project_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
      ],
      normal_fk_attribute_infos: vec![
      ],
      normal_children: vec![
          sub_projects_attribute_info.clone(),
          common_attributes_attribute_info.clone(),
          data_types_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idProject".to_owned(), id_project_attribute_info),
          ("code".to_owned(), code_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("path".to_owned(), path_attribute_info),
          ("templateCode".to_owned(), template_code_attribute_info),
          ("webTemplateCode".to_owned(), web_template_code_attribute_info),
          ("note".to_owned(), note_attribute_info),
          ("fileNameType".to_owned(), file_name_type_attribute_info),
          ("subProjects".to_owned(), sub_projects_attribute_info),
          ("commonAttributes".to_owned(), common_attributes_attribute_info),
          ("dataTypes".to_owned(), data_types_attribute_info),
      ]),
    };

    entity_desc
}
