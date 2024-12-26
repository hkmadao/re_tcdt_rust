use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_entity_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEntity".to_owned(),
        display_name: "DTO实体信息".to_owned(),
        class_name: "DtoEntity".to_owned(),
        table_name: "dto_entity".to_owned(),
        base_path: "entity::dto_entity".to_owned(),
    };
    let id_dto_entity_attribute_info = AttributeInfo {
        column_name: "id_dto_entity".to_owned(),
        name: "idDtoEntity".to_owned(),
        display_name: " DTO实体信息id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let display_name_attribute_info = AttributeInfo {
        column_name: "display_name".to_owned(),
        name: "displayName".to_owned(),
        display_name: "显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let class_name_attribute_info = AttributeInfo {
        column_name: "class_name".to_owned(),
        name: "className".to_owned(),
        display_name: "类名".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let table_name_attribute_info = AttributeInfo {
        column_name: "table_name".to_owned(),
        name: "tableName".to_owned(),
        display_name: "表名".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let pk_attribute_code_attribute_info = AttributeInfo {
        column_name: "pk_attribute_code".to_owned(),
        name: "pkAttributeCode".to_owned(),
        display_name: "主属性code".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let pk_attribute_name_attribute_info = AttributeInfo {
        column_name: "pk_attribute_name".to_owned(),
        name: "pkAttributeName".to_owned(),
        display_name: "主属性名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let pk_attribute_type_name_attribute_info = AttributeInfo {
        column_name: "pk_attribute_type_name".to_owned(),
        name: "pkAttributeTypeName".to_owned(),
        display_name: "主属性类型名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_ref_attribute_info = AttributeInfo {
        column_name: "id_ref".to_owned(),
        name: "idRef".to_owned(),
        display_name: "引用实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "refEntity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntitys".to_owned(),
        ..Default::default()
    };
    let ref_entity_attribute_info = AttributeInfo {
        column_name: "ref_entity".to_owned(),
        name: "refEntity".to_owned(),
        display_name: "引用实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idRef".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntitys".to_owned(),
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
        out_entity_reversal_attribute_name: "dtoEntities".to_owned(),
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
        out_entity_reversal_attribute_name: "dtoEntities".to_owned(),
        ..Default::default()
    };
    let down_associates_attribute_info = AttributeInfo {
        column_name: "down_associates".to_owned(),
        name: "downAssociates".to_owned(),
        display_name: "下级关系连线".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "upEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idUp".to_owned(),
        ..Default::default()
    };
    let dto_enum_associates_attribute_info = AttributeInfo {
        column_name: "dto_enum_associates".to_owned(),
        name: "dtoEnumAssociates".to_owned(),
        display_name: "DTO实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntity".to_owned(),
        ..Default::default()
    };
    let dc_attributes_attribute_info = AttributeInfo {
        column_name: "dc_attributes".to_owned(),
        name: "dcAttributes".to_owned(),
        display_name: "DTO计算属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoComputationAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoComputationAttribute".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntity".to_owned(),
        ..Default::default()
    };
    let up_associates_attribute_info = AttributeInfo {
        column_name: "up_associates".to_owned(),
        name: "upAssociates".to_owned(),
        display_name: "下级关系连线".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "downEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idDown".to_owned(),
        ..Default::default()
    };
    let de_attributes_attribute_info = AttributeInfo {
        column_name: "de_attributes".to_owned(),
        name: "deAttributes".to_owned(),
        display_name: "DTO实体属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntityAttribute".to_owned(),
        out_entity_reversal_attribute_name: "dtoEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idDtoEntity".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_entity_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_ref_attribute_info.clone(),
          id_dto_entity_collection_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          ref_entity_attribute_info.clone(),
          dto_entity_collection_attribute_info.clone(),
      ],
      normal_children: vec![
          down_associates_attribute_info.clone(),
          dto_enum_associates_attribute_info.clone(),
          dc_attributes_attribute_info.clone(),
          up_associates_attribute_info.clone(),
          de_attributes_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEntity".to_owned(), id_dto_entity_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("className".to_owned(), class_name_attribute_info),
          ("tableName".to_owned(), table_name_attribute_info),
          ("pkAttributeCode".to_owned(), pk_attribute_code_attribute_info),
          ("pkAttributeName".to_owned(), pk_attribute_name_attribute_info),
          ("pkAttributeTypeName".to_owned(), pk_attribute_type_name_attribute_info),
          ("idRef".to_owned(), id_ref_attribute_info),
          ("refEntity".to_owned(), ref_entity_attribute_info),
          ("idDtoEntityCollection".to_owned(), id_dto_entity_collection_attribute_info),
          ("dtoEntityCollection".to_owned(), dto_entity_collection_attribute_info),
          ("downAssociates".to_owned(), down_associates_attribute_info),
          ("dtoEnumAssociates".to_owned(), dto_enum_associates_attribute_info),
          ("dcAttributes".to_owned(), dc_attributes_attribute_info),
          ("upAssociates".to_owned(), up_associates_attribute_info),
          ("deAttributes".to_owned(), de_attributes_attribute_info),
      ]),
    };

    entity_desc
}
