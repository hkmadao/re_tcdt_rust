use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_factory_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "Factory".to_owned(),
        display_name: "UI工厂".to_owned(),
        class_name: "Factory".to_owned(),
        table_name: "ui_factory".to_owned(),
        base_path: "entity::factory".to_owned(),
    };
    let id_factory_attribute_info = AttributeInfo {
        column_name: "id_factory".to_owned(),
        name: "idFactory".to_owned(),
        display_name: "工厂id".to_owned(),
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
    let ref_id_content_attribute_info = AttributeInfo {
        column_name: "ref_id_content".to_owned(),
        name: "refIdContent".to_owned(),
        display_name: "引用组件id内容".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_component_module_attribute_info = AttributeInfo {
        column_name: "id_component_module".to_owned(),
        name: "idComponentModule".to_owned(),
        display_name: "组件模块id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let component_module_name_attribute_info = AttributeInfo {
        column_name: "component_module_name".to_owned(),
        name: "componentModuleName".to_owned(),
        display_name: "组件模块名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_component_attribute_info = AttributeInfo {
        column_name: "id_component".to_owned(),
        name: "idComponent".to_owned(),
        display_name: "组件id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let component_name_attribute_info = AttributeInfo {
        column_name: "component_name".to_owned(),
        name: "componentName".to_owned(),
        display_name: "组件名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_template_attribute_info = AttributeInfo {
        column_name: "fg_template".to_owned(),
        name: "fgTemplate".to_owned(),
        display_name: "模板标志".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_factory_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
      ],
      normal_fk_attribute_infos: vec![
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idFactory".to_owned(), id_factory_attribute_info),
          ("content".to_owned(), content_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("idProject".to_owned(), id_project_attribute_info),
          ("projectName".to_owned(), project_name_attribute_info),
          ("idSubProject".to_owned(), id_sub_project_attribute_info),
          ("subProjectName".to_owned(), sub_project_name_attribute_info),
          ("refIdContent".to_owned(), ref_id_content_attribute_info),
          ("idComponentModule".to_owned(), id_component_module_attribute_info),
          ("componentModuleName".to_owned(), component_module_name_attribute_info),
          ("idComponent".to_owned(), id_component_attribute_info),
          ("componentName".to_owned(), component_name_attribute_info),
          ("fgTemplate".to_owned(), fg_template_attribute_info),
      ]),
    };

    entity_desc
}
