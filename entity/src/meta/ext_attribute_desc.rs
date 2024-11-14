use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_ext_attribute_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "ExtAttribute".to_owned(),
        display_name: "组件实体属性".to_owned(),
        class_name: "ExtAttribute".to_owned(),
        table_name: "dd_ext_attribute".to_owned(),
        base_path: "entity::ext_attribute".to_owned(),
    };
    let id_ext_attribute_attribute_info = AttributeInfo {
        column_name: "id_ext_attribute".to_owned(),
        name: "idExtAttribute".to_owned(),
        display_name: "扩展属性id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let ext1_attribute_info = AttributeInfo {
        column_name: "ext1".to_owned(),
        name: "ext1".to_owned(),
        display_name: "扩展字段1".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let sn_attribute_info = AttributeInfo {
        column_name: "sn".to_owned(),
        name: "sn".to_owned(),
        display_name: "排序".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let id_component_entity_attribute_info = AttributeInfo {
        column_name: "id_component_entity".to_owned(),
        name: "idComponentEntity".to_owned(),
        display_name: "组件实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "component_entity".to_owned(),
        out_entity_name: "ComponentEntity".to_owned(),
        out_entity_pk_attribute_name: "idComponentEntity".to_owned(),
        out_entity_reversal_attribute_name: "extAttributes".to_owned(),
        ..Default::default()
    };
    let component_entity_attribute_info = AttributeInfo {
        column_name: "component_entity".to_owned(),
        name: "componentEntity".to_owned(),
        display_name: "组件实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_component_entity".to_owned(),
        out_entity_name: "ComponentEntity".to_owned(),
        out_entity_pk_attribute_name: "idComponentEntity".to_owned(),
        out_entity_reversal_attribute_name: "extAttributes".to_owned(),
        ..Default::default()
    };
    let id_attribute_attribute_info = AttributeInfo {
        column_name: "id_attribute".to_owned(),
        name: "idAttribute".to_owned(),
        display_name: "属性id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "attribute".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "extAttributes".to_owned(),
        ..Default::default()
    };
    let attribute_attribute_info = AttributeInfo {
        column_name: "attribute".to_owned(),
        name: "attribute".to_owned(),
        display_name: "属性".to_owned(),
        data_type: "InternalSingleRef".to_owned(),
        inner_attribute_name: "id_attribute".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "extAttributes".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_ext_attribute_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_component_entity_attribute_info.clone(),
          id_attribute_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          component_entity_attribute_info.clone(),
          attribute_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idExtAttribute".to_owned(), id_ext_attribute_attribute_info),
          ("ext1".to_owned(), ext1_attribute_info),
          ("sn".to_owned(), sn_attribute_info),
          ("idComponentEntity".to_owned(), id_component_entity_attribute_info),
          ("componentEntity".to_owned(), component_entity_attribute_info),
          ("idAttribute".to_owned(), id_attribute_attribute_info),
          ("attribute".to_owned(), attribute_attribute_info),
      ]),
    };

    entity_desc
}
