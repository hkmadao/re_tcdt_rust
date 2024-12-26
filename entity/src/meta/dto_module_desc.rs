use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_module_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoModule".to_owned(),
        display_name: "DTO模块".to_owned(),
        class_name: "DtoModule".to_owned(),
        table_name: "dto_module".to_owned(),
        base_path: "entity::dto_module".to_owned(),
    };
    let id_dto_module_attribute_info = AttributeInfo {
        column_name: "id_dto_module".to_owned(),
        name: "idDtoModule".to_owned(),
        display_name: "DTO模块id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let name_attribute_info = AttributeInfo {
        column_name: "name".to_owned(),
        name: "name".to_owned(),
        display_name: "DTO模块名称".to_owned(),
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
        display_name: "DTO模块空间路径".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_sub_project_attribute_info = AttributeInfo {
        column_name: "id_sub_project".to_owned(),
        name: "idSubProject".to_owned(),
        display_name: "子项目id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "subProject".to_owned(),
        out_entity_name: "SubProject".to_owned(),
        out_entity_pk_attribute_name: "idSubProject".to_owned(),
        out_entity_reversal_attribute_name: "dtoModules".to_owned(),
        ..Default::default()
    };
    let sub_project_attribute_info = AttributeInfo {
        column_name: "sub_project".to_owned(),
        name: "subProject".to_owned(),
        display_name: "模块".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idSubProject".to_owned(),
        out_entity_name: "SubProject".to_owned(),
        out_entity_pk_attribute_name: "idSubProject".to_owned(),
        out_entity_reversal_attribute_name: "dtoModules".to_owned(),
        ..Default::default()
    };
    let de_collections_attribute_info = AttributeInfo {
        column_name: "de_collections".to_owned(),
        name: "deCollections".to_owned(),
        display_name: "DTO实体集".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoModule".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoModule".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_module_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_sub_project_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          sub_project_attribute_info.clone(),
      ],
      normal_children: vec![
          de_collections_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoModule".to_owned(), id_dto_module_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("path".to_owned(), path_attribute_info),
          ("idSubProject".to_owned(), id_sub_project_attribute_info),
          ("subProject".to_owned(), sub_project_attribute_info),
          ("deCollections".to_owned(), de_collections_attribute_info),
      ]),
    };

    entity_desc
}
