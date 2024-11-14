use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_node_ui_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoNodeUi".to_owned(),
        display_name: "DTO实体集ui信息".to_owned(),
        class_name: "DtoNodeUi".to_owned(),
        table_name: "dto_node_ui".to_owned(),
        base_path: "entity::dto_node_ui".to_owned(),
    };
    let id_dto_node_ui_attribute_info = AttributeInfo {
        column_name: "id_dto_node_ui".to_owned(),
        name: "idDtoNodeUi".to_owned(),
        display_name: "DTO实体集ui信息id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let x_attribute_info = AttributeInfo {
        column_name: "x".to_owned(),
        name: "x".to_owned(),
        display_name: "x坐标".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let y_attribute_info = AttributeInfo {
        column_name: "y".to_owned(),
        name: "y".to_owned(),
        display_name: "y坐标".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let width_attribute_info = AttributeInfo {
        column_name: "width".to_owned(),
        name: "width".to_owned(),
        display_name: "宽度".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let height_attribute_info = AttributeInfo {
        column_name: "height".to_owned(),
        name: "height".to_owned(),
        display_name: "高度".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let id_element_attribute_info = AttributeInfo {
        column_name: "id_element".to_owned(),
        name: "idElement".to_owned(),
        display_name: "元素id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_collection".to_owned(),
        name: "idDtoEntityCollection".to_owned(),
        display_name: "DTO实体集id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dto_entity_collection".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoNodeUis".to_owned(),
        ..Default::default()
    };
    let dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "dto_entity_collection".to_owned(),
        name: "dtoEntityCollection".to_owned(),
        display_name: "DTO实体集".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_dto_entity_collection".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoNodeUis".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_node_ui_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_dto_entity_collection_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dto_entity_collection_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoNodeUi".to_owned(), id_dto_node_ui_attribute_info),
          ("x".to_owned(), x_attribute_info),
          ("y".to_owned(), y_attribute_info),
          ("width".to_owned(), width_attribute_info),
          ("height".to_owned(), height_attribute_info),
          ("idElement".to_owned(), id_element_attribute_info),
          ("idDtoEntityCollection".to_owned(), id_dto_entity_collection_attribute_info),
          ("dtoEntityCollection".to_owned(), dto_entity_collection_attribute_info),
      ]),
    };

    entity_desc
}
