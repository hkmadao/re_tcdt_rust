use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_button_action_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "ButtonAction".to_owned(),
        display_name: "按钮操作".to_owned(),
        class_name: "ButtonAction".to_owned(),
        table_name: "ui_button_action".to_owned(),
        base_path: "entity::button_action".to_owned(),
    };
    let id_button_action_attribute_info = AttributeInfo {
        column_name: "id_button_action".to_owned(),
        name: "idButtonAction".to_owned(),
        display_name: "树id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let content_attribute_info = AttributeInfo {
        column_name: "content".to_owned(),
        name: "content".to_owned(),
        display_name: "配置内容".to_owned(),
        data_type: "LongText".to_owned(),
        ..Default::default()
    };
    let name_attribute_info = AttributeInfo {
        column_name: "name".to_owned(),
        name: "name".to_owned(),
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
    let id_project_attribute_info = AttributeInfo {
        column_name: "id_project".to_owned(),
        name: "idProject".to_owned(),
        display_name: "项目id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let project_name_attribute_info = AttributeInfo {
        column_name: "project_name".to_owned(),
        name: "projectName".to_owned(),
        display_name: "项目名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_sub_project_attribute_info = AttributeInfo {
        column_name: "id_sub_project".to_owned(),
        name: "idSubProject".to_owned(),
        display_name: "子项目id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let sub_project_name_attribute_info = AttributeInfo {
        column_name: "sub_project_name".to_owned(),
        name: "subProjectName".to_owned(),
        display_name: "子项目名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_button_action_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
      ],
      normal_fk_attribute_infos: vec![
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idButtonAction".to_owned(), id_button_action_attribute_info),
          ("content".to_owned(), content_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("idProject".to_owned(), id_project_attribute_info),
          ("projectName".to_owned(), project_name_attribute_info),
          ("idSubProject".to_owned(), id_sub_project_attribute_info),
          ("subProjectName".to_owned(), sub_project_name_attribute_info),
      ]),
    };

    entity_desc
}
