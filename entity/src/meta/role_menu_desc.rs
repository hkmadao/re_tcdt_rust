use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_role_menu_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "RoleMenu".to_owned(),
        display_name: "角色与菜单".to_owned(),
        class_name: "RoleMenu".to_owned(),
        table_name: "sys_role_menu".to_owned(),
        base_path: "entity::role_menu".to_owned(),
    };
    let id_role_menu_attribute_info = AttributeInfo {
        column_name: "id_role_menu".to_owned(),
        name: "idRoleMenu".to_owned(),
        display_name: "角色与菜单id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let id_menu_attribute_info = AttributeInfo {
        column_name: "id_menu".to_owned(),
        name: "idMenu".to_owned(),
        display_name: "系统菜单id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "menu".to_owned(),
        out_entity_name: "Menu".to_owned(),
        out_entity_pk_attribute_name: "idMenu".to_owned(),
        out_entity_reversal_attribute_name: "roleMenus".to_owned(),
        ..Default::default()
    };
    let menu_attribute_info = AttributeInfo {
        column_name: "menu".to_owned(),
        name: "menu".to_owned(),
        display_name: "系统菜单".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idMenu".to_owned(),
        out_entity_name: "Menu".to_owned(),
        out_entity_pk_attribute_name: "idMenu".to_owned(),
        out_entity_reversal_attribute_name: "roleMenus".to_owned(),
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
        out_entity_reversal_attribute_name: "roleMenus".to_owned(),
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
        out_entity_reversal_attribute_name: "roleMenus".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_role_menu_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_menu_attribute_info.clone(),
          id_role_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          menu_attribute_info.clone(),
          role_attribute_info.clone(),
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idRoleMenu".to_owned(), id_role_menu_attribute_info),
          ("idMenu".to_owned(), id_menu_attribute_info),
          ("menu".to_owned(), menu_attribute_info),
          ("idRole".to_owned(), id_role_attribute_info),
          ("role".to_owned(), role_attribute_info),
      ]),
    };

    entity_desc
}
