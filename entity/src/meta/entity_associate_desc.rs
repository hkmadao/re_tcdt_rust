use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_entity_associate_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "EntityAssociate".to_owned(),
        display_name: "关系连线".to_owned(),
        class_name: "EntityAssociate".to_owned(),
        table_name: "dd_entity_associate".to_owned(),
        base_path: "entity::entity_associate".to_owned(),
    };
    let id_entity_associate_attribute_info = AttributeInfo {
        column_name: "id_entity_associate".to_owned(),
        name: "idEntityAssociate".to_owned(),
        display_name: "实体连线id".to_owned(),
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
    let fg_foreign_key_attribute_info = AttributeInfo {
        column_name: "fg_foreign_key".to_owned(),
        name: "fgForeignKey".to_owned(),
        display_name: "是否建立物理外键".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let down_order_str_attribute_info = AttributeInfo {
        column_name: "down_order_str".to_owned(),
        name: "downOrderStr".to_owned(),
        display_name: "下级实体排序".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let down_batch_size_attribute_info = AttributeInfo {
        column_name: "down_batch_size".to_owned(),
        name: "downBatchSize".to_owned(),
        display_name: "批量获取下级实体数量".to_owned(),
        data_type: "Integer".to_owned(),
        ..Default::default()
    };
    let fg_sys_ref_attribute_info = AttributeInfo {
        column_name: "fg_sys_ref".to_owned(),
        name: "fgSysRef".to_owned(),
        display_name: "是否系统引用连线".to_owned(),
        data_type: "Boolean".to_owned(),
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
        out_entity_reversal_attribute_name: "entityAssociates".to_owned(),
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
        out_entity_reversal_attribute_name: "entityAssociates".to_owned(),
        ..Default::default()
    };
    let id_up_attribute_info = AttributeInfo {
        column_name: "id_up".to_owned(),
        name: "idUp".to_owned(),
        display_name: "上级实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "upEntity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "downAssociates".to_owned(),
        ..Default::default()
    };
    let up_entity_attribute_info = AttributeInfo {
        column_name: "up_entity".to_owned(),
        name: "upEntity".to_owned(),
        display_name: "上级实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idUp".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "downAssociates".to_owned(),
        ..Default::default()
    };
    let id_down_attribute_info = AttributeInfo {
        column_name: "id_down".to_owned(),
        name: "idDown".to_owned(),
        display_name: "下级实体id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "downEntity".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "upAssociates".to_owned(),
        ..Default::default()
    };
    let down_entity_attribute_info = AttributeInfo {
        column_name: "down_entity".to_owned(),
        name: "downEntity".to_owned(),
        display_name: "下级实体".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idDown".to_owned(),
        out_entity_name: "DdEntity".to_owned(),
        out_entity_pk_attribute_name: "idEntity".to_owned(),
        out_entity_reversal_attribute_name: "upAssociates".to_owned(),
        ..Default::default()
    };
    let component_entity_associates_attribute_info = AttributeInfo {
        column_name: "component_entity_associates".to_owned(),
        name: "componentEntityAssociates".to_owned(),
        display_name: "组件关系".to_owned(),
        data_type: "InternalSingle".to_owned(),
        out_entity_name: "ComponentEntityAssociate".to_owned(),
        out_entity_pk_attribute_name: "idComponentEntityAssociate".to_owned(),
        out_entity_reversal_attribute_name: "entityAssociate".to_owned(),
        out_entity_id_reversal_attribute_name: "idEntityAssociate".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_entity_associate_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_entity_collection_attribute_info.clone(),
          id_up_attribute_info.clone(),
          id_down_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          entity_collection_attribute_info.clone(),
          up_entity_attribute_info.clone(),
          down_entity_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
          component_entity_associates_attribute_info.clone(),
      ],
      attribute_info_map: HashMap::from([
          ("idEntityAssociate".to_owned(), id_entity_associate_attribute_info),
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
          ("fgForeignKey".to_owned(), fg_foreign_key_attribute_info),
          ("downOrderStr".to_owned(), down_order_str_attribute_info),
          ("downBatchSize".to_owned(), down_batch_size_attribute_info),
          ("fgSysRef".to_owned(), fg_sys_ref_attribute_info),
          ("idEntityCollection".to_owned(), id_entity_collection_attribute_info),
          ("entityCollection".to_owned(), entity_collection_attribute_info),
          ("idUp".to_owned(), id_up_attribute_info),
          ("upEntity".to_owned(), up_entity_attribute_info),
          ("idDown".to_owned(), id_down_attribute_info),
          ("downEntity".to_owned(), down_entity_attribute_info),
          ("componentEntityAssociates".to_owned(), component_entity_associates_attribute_info),
      ]),
    };

    entity_desc
}
