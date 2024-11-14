use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_entity_collection_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "EntityCollection".to_owned(),
        display_name: "实体集".to_owned(),
        class_name: "EntityCollection".to_owned(),
        table_name: "dd_entity_collection".to_owned(),
        base_path: "entity::entity_collection".to_owned(),
    };
    let id_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_entity_collection".to_owned(),
        name: "idEntityCollection".to_owned(),
        display_name: "实体集id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let package_name_attribute_info = AttributeInfo {
        column_name: "package_name".to_owned(),
        name: "packageName".to_owned(),
        display_name: "代码包名".to_owned(),
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
    let id_sub_project_attribute_info = AttributeInfo {
        column_name: "id_sub_project".to_owned(),
        name: "idSubProject".to_owned(),
        display_name: "子项目id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "sub_project".to_owned(),
        out_entity_name: "SubProject".to_owned(),
        out_entity_pk_attribute_name: "idSubProject".to_owned(),
        out_entity_reversal_attribute_name: "entityCollections".to_owned(),
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
        out_entity_reversal_attribute_name: "entityCollections".to_owned(),
        ..Default::default()
    };
    let entity_associates_attribute_info = AttributeInfo {
        column_name: "entity_associates".to_owned(),
        name: "entityAssociates".to_owned(),
        display_name: "关系连线".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "entityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntityCollection".to_owned(),
        ..Default::default()
    };
    let node_uis_attribute_info = AttributeInfo {
        column_name: "node_uis".to_owned(),
        name: "nodeUis".to_owned(),
        display_name: "ui信息".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "NodeUi".to_owned(),
        out_entity_pk_attribute_name: "idNodeUi".to_owned(),
        out_entity_reversal_attribute_name: "entityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntityCollection".to_owned(),
        ..Default::default()
    };
    let enum_associates_attribute_info = AttributeInfo {
        column_name: "enum_associates".to_owned(),
        name: "enumAssociates".to_owned(),
        display_name: "实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "entityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntityCollection".to_owned(),
        ..Default::default()
    };
    let enums_attribute_info = AttributeInfo {
        column_name: "enums".to_owned(),
        name: "enums".to_owned(),
        display_name: "枚举实体".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DdEnum".to_owned(),
        out_entity_pk_attribute_name: "idEnum".to_owned(),
        out_entity_reversal_attribute_name: "entityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntityCollection".to_owned(),
        ..Default::default()
    };
    let entities_attribute_info = AttributeInfo {
        column_name: "entities".to_owned(),
        name: "entities".to_owned(),
        display_name: "实体信息".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "entityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntityCollection".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_entity_collection_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_sub_project_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          sub_project_attribute_info.clone(),
      ],
      normal_children: vec![
          entity_associates_attribute_info.clone(),
          node_uis_attribute_info.clone(),
          enum_associates_attribute_info.clone(),
          enums_attribute_info.clone(),
          entities_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idEntityCollection".to_owned(), id_entity_collection_attribute_info),
          ("packageName".to_owned(), package_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("idSubProject".to_owned(), id_sub_project_attribute_info),
          ("subProject".to_owned(), sub_project_attribute_info),
          ("entityAssociates".to_owned(), entity_associates_attribute_info),
          ("nodeUis".to_owned(), node_uis_attribute_info),
          ("enumAssociates".to_owned(), enum_associates_attribute_info),
          ("enums".to_owned(), enums_attribute_info),
          ("entities".to_owned(), entities_attribute_info),
      ]),
    };

    entity_desc
}
