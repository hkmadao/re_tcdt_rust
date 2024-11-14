use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_component_entity_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "ComponentEntity".to_owned(),
        display_name: "组件实体".to_owned(),
        class_name: "ComponentEntity".to_owned(),
        table_name: "dd_component_entity".to_owned(),
        base_path: "entity::component_entity".to_owned(),
    };
    let id_component_entity_attribute_info = AttributeInfo {
        column_name: "id_component_entity".to_owned(),
        name: "idComponentEntity".to_owned(),
        display_name: "组件实体id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let fg_virtual_attribute_info = AttributeInfo {
        column_name: "fg_virtual".to_owned(),
        name: "fgVirtual".to_owned(),
        display_name: "虚拟实体标志".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let id_entity_attribute_info = AttributeInfo {
        column_name: "id_entity".to_owned(),
        name: "idEntity".to_owned(),
        display_name: "实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dd_entity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "componentEntities".to_owned(),
        ..Default::default()
    };
    let dd_entity_attribute_info = AttributeInfo {
        column_name: "dd_entity".to_owned(),
        name: "ddEntity".to_owned(),
        display_name: "实体信息".to_owned(),
        data_type: "InternalSingleRef".to_owned(),
        inner_attribute_name: "idEntity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "componentEntities".to_owned(),
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
        out_entity_reversal_attribute_name: "componentEntities".to_owned(),
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
        out_entity_reversal_attribute_name: "componentEntities".to_owned(),
        ..Default::default()
    };
    let computation_attributes_attribute_info = AttributeInfo {
        column_name: "computation_attributes".to_owned(),
        name: "computationAttributes".to_owned(),
        display_name: "计算属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComputationAttribute".to_owned(),
        out_entity_pk_attribute_name: "idComputationAttribute".to_owned(),
        out_entity_reversal_attribute_name: "componentEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponentEntity".to_owned(),
        ..Default::default()
    };
    let ext_attributes_attribute_info = AttributeInfo {
        column_name: "ext_attributes".to_owned(),
        name: "extAttributes".to_owned(),
        display_name: "组件实体属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ExtAttribute".to_owned(),
        out_entity_pk_attribute_name: "idExtAttribute".to_owned(),
        out_entity_reversal_attribute_name: "componentEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponentEntity".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_component_entity_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_entity_attribute_info.clone(),
          id_component_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          component_attribute_info.clone(),
          dd_entity_attribute_info.clone(),
      ],
      normal_children: vec![
          computation_attributes_attribute_info.clone(),
          ext_attributes_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idComponentEntity".to_owned(), id_component_entity_attribute_info),
          ("fgVirtual".to_owned(), fg_virtual_attribute_info),
          ("idEntity".to_owned(), id_entity_attribute_info),
          ("ddEntity".to_owned(), dd_entity_attribute_info),
          ("idComponent".to_owned(), id_component_attribute_info),
          ("component".to_owned(), component_attribute_info),
          ("computationAttributes".to_owned(), computation_attributes_attribute_info),
          ("extAttributes".to_owned(), ext_attributes_attribute_info),
      ]),
    };

    entity_desc
}
