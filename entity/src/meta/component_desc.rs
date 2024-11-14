use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_component_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "Component".to_owned(),
        display_name: "组件".to_owned(),
        class_name: "Component".to_owned(),
        table_name: "dd_component".to_owned(),
        base_path: "entity::component".to_owned(),
    };
    let id_component_attribute_info = AttributeInfo {
        column_name: "id_component".to_owned(),
        name: "idComponent".to_owned(),
        display_name: "组件id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let id_main_component_entity_attribute_info = AttributeInfo {
        column_name: "id_main_component_entity".to_owned(),
        name: "idMainComponentEntity".to_owned(),
        display_name: "主实体id".to_owned(),
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
    let package_name_attribute_info = AttributeInfo {
        column_name: "package_name".to_owned(),
        name: "packageName".to_owned(),
        display_name: "包名".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let component_type_attribute_info = AttributeInfo {
        column_name: "component_type".to_owned(),
        name: "componentType".to_owned(),
        display_name: "组件类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_component_module_attribute_info = AttributeInfo {
        column_name: "id_component_module".to_owned(),
        name: "idComponentModule".to_owned(),
        display_name: "组件模块id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "component_module".to_owned(),
        out_entity_name: "ComponentModule".to_owned(),
        out_entity_pk_attribute_name: "idComponentModule".to_owned(),
        out_entity_reversal_attribute_name: "components".to_owned(),
        ..Default::default()
    };
    let component_module_attribute_info = AttributeInfo {
        column_name: "component_module".to_owned(),
        name: "componentModule".to_owned(),
        display_name: "组件模块".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_component_module".to_owned(),
        out_entity_name: "ComponentModule".to_owned(),
        out_entity_pk_attribute_name: "idComponentModule".to_owned(),
        out_entity_reversal_attribute_name: "components".to_owned(),
        ..Default::default()
    };
    let component_enums_attribute_info = AttributeInfo {
        column_name: "component_enums".to_owned(),
        name: "componentEnums".to_owned(),
        display_name: "组件枚举".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComponentEnum".to_owned(),
        out_entity_pk_attribute_name: "idComponentEnum".to_owned(),
        out_entity_reversal_attribute_name: "component".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponent".to_owned(),
        ..Default::default()
    };
    let component_entities_attribute_info = AttributeInfo {
        column_name: "component_entities".to_owned(),
        name: "componentEntities".to_owned(),
        display_name: "组件实体".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComponentEntity".to_owned(),
        out_entity_pk_attribute_name: "idComponentEntity".to_owned(),
        out_entity_reversal_attribute_name: "component".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponent".to_owned(),
        ..Default::default()
    };
    let component_entity_associates_attribute_info = AttributeInfo {
        column_name: "component_entity_associates".to_owned(),
        name: "componentEntityAssociates".to_owned(),
        display_name: "组件关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComponentEntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idComponentEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "component".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponent".to_owned(),
        ..Default::default()
    };
    let component_node_uis_attribute_info = AttributeInfo {
        column_name: "component_node_uis".to_owned(),
        name: "componentNodeUis".to_owned(),
        display_name: "ui信息".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComponentNodeUi".to_owned(),
        out_entity_pk_attribute_name: "idComponentNodeUi".to_owned(),
        out_entity_reversal_attribute_name: "component".to_owned(),
        out_entity_id_reversal_attribute_name: "idComponent".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_component_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_component_module_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          component_module_attribute_info.clone(),
      ],
      normal_children: vec![
          component_enums_attribute_info.clone(),
          component_entities_attribute_info.clone(),
          component_entity_associates_attribute_info.clone(),
          component_node_uis_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idComponent".to_owned(), id_component_attribute_info),
          ("idMainComponentEntity".to_owned(), id_main_component_entity_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("packageName".to_owned(), package_name_attribute_info),
          ("componentType".to_owned(), component_type_attribute_info),
          ("idComponentModule".to_owned(), id_component_module_attribute_info),
          ("componentModule".to_owned(), component_module_attribute_info),
          ("componentEnums".to_owned(), component_enums_attribute_info),
          ("componentEntities".to_owned(), component_entities_attribute_info),
          ("componentEntityAssociates".to_owned(), component_entity_associates_attribute_info),
          ("componentNodeUis".to_owned(), component_node_uis_attribute_info),
      ]),
    };

    entity_desc
}
