use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_enum_associate_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "EnumAssociate".to_owned(),
        display_name: "实体枚举关系".to_owned(),
        class_name: "EnumAssociate".to_owned(),
        table_name: "dd_enum_associate".to_owned(),
        base_path: "entity::enum_associate".to_owned(),
    };
    let id_enum_associate_attribute_info = AttributeInfo {
        column_name: "id_enum_associate".to_owned(),
        name: "idEnumAssociate".to_owned(),
        display_name: "枚举关系id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let group_order_attribute_info = AttributeInfo {
        column_name: "group_order".to_owned(),
        name: "groupOrder".to_owned(),
        display_name: "两个相同实体和枚举多条连线时，连线的序号".to_owned(),
        data_type: "Integer".to_owned(),
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
        out_entity_reversal_attribute_name: "enumAssociate".to_owned(),
        ..Default::default()
    };
    let attribute_attribute_info = AttributeInfo {
        column_name: "attribute".to_owned(),
        name: "attribute".to_owned(),
        display_name: "属性".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_attribute".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "enumAssociate".to_owned(),
        ..Default::default()
    };
    let id_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_entity_collection".to_owned(),
        name: "idEntityCollection".to_owned(),
        display_name: "实体集id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "entity_collection".to_owned(),
        out_entity_name: "EntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "enumAssociates".to_owned(),
        ..Default::default()
    };
    let entity_collection_attribute_info = AttributeInfo {
        column_name: "entity_collection".to_owned(),
        name: "entityCollection".to_owned(),
        display_name: "实体集".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_entity_collection".to_owned(),
        out_entity_name: "EntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "enumAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "enumAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "enumAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "enumAssociates".to_owned(),
        ..Default::default()
    };
    let dd_entity_attribute_info = AttributeInfo {
        column_name: "dd_entity".to_owned(),
        name: "ddEntity".to_owned(),
        display_name: "实体信息".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "id_entity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "enumAssociates".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_enum_associate_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_attribute_attribute_info.clone(),
          id_entity_collection_attribute_info.clone(),
          id_enum_attribute_info.clone(),
          id_entity_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          attribute_attribute_info.clone(),
          entity_collection_attribute_info.clone(),
          dd_enum_attribute_info.clone(),
          dd_entity_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idEnumAssociate".to_owned(), id_enum_associate_attribute_info),
          ("groupOrder".to_owned(), group_order_attribute_info),
          ("idAttribute".to_owned(), id_attribute_attribute_info),
          ("attribute".to_owned(), attribute_attribute_info),
          ("idEntityCollection".to_owned(), id_entity_collection_attribute_info),
          ("entityCollection".to_owned(), entity_collection_attribute_info),
          ("idEnum".to_owned(), id_enum_attribute_info),
          ("ddEnum".to_owned(), dd_enum_attribute_info),
          ("idEntity".to_owned(), id_entity_attribute_info),
          ("ddEntity".to_owned(), dd_entity_attribute_info),
      ]),
    };

    entity_desc
}
