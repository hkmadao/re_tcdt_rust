use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_user_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "User".to_owned(),
        display_name: "系统用户".to_owned(),
        class_name: "User".to_owned(),
        table_name: "sys_user".to_owned(),
        base_path: "entity::user".to_owned(),
    };
    let id_user_attribute_info = AttributeInfo {
        column_name: "id_user".to_owned(),
        name: "idUser".to_owned(),
        display_name: "系统用户id".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let account_attribute_info = AttributeInfo {
        column_name: "account".to_owned(),
        name: "account".to_owned(),
        display_name: "登录账号 ".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let user_pwd_attribute_info = AttributeInfo {
        column_name: "user_pwd".to_owned(),
        name: "userPwd".to_owned(),
        display_name: "用户密码 ".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let phone_attribute_info = AttributeInfo {
        column_name: "phone".to_owned(),
        name: "phone".to_owned(),
        display_name: "手机号码".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let email_attribute_info = AttributeInfo {
        column_name: "email".to_owned(),
        name: "email".to_owned(),
        display_name: "邮箱".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let name_attribute_info = AttributeInfo {
        column_name: "name".to_owned(),
        name: "name".to_owned(),
        display_name: "姓名 ".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let nick_name_attribute_info = AttributeInfo {
        column_name: "nick_name".to_owned(),
        name: "nickName".to_owned(),
        display_name: "昵称".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let gender_attribute_info = AttributeInfo {
        column_name: "gender".to_owned(),
        name: "gender".to_owned(),
        display_name: "性别".to_owned(),
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
    let user_roles_attribute_info = AttributeInfo {
        column_name: "user_roles".to_owned(),
        name: "userRoles".to_owned(),
        display_name: "用户角色关系".to_owned(),
        data_type: "InternalArray".to_owned(),
        out_entity_name: "UserRole".to_owned(),
        out_entity_pk_attribute_name: "idSysUserRole".to_owned(),
        out_entity_reversal_attribute_name: "user".to_owned(),
        out_entity_id_reversal_attribute_name: "idUser".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_user_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
      ],
      normal_fk_attribute_infos: vec![
      ],
      normal_children: vec![
          user_roles_attribute_info.clone(),
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idUser".to_owned(), id_user_attribute_info),
          ("account".to_owned(), account_attribute_info),
          ("userPwd".to_owned(), user_pwd_attribute_info),
          ("phone".to_owned(), phone_attribute_info),
          ("email".to_owned(), email_attribute_info),
          ("name".to_owned(), name_attribute_info),
          ("nickName".to_owned(), nick_name_attribute_info),
          ("gender".to_owned(), gender_attribute_info),
          ("fgActive".to_owned(), fg_active_attribute_info),
          ("userRoles".to_owned(), user_roles_attribute_info),
      ]),
    };

    entity_desc
}
