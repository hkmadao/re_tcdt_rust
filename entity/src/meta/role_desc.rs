use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_role_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "Role".to_owned(),
        display_name: "角色".to_owned(),
        class_name: "Role".to_owned(),
        table_name: "sys_role".to_owned(),
        base_path: "entity::role".to_owned(),
    };
    let id_role_attribute_info = AttributeInfo {
        column_name: "id_role".to_owned(),
        name: "idRole".to_owned(),
        display_name: "角色id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let name_attribute_info = AttributeInfo {
        column_name: "name".to_owned(),
        name: "name".to_owned(),
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
    let role_menus_attribute_info = AttributeInfo {
        column_name: "role_menus".to_owned(),
        name: "roleMenus".to_owned(),
        display_name: "角色与菜单".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "RoleMenu".to_owned(),
        out_entity_pk_attribute_name: "idRoleMenu".to_owned(),
        out_entity_reversal_attribute_name: "role".to_owned(),
        out_entity_id_reversal_attribute_name: "idRole".to_owned(),
        ..Default::default()
    };
    let user_roles_attribute_info = AttributeInfo {
        column_name: "user_roles".to_owned(),
        name: "userRoles".to_owned(),
        display_name: "用户角色关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "UserRole".to_owned(),
        out_entity_pk_attribute_name: "idSysUserRole".to_owned(),
        out_entity_reversal_attribute_name: "role".to_owned(),
        out_entity_id_reversal_attribute_name: "idRole".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_role_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
      ],
      normal_fk_attribute_infos: vec![
      ],
      normal_children: vec![
          role_menus_attribute_info.clone(),
          user_roles_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idRole".to_owned(), id_role_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("roleMenus".to_owned(), role_menus_attribute_info),
          ("userRoles".to_owned(), user_roles_attribute_info),
      ]),
    };

    entity_desc
}
