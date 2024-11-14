use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_component_entity_associate_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "ComponentEntityAssociate".to_owned(),
        display_name: "组件关系".to_owned(),
        class_name: "ComponentEntityAssociate".to_owned(),
        table_name: "dd_component_entity_associate".to_owned(),
        base_path: "entity::component_entity_associate".to_owned(),
    };
    let id_component_entity_associate_attribute_info = AttributeInfo {
        column_name: "id_component_entity_associate".to_owned(),
        name: "idComponentEntityAssociate".to_owned(),
        display_name: "组件关系id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let down_package_name_attribute_info = AttributeInfo {
        column_name: "down_package_name".to_owned(),
        name: "downPackageName".to_owned(),
        display_name: "下级实体包名".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let up_package_name_attribute_info = AttributeInfo {
        column_name: "up_package_name".to_owned(),
        name: "upPackageName".to_owned(),
        display_name: "上级实体包名".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_agg_asso_attribute_info = AttributeInfo {
        column_name: "fg_agg_asso".to_owned(),
        name: "fgAggAsso".to_owned(),
        display_name: "是否agg关系连线".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let id_entity_associate_attribute_info = AttributeInfo {
        column_name: "id_entity_associate".to_owned(),
        name: "idEntityAssociate".to_owned(),
        display_name: "id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "entity_associate".to_owned(),
        out_entity_name: "EntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "componentEntityAssociates".to_owned(),
        ..Default::default()
    };
    let entity_associate_attribute_info = AttributeInfo {
        column_name: "entity_associate".to_owned(),
        name: "entityAssociate".to_owned(),
        display_name: "关系连线".to_owned(),
        data_type: "InternalSingleRef".to_owned(),
        inner_attribute_name: "id_entity_associate".to_owned(),
        out_entity_name: "EntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "componentEntityAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "componentEntityAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "componentEntityAssociates".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_component_entity_associate_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_entity_associate_attribute_info.clone(),
          id_component_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          component_attribute_info.clone(),
          entity_associate_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idComponentEntityAssociate".to_owned(), id_component_entity_associate_attribute_info),
          ("downPackageName".to_owned(), down_package_name_attribute_info),
          ("upPackageName".to_owned(), up_package_name_attribute_info),
          ("fgAggAsso".to_owned(), fg_agg_asso_attribute_info),
          ("idEntityAssociate".to_owned(), id_entity_associate_attribute_info),
          ("entityAssociate".to_owned(), entity_associate_attribute_info),
          ("idComponent".to_owned(), id_component_attribute_info),
          ("component".to_owned(), component_attribute_info),
      ]),
    };

    entity_desc
}
