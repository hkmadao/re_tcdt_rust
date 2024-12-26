use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_entity_collection_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEntityCollection".to_owned(),
        display_name: "DTO实体集".to_owned(),
        class_name: "DtoEntityCollection".to_owned(),
        table_name: "dto_entity_collection".to_owned(),
        base_path: "entity::dto_entity_collection".to_owned(),
    };
    let id_dto_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_collection".to_owned(),
        name: "idDtoEntityCollection".to_owned(),
        display_name: "DTO实体集id".to_owned(),
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
    let id_main_dto_entity_attribute_info = AttributeInfo {
        column_name: "id_main_dto_entity".to_owned(),
        name: "idMainDtoEntity".to_owned(),
        display_name: "主DTO实体集id".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_dto_module_attribute_info = AttributeInfo {
        column_name: "id_dto_module".to_owned(),
        name: "idDtoModule".to_owned(),
        display_name: "DTO模块id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "dtoModule".to_owned(),
        out_entity_name: "DtoModule".to_owned(),
        out_entity_pk_attribute_name: "idDtoModule".to_owned(),
        out_entity_reversal_attribute_name: "deCollections".to_owned(),
        ..Default::default()
    };
    let dto_module_attribute_info = AttributeInfo {
        column_name: "dto_module".to_owned(),
        name: "dtoModule".to_owned(),
        display_name: "DTO模块".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDtoModule".to_owned(),
        out_entity_name: "DtoModule".to_owned(),
        out_entity_pk_attribute_name: "idDtoModule".to_owned(),
        out_entity_reversal_attribute_name: "deCollections".to_owned(),
        ..Default::default()
    };
    let dto_enum_associates_attribute_info = AttributeInfo {
        column_name: "dto_enum_associates".to_owned(),
        name: "dtoEnumAssociates".to_owned(),
        display_name: "DTO实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntityCollection".to_owned(),
        ..Default::default()
    };
    let de_associates_attribute_info = AttributeInfo {
        column_name: "de_associates".to_owned(),
        name: "deAssociates".to_owned(),
        display_name: "DTO关系连线".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntityCollection".to_owned(),
        ..Default::default()
    };
    let dto_enums_attribute_info = AttributeInfo {
        column_name: "dto_enums".to_owned(),
        name: "dtoEnums".to_owned(),
        display_name: "DTO枚举实体".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEnum".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnum".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntityCollection".to_owned(),
        ..Default::default()
    };
    let dto_node_uis_attribute_info = AttributeInfo {
        column_name: "dto_node_uis".to_owned(),
        name: "dtoNodeUis".to_owned(),
        display_name: "DTO实体集ui信息".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoNodeUi".to_owned(),
        out_entity_pk_attribute_name: "idDtoNodeUi".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntityCollection".to_owned(),
        ..Default::default()
    };
    let dto_entities_attribute_info = AttributeInfo {
        column_name: "dto_entities".to_owned(),
        name: "dtoEntities".to_owned(),
        display_name: "DTO实体信息".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntityCollection".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntityCollection".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_entity_collection_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_dto_module_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dto_module_attribute_info.clone(),
      ],
      normal_children: vec![
          dto_enum_associates_attribute_info.clone(),
          de_associates_attribute_info.clone(),
          dto_enums_attribute_info.clone(),
          dto_node_uis_attribute_info.clone(),
          dto_entities_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEntityCollection".to_owned(), id_dto_entity_collection_attribute_info),
          ("packageName".to_owned(), package_name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("idMainDtoEntity".to_owned(), id_main_dto_entity_attribute_info),
          ("idDtoModule".to_owned(), id_dto_module_attribute_info),
          ("dtoModule".to_owned(), dto_module_attribute_info),
          ("dtoEnumAssociates".to_owned(), dto_enum_associates_attribute_info),
          ("deAssociates".to_owned(), de_associates_attribute_info),
          ("dtoEnums".to_owned(), dto_enums_attribute_info),
          ("dtoNodeUis".to_owned(), dto_node_uis_attribute_info),
          ("dtoEntities".to_owned(), dto_entities_attribute_info),
      ]),
    };

    entity_desc
}
