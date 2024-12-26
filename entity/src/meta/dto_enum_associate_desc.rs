use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_enum_associate_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEnumAssociate".to_owned(),
        display_name: "DTO实体枚举关系".to_owned(),
        class_name: "DtoEnumAssociate".to_owned(),
        table_name: "dto_enum_associate".to_owned(),
        base_path: "entity::dto_enum_associate".to_owned(),
    };
    let id_dto_enum_associate_attribute_info = AttributeInfo {
        column_name: "id_dto_enum_associate".to_owned(),
        name: "idDtoEnumAssociate".to_owned(),
        display_name: "DTO枚举关系id".to_owned(),
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
    let id_dto_enum_attribute_info = AttributeInfo {
        column_name: "id_dto_enum".to_owned(),
        name: "idDtoEnum".to_owned(),
        display_name: "DTO枚举id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEnum".to_owned(),
        out_entity_name: "DtoEnum".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let dto_enum_attribute_info = AttributeInfo {
        column_name: "dto_enum".to_owned(),
        name: "dtoEnum".to_owned(),
        display_name: "DTO枚举实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoEnum".to_owned(),
        out_entity_name: "DtoEnum".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_collection".to_owned(),
        name: "idDtoEntityCollection".to_owned(),
        display_name: "DTO实体集id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "dto_entity_collection".to_owned(),
        name: "dtoEntityCollection".to_owned(),
        display_name: "DTO实体集".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_name: "DtoEntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_attribute_info = AttributeInfo {
        column_name: "id_dto_entity".to_owned(),
        name: "idDtoEntity".to_owned(),
        display_name: " DTO实体信息id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEntity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let dto_entity_attribute_info = AttributeInfo {
        column_name: "dto_entity".to_owned(),
        name: "dtoEntity".to_owned(),
        display_name: "DTO实体信息".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoEntity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let id_dto_entity_attribute_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_attribute".to_owned(),
        name: "idDtoEntityAttribute".to_owned(),
        display_name: "DTO实体属性id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoEntityAttribute".to_owned(),
        out_entity_name: "DtoEntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAttribute".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let dto_entity_attribute_attribute_info = AttributeInfo {
        column_name: "dto_entity_attribute".to_owned(),
        name: "dtoEntityAttribute".to_owned(),
        display_name: "DTO实体属性".to_owned(),
        data_type: "InternalSingleRef".to_owned(),
        inner_attribute_name: "idDtoEntityAttribute".to_owned(),
        out_entity_name: "DtoEntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAttribute".to_owned(),
        out_entity_reversal_attribute_name: "dtoEnumAssociates".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_enum_associate_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_dto_enum_attribute_info.clone(),
          id_dto_entity_collection_attribute_info.clone(),
          id_dto_entity_attribute_info.clone(),
          id_dto_entity_attribute_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dto_enum_attribute_info.clone(),
          dto_entity_collection_attribute_info.clone(),
          dto_entity_attribute_info.clone(),
          dto_entity_attribute_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEnumAssociate".to_owned(), id_dto_enum_associate_attribute_info),
          ("groupOrder".to_owned(), group_order_attribute_info),
          ("idDtoEnum".to_owned(), id_dto_enum_attribute_info),
          ("dtoEnum".to_owned(), dto_enum_attribute_info),
          ("idDtoEntityCollection".to_owned(), id_dto_entity_collection_attribute_info),
          ("dtoEntityCollection".to_owned(), dto_entity_collection_attribute_info),
          ("idDtoEntity".to_owned(), id_dto_entity_attribute_info),
          ("dtoEntity".to_owned(), dto_entity_attribute_info),
          ("idDtoEntityAttribute".to_owned(), id_dto_entity_attribute_attribute_info),
          ("dtoEntityAttribute".to_owned(), dto_entity_attribute_attribute_info),
      ]),
    };

    entity_desc
}
