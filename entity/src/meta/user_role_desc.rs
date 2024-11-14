use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_user_role_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "UserRole".to_owned(),
        display_name: "用户角色关系".to_owned(),
        class_name: "UserRole".to_owned(),
        table_name: "sys_user_role".to_owned(),
        base_path: "entity::user_role".to_owned(),
    };
    let id_sys_user_role_attribute_info = AttributeInfo {
        column_name: "id_sys_user_role".to_owned(),
        name: "idSysUserRole".to_owned(),
        display_name: "用户角色关系主属性".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let id_user_attribute_info = AttributeInfo {
        column_name: "id_user".to_owned(),
        name: "idUser".to_owned(),
        display_name: "系统用户id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "user".to_owned(),
        out_entity_name: "User".to_owned(),
        out_entity_pk_attribute_name: "idUser".to_owned(),
        out_entity_reversal_attribute_name: "userRoles".to_owned(),
        ..Default::default()
    };
    let user_attribute_info = AttributeInfo {
        column_name: "user".to_owned(),
        name: "user".to_owned(),
        display_name: "系统用户".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idUser".to_owned(),
        out_entity_name: "User".to_owned(),
        out_entity_pk_attribute_name: "idUser".to_owned(),
        out_entity_reversal_attribute_name: "userRoles".to_owned(),
        ..Default::default()
    };
    let id_role_attribute_info = AttributeInfo {
        column_name: "id_role".to_owned(),
        name: "idRole".to_owned(),
        display_name: "角色id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "role".to_owned(),
        out_entity_name: "Role".to_owned(),
        out_entity_pk_attribute_name: "idRole".to_owned(),
        out_entity_reversal_attribute_name: "userRoles".to_owned(),
        ..Default::default()
    };
    let role_attribute_info = AttributeInfo {
        column_name: "role".to_owned(),
        name: "role".to_owned(),
        display_name: "角色".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idRole".to_owned(),
        out_entity_name: "Role".to_owned(),
        out_entity_pk_attribute_name: "idRole".to_owned(),
        out_entity_reversal_attribute_name: "userRoles".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_sys_user_role_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_user_attribute_info.clone(),
          id_role_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          user_attribute_info.clone(),
          role_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idSysUserRole".to_owned(), id_sys_user_role_attribute_info),
          ("idUser".to_owned(), id_user_attribute_info),
          ("user".to_owned(), user_attribute_info),
          ("idRole".to_owned(), id_role_attribute_info),
          ("role".to_owned(), role_attribute_info),
      ]),
    };

    entity_desc
}
