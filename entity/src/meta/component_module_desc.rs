use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_component_module_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "ComponentModule".to_owned(),
        display_name: "组件模块".to_owned(),
        class_name: "ComponentModule".to_owned(),
        table_name: "dd_component_module".to_owned(),
        base_path: "entity::component_module".to_owned(),
    };
    let id_component_module_attribute_info = AttributeInfo {
        column_name: "id_component_module".to_owned(),
        name: "idComponentModule".to_owned(),
        display_name: "组件模块id".to_owned(),
        data_type: "InternalPK".to_owned(),
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
        display_name: "组件模块空间路径".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let name_attribute_info = AttributeInfo {
        column_name: "name".to_owned(),
        name: "name".to_owned(),
        display_name: "名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_sub_project_attribute_info = AttributeInfo {
        column_name: "id_sub_project".to_owned(),
        name: "idSubProject".to_owned(),
        display_name: "子项目id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "sub_project".to_owned(),
        out_entity_name: "SubProject".to_owned(),
        out_entity_pk_attribute_name: "idSubProject".to_owned(),
        out_entity_reversal_attribute_name: "componentModules".to_owned(),
        ..Default::default()
    };
    let sub_project_attribute_info = AttributeInfo {
        column_name: "sub_project".to_owned(),
        name: "subProject".to_owned(),
        display_name: "模块".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_sub_project".to_owned(),
        out_entity_name: "SubProject".to_owned(),
        out_entity_pk_attribute_name: "idSubProject".to_owned(),
        out_entity_reversal_attribute_name: "componentModules".to_owned(),
        ..Default::default()
    };
    let components_attribute_info = AttributeInfo {
        column_name: "components".to_owned(),
        name: "components".to_owned(),
        display_name: "组件".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "Component".to_owned(),
        out_entity_pk_attribute_name: "idComponent".to_owned(),
        out_entity_reversal_attribute_name: "componentModule".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponentModule".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_component_module_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_sub_project_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          sub_project_attribute_info.clone(),
      ],
      normal_children: vec![
          components_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idComponentModule".to_owned(), id_component_module_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("path".to_owned(), path_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("idSubProject".to_owned(), id_sub_project_attribute_info),
          ("subProject".to_owned(), sub_project_attribute_info),
          ("components".to_owned(), components_attribute_info),
      ]),
    };

    entity_desc
}
