use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_menu_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "Menu".to_owned(),
        display_name: "系统菜单".to_owned(),
        class_name: "Menu".to_owned(),
        table_name: "sys_menu".to_owned(),
        base_path: "entity::menu".to_owned(),
    };
    let id_menu_attribute_info = AttributeInfo {
        column_name: "id_menu".to_owned(),
        name: "idMenu".to_owned(),
        display_name: "系统菜单id".to_owned(),
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
    let fg_show_attribute_info = AttributeInfo {
        column_name: "fg_show".to_owned(),
        name: "fgShow".to_owned(),
        display_name: "显示标志".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let query_attribute_info = AttributeInfo {
        column_name: "query".to_owned(),
        name: "query".to_owned(),
        display_name: "路由参数".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let menu_type_attribute_info = AttributeInfo {
        column_name: "menu_type".to_owned(),
        name: "menuType".to_owned(),
        display_name: "菜单类型".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let fg_active_attribute_info = AttributeInfo {
        column_name: "fg_active".to_owned(),
        name: "fgActive".to_owned(),
        display_name: "启用标志".to_owned(),
        data_type: "Boolean".to_owned(),
        ..Default::default()
    };
    let web_perms_attribute_info = AttributeInfo {
        column_name: "web_perms".to_owned(),
        name: "webPerms".to_owned(),
        display_name: "前端权限标识".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let service_perms_attribute_info = AttributeInfo {
        column_name: "service_perms".to_owned(),
        name: "servicePerms".to_owned(),
        display_name: "后台权限标识".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let id_parent_attribute_info = AttributeInfo {
        column_name: "id_parent".to_owned(),
        name: "idParent".to_owned(),
        display_name: "上级系统菜单id".to_owned(),
        data_type: "InternalFK".to_owned(),
        inner_attribute_name: "parent".to_owned(),
        out_entity_name: "Menu".to_owned(),
        out_entity_pk_attribute_name: "idMenu".to_owned(),
        out_entity_reversal_attribute_name: "children".to_owned(),
        ..Default::default()
    };
    let parent_attribute_info = AttributeInfo {
        column_name: "parent".to_owned(),
        name: "parent".to_owned(),
        display_name: "上级系统菜单".to_owned(),
        data_type: "InternalRef".to_owned(),
        inner_attribute_name: "idParent".to_owned(),
        out_entity_name: "Menu".to_owned(),
        out_entity_pk_attribute_name: "idMenu".to_owned(),
        out_entity_reversal_attribute_name: "children".to_owned(),
        ..Default::default()
    };
    let role_menus_attribute_info = AttributeInfo {
        column_name: "role_menus".to_owned(),
        name: "roleMenus".to_owned(),
        display_name: "角色与菜单".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "RoleMenu".to_owned(),
        out_entity_pk_attribute_name: "idRoleMenu".to_owned(),
        out_entity_reversal_attribute_name: "menu".to_owned(),
        out_entity_id_reversal_attribute_name: "idMenu".to_owned(),
        ..Default::default()
    };
    let children_attribute_info = AttributeInfo {
        column_name: "children".to_owned(),
        name: "children".to_owned(),
        display_name: "下级系统菜单".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "Menu".to_owned(),
        out_entity_pk_attribute_name: "idMenu".to_owned(),
        out_entity_reversal_attribute_name: "parent".to_owned(),
        out_entity_id_reversal_attribute_name: "idParent".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_menu_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
          id_parent_attribute_info.clone(),
      ],
      normal_fk_attribute_infos: vec![
          parent_attribute_info.clone(),
      ],
      normal_children: vec![
          role_menus_attribute_info.clone(),
          children_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idMenu".to_owned(), id_menu_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("displayName".to_owned(), display_name_attribute_info),
          ("fgShow".to_owned(), fg_show_attribute_info),
          ("query".to_owned(), query_attribute_info),
          ("menuType".to_owned(), menu_type_attribute_info),
          ("fgActive".to_owned(), fg_active_attribute_info),
          ("webPerms".to_owned(), web_perms_attribute_info),
          ("servicePerms".to_owned(), service_perms_attribute_info),
          ("idParent".to_owned(), id_parent_attribute_info),
          ("parent".to_owned(), parent_attribute_info),
          ("roleMenus".to_owned(), role_menus_attribute_info),
          ("children".to_owned(), children_attribute_info),
      ]),
    };

    entity_desc
}
