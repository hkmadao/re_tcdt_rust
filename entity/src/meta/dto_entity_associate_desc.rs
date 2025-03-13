use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_dto_entity_associate_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "DtoEntityAssociate".to_owned(),
        display_name: "DTO关系连线".to_owned(),
        class_name: "DtoEntityAssociate".to_owned(),
        table_name: "dto_entity_associate".to_owned(),
        base_path: "entity::dto_entity_associate".to_owned(),
    };
    let id_dto_entity_associate_attribute_info = AttributeInfo {
        column_name: "id_dto_entity_associate".to_owned(),
        name: "idDtoEntityAssociate".to_owned(),
        display_name: "DTO关系连线id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let group_order_attribute_info = AttributeInfo {
        column_name: "group_order".to_owned(),
        name: "groupOrder".to_owned(),
        display_name: "两个实体多条连线时，连线的序号".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let up_associate_type_attribute_info = AttributeInfo {
        column_name: "up_associate_type".to_owned(),
        name: "upAssociateType".to_owned(),
        display_name: "上级关系".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let down_associate_type_attribute_info = AttributeInfo {
        column_name: "down_associate_type".to_owned(),
        name: "downAssociateType".to_owned(),
        display_name: "下级关系".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let down_attribute_name_attribute_info = AttributeInfo {
        column_name: "down_attribute_name".to_owned(),
        name: "downAttributeName".to_owned(),
        display_name: "下级实体属性名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let down_attribute_display_name_attribute_info = AttributeInfo {
        column_name: "down_attribute_display_name".to_owned(),
        name: "downAttributeDisplayName".to_owned(),
        display_name: "下级实体属性显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ref_attribute_name_attribute_info = AttributeInfo {
        column_name: "ref_attribute_name".to_owned(),
        name: "refAttributeName".to_owned(),
        display_name: "引用实体属性".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let ref_attribute_display_name_attribute_info = AttributeInfo {
        column_name: "ref_attribute_display_name".to_owned(),
        name: "refAttributeDisplayName".to_owned(),
        display_name: "引用实体属性显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fk_column_name_attribute_info = AttributeInfo {
        column_name: "fk_column_name".to_owned(),
        name: "fkColumnName".to_owned(),
        display_name: "外键字段名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fk_attribute_name_attribute_info = AttributeInfo {
        column_name: "fk_attribute_name".to_owned(),
        name: "fkAttributeName".to_owned(),
        display_name: "外键属性".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fk_attribute_display_name_attribute_info = AttributeInfo {
        column_name: "fk_attribute_display_name".to_owned(),
        name: "fkAttributeDisplayName".to_owned(),
        display_name: "外键属性显示名称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_sys_ref_attribute_info = AttributeInfo {
        column_name: "fg_sys_ref".to_owned(),
        name: "fgSysRef".to_owned(),
        display_name: "是否系统引用连线".to_owned(),
        data_type: "Boolean".to_owned(),
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
        out_entity_reversal_attribute_name: "deAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "deAssociates".to_owned(),
        ..Default::default()
    };
    let id_up_attribute_info = AttributeInfo {
        column_name: "id_up".to_owned(),
        name: "idUp".to_owned(),
        display_name: "上级实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "upEntity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "downAssociates".to_owned(),
        ..Default::default()
    };
    let up_entity_attribute_info = AttributeInfo {
        column_name: "up_entity".to_owned(),
        name: "upEntity".to_owned(),
        display_name: "上级实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idUp".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "downAssociates".to_owned(),
        ..Default::default()
    };
    let id_down_attribute_info = AttributeInfo {
        column_name: "id_down".to_owned(),
        name: "idDown".to_owned(),
        display_name: "下级实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "downEntity".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "upAssociates".to_owned(),
        ..Default::default()
    };
    let down_entity_attribute_info = AttributeInfo {
        column_name: "down_entity".to_owned(),
        name: "downEntity".to_owned(),
        display_name: "下级实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDown".to_owned(),
        out_entity_name: "DtoEntity".to_owned(),
        out_entity_pk_attribute_name: "idDtoEntity".to_owned(),
        out_entity_reversal_attribute_name: "upAssociates".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_dto_entity_associate_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_dto_entity_collection_attribute_info.clone(),
          id_up_attribute_info.clone(),
          id_down_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          dto_entity_collection_attribute_info.clone(),
          up_entity_attribute_info.clone(),
          down_entity_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idDtoEntityAssociate".to_owned(), id_dto_entity_associate_attribute_info),
          ("groupOrder".to_owned(), group_order_attribute_info),
          ("upAssociateType".to_owned(), up_associate_type_attribute_info),
          ("downAssociateType".to_owned(), down_associate_type_attribute_info),
          ("downAttributeName".to_owned(), down_attribute_name_attribute_info),
          ("downAttributeDisplayName".to_owned(), down_attribute_display_name_attribute_info),
          ("refAttributeName".to_owned(), ref_attribute_name_attribute_info),
          ("refAttributeDisplayName".to_owned(), ref_attribute_display_name_attribute_info),
          ("fkColumnName".to_owned(), fk_column_name_attribute_info),
          ("fkAttributeName".to_owned(), fk_attribute_name_attribute_info),
          ("fkAttributeDisplayName".to_owned(), fk_attribute_display_name_attribute_info),
          ("fgSysRef".to_owned(), fg_sys_ref_attribute_info),
          ("idDtoEntityCollection".to_owned(), id_dto_entity_collection_attribute_info),
          ("dtoEntityCollection".to_owned(), dto_entity_collection_attribute_info),
          ("idUp".to_owned(), id_up_attribute_info),
          ("upEntity".to_owned(), up_entity_attribute_info),
          ("idDown".to_owned(), id_down_attribute_info),
          ("downEntity".to_owned(), down_entity_attribute_info),
      ]),
    };

    entity_desc
}
