use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dd_enum_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DdEnum".to_owned(),
        display_name: "枚举实体".to_owned(),
        class_name: "DdEnum".to_owned(),
        table_name: "dd_enum".to_owned(),
        base_path: "entity::dd_enum".to_owned(),
    };
    let id_enum_attribute_info = AttributeInfo {
        column_name: "id_enum".to_owned(),
        name: "idEnum".to_owned(),
        display_name: "枚举id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let class_name_attribute_info = AttributeInfo {
        column_name: "class_name".to_owned(),
        name: "className".to_owned(),
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
    let enum_value_type_attribute_info = AttributeInfo {
        column_name: "enum_value_type".to_owned(),
        name: "enumValueType".to_owned(),
        display_name: "枚举值的类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_entity_collection".to_owned(),
        name: "idEntityCollection".to_owned(),
        display_name: "实体集id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "entityCollection".to_owned(),
        out_entity_name: "EntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "enums".to_owned(),
        ..Default::default()
    };
    let entity_collection_attribute_info = AttributeInfo {
        column_name: "entity_collection".to_owned(),
        name: "entityCollection".to_owned(),
        display_name: "实体集".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idEntityCollection".to_owned(),
        out_entity_name: "EntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "enums".to_owned(),
        ..Default::default()
    };
    let enum_associates_attribute_info = AttributeInfo {
        column_name: "enum_associates".to_owned(),
        name: "enumAssociates".to_owned(),
        display_name: "实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "ddEnum".to_owned(),
        out_entity_id_reversal_attribute_name: "idEnum".to_owned(),
        ..Default::default()
    };
    let component_enums_attribute_info = AttributeInfo {
        column_name: "component_enums".to_owned(),
        name: "componentEnums".to_owned(),
        display_name: "组件枚举".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "ComponentEnum".to_owned(),
        out_entity_pk_attribute_name: "idComponentEnum".to_owned(),
        out_entity_reversal_attribute_name: "ddEnum".to_owned(),
        out_entity_id_reversal_attribute_name: "idEnum".to_owned(),
        ..Default::default()
    };
    let attributes_attribute_info = AttributeInfo {
        column_name: "attributes".to_owned(),
        name: "attributes".to_owned(),
        display_name: "枚举属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EnumAttribute".to_owned(),
        out_entity_pk_attribute_name: "idEnumAttribute".to_owned(),
        out_entity_reversal_attribute_name: "ddEnum".to_owned(),
        out_entity_id_reversal_attribute_name: "idEnum".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_enum_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_entity_collection_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          entity_collection_attribute_info.clone(),
      ],
      normal_children: vec![
          enum_associates_attribute_info.clone(),
          component_enums_attribute_info.clone(),
          attributes_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idEnum".to_owned(), id_enum_attribute_info),
          ("className".to_owned(), class_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("enumValueType".to_owned(), enum_value_type_attribute_info),
          ("idEntityCollection".to_owned(), id_entity_collection_attribute_info),
          ("entityCollection".to_owned(), entity_collection_attribute_info),
          ("enumAssociates".to_owned(), enum_associates_attribute_info),
          ("componentEnums".to_owned(), component_enums_attribute_info),
          ("attributes".to_owned(), attributes_attribute_info),
      ]),
    };

    entity_desc
}
