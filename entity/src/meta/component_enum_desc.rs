use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_component_enum_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "ComponentEnum".to_owned(),
        display_name: "组件枚举".to_owned(),
        class_name: "ComponentEnum".to_owned(),
        table_name: "dd_component_enum".to_owned(),
        base_path: "entity::component_enum".to_owned(),
    };
    let id_component_enum_attribute_info = AttributeInfo {
        column_name: "id_component_enum".to_owned(),
        name: "idComponentEnum".to_owned(),
        display_name: "组件枚举id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let id_enum_attribute_info = AttributeInfo {
        column_name: "id_enum".to_owned(),
        name: "idEnum".to_owned(),
        display_name: "枚举id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dd_enum".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "componentEnums".to_owned(),
        ..Default::default()
    };
    let dd_enum_attribute_info = AttributeInfo {
        column_name: "dd_enum".to_owned(),
        name: "ddEnum".to_owned(),
        display_name: "枚举实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_enum".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "componentEnums".to_owned(),
        ..Default::default()
    };
    let id_component_attribute_info = AttributeInfo {
        column_name: "id_component".to_owned(),
        name: "idComponent".to_owned(),
        display_name: "组件id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "component".to_owned(),
        out_entity_name: "Component".to_owned(),
        out_entity_pk_attribute_name: "idComponent".to_owned(),
        out_entity_reversal_attribute_name: "componentEnums".to_owned(),
        ..Default::default()
    };
    let component_attribute_info = AttributeInfo {
        column_name: "component".to_owned(),
        name: "component".to_owned(),
        display_name: "组件".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_component".to_owned(),
        out_entity_name: "Component".to_owned(),
        out_entity_pk_attribute_name: "idComponent".to_owned(),
        out_entity_reversal_attribute_name: "componentEnums".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_component_enum_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_enum_attribute_info.clone(),
          id_component_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dd_enum_attribute_info.clone(),
          component_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idComponentEnum".to_owned(), id_component_enum_attribute_info),
          ("idEnum".to_owned(), id_enum_attribute_info),
          ("ddEnum".to_owned(), dd_enum_attribute_info),
          ("idComponent".to_owned(), id_component_attribute_info),
          ("component".to_owned(), component_attribute_info),
      ]),
    };

    entity_desc
}
