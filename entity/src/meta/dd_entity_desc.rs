use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dd_entity_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DdEntity".to_owned(),
        display_name: "实体信息".to_owned(),
        class_name: "DdEntity".to_owned(),
        table_name: "dd_entity".to_owned(),
        base_path: "entity::dd_entity".to_owned(),
    };
    let id_entity_attribute_info = AttributeInfo {
        column_name: "id_entity".to_owned(),
        name: "idEntity".to_owned(),
        display_name: "实体id".to_owned(),
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
    let id_entity_collection_attribute_info = AttributeInfo {
        column_name: "id_entity_collection".to_owned(),
        name: "idEntityCollection".to_owned(),
        display_name: "实体集id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "entityCollection".to_owned(),
        out_entity_name: "EntityCollection".to_owned(),
        out_entity_pk_attribute_name: "idEntityCollection".to_owned(),
        out_entity_reversal_attribute_name: "entities".to_owned(),
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
        out_entity_reversal_attribute_name: "entities".to_owned(),
        ..Default::default()
    };
    let common_attributes_attribute_info = AttributeInfo {
        column_name: "common_attributes".to_owned(),
        name: "commonAttributes".to_owned(),
        display_name: "公共属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "CommonAttribute".to_owned(),
        out_entity_pk_attribute_name: "idCommonAttribute".to_owned(),
        out_entity_reversal_attribute_name: "refEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idRefEntity".to_owned(),
        ..Default::default()
    };
    let dto_entitys_attribute_info = AttributeInfo {
        column_name: "dto_entitys".to_owned(),
        name: "dtoEntitys".to_owned(),
        display_name: "DTO实体信息".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "refEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idRef".to_owned(),
        ..Default::default()
    };
    let down_associates_attribute_info = AttributeInfo {
        column_name: "down_associates".to_owned(),
        name: "downAssociates".to_owned(),
        display_name: "下级关系连线".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "upEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idUp".to_owned(),
        ..Default::default()
    };
    let component_entities_attribute_info = AttributeInfo {
        column_name: "component_entities".to_owned(),
        name: "componentEntities".to_owned(),
        display_name: "组件实体".to_owned(),
        data_type: "InternalSingle".to_owned(),
        out_entity_name: "ComponentEntity".to_owned(),
        out_entity_pk_attribute_name: "idComponentEntity".to_owned(),
        out_entity_reversal_attribute_name: "ddEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntity".to_owned(),
        ..Default::default()
    };
    let attributes_attribute_info = AttributeInfo {
        column_name: "attributes".to_owned(),
        name: "attributes".to_owned(),
        display_name: "属性".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EntityAttribute".to_owned(),
        out_entity_pk_attribute_name: "idAttribute".to_owned(),
        out_entity_reversal_attribute_name: "ddEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntity".to_owned(),
        ..Default::default()
    };
    let enum_associates_attribute_info = AttributeInfo {
        column_name: "enum_associates".to_owned(),
        name: "enumAssociates".to_owned(),
        display_name: "实体枚举关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EnumAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEnumAssociate".to_owned(),
        out_entity_reversal_attribute_name: "ddEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntity".to_owned(),
        ..Default::default()
    };
    let up_associates_attribute_info = AttributeInfo {
        column_name: "up_associates".to_owned(),
        name: "upAssociates".to_owned(),
        display_name: "上级关系连线".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "EntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "downEntity".to_owned(),
        out_entity_id_reversal_attribute_name: "idDown".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_entity_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_entity_collection_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          entity_collection_attribute_info.clone(),
      ],
      normal_children: vec![
          common_attributes_attribute_info.clone(),
          dto_entitys_attribute_info.clone(),
          down_associates_attribute_info.clone(),
          attributes_attribute_info.clone(),
          enum_associates_attribute_info.clone(),
          up_associates_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
          component_entities_attribute_info.clone(),
      ],
      attribute_info_map: HashMap::from([
          ("idEntity".to_owned(), id_entity_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("className".to_owned(), class_name_attribute_info),
          ("tableName".to_owned(), table_name_attribute_info),
          ("pkAttributeCode".to_owned(), pk_attribute_code_attribute_info),
          ("pkAttributeName".to_owned(), pk_attribute_name_attribute_info),
          ("pkAttributeTypeName".to_owned(), pk_attribute_type_name_attribute_info),
          ("idEntityCollection".to_owned(), id_entity_collection_attribute_info),
          ("entityCollection".to_owned(), entity_collection_attribute_info),
          ("commonAttributes".to_owned(), common_attributes_attribute_info),
          ("dtoEntitys".to_owned(), dto_entitys_attribute_info),
          ("downAssociates".to_owned(), down_associates_attribute_info),
          ("componentEntities".to_owned(), component_entities_attribute_info),
          ("attributes".to_owned(), attributes_attribute_info),
          ("enumAssociates".to_owned(), enum_associates_attribute_info),
          ("upAssociates".to_owned(), up_associates_attribute_info),
      ]),
    };

    entity_desc
}
