use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_sub_project_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "SubProject".to_owned(),
        display_name: "子项目".to_owned(),
        class_name: "SubProject".to_owned(),
        table_name: "dd_sub_project".to_owned(),
        base_path: "entity::sub_project".to_owned(),
    };
    let id_sub_project_attribute_info = AttributeInfo {
        column_name: "id_sub_project".to_owned(),
        name: "idSubProject".to_owned(),
        display_name: "子项目id".to_owned(),
        data_type: "InternalPK".to_owned(),
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
    let path_attribute_info = AttributeInfo {
        column_name: "path".to_owned(),
        name: "path".to_owned(),
        display_name: "子系统路径".to_owned(),
        data_type: "String".to_owned(),
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
        out_entity_reversal_attribute_name: "subProjects".to_owned(),
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
        out_entity_reversal_attribute_name: "subProjects".to_owned(),
        ..Default::default()
    };
    let component_modules_attribute_info = AttributeInfo {
        column_name: "component_modules".to_owned(),
        name: "componentModules".to_owned(),
        display_name: "组件模块".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComponentModule".to_owned(),
        out_entity_pk_attribute_name: "idComponentModule".to_owned(),
        out_entity_reversal_attribute_name: "subProject".to_owned(),
        out_entity_id_reversal_attribute_name: "idSubProject".to_owned(),
        ..Default::default()
    };
    let dto_modules_attribute_info = AttributeInfo {
        column_name: "dto_modules".to_owned(),
        name: "dtoModules".to_owned(),
        display_name: "DTO模块".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoModule".to_owned(),
        out_entity_pk_attribute_name: "idDtoModule".to_owned(),
        out_entity_reversal_attribute_name: "subProject".to_owned(),
        out_entity_id_reversal_attribute_name: "idSubProject".to_owned(),
        ..Default::default()
    };
    let entity_collections_attribute_info = AttributeInfo {
        column_name: "entity_collections".to_owned(),
        name: "entityCollections".to_owned(),
        display_name: "实体集".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "subProject".to_owned(),
        out_entity_id_reversal_attribute_name: "idSubProject".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_sub_project_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_project_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          project_attribute_info.clone(),
      ],
      normal_children: vec![
          component_modules_attribute_info.clone(),
          dto_modules_attribute_info.clone(),
          entity_collections_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idSubProject".to_owned(), id_sub_project_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("path".to_owned(), path_attribute_info),
          ("idProject".to_owned(), id_project_attribute_info),
          ("project".to_owned(), project_attribute_info),
          ("componentModules".to_owned(), component_modules_attribute_info),
          ("dtoModules".to_owned(), dto_modules_attribute_info),
          ("entityCollections".to_owned(), entity_collections_attribute_info),
      ]),
    };

    entity_desc
}
