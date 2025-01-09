use std::collections::HashMap;

use crate::common::desc::{AttributeInfo, EntityDesc, EntityInfo};

pub fn get_token_desc() ->  EntityDesc {
    let entity_info = EntityInfo {
        name: "Token".to_owned(),
        display_name: "令牌".to_owned(),
        class_name: "Token".to_owned(),
        table_name: "sys_token".to_owned(),
        base_path: "entity::token".to_owned(),
    };
    let id_sys_token_attribute_info = AttributeInfo {
        column_name: "id_sys_token".to_owned(),
        name: "idSysToken".to_owned(),
        display_name: "令牌主属性".to_owned(),
        data_type: "InternalPK".to_owned(),
        ..Default::default()
    };
    let username_attribute_info = AttributeInfo {
        column_name: "username".to_owned(),
        name: "username".to_owned(),
        display_name: "用户名称".to_owned(),
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
    let create_time_attribute_info = AttributeInfo {
        column_name: "create_time".to_owned(),
        name: "createTime".to_owned(),
        display_name: "创建时间".to_owned(),
        data_type: "DateTime".to_owned(),
        ..Default::default()
    };
    let token_attribute_info = AttributeInfo {
        column_name: "token".to_owned(),
        name: "token".to_owned(),
        display_name: "令牌".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let expired_time_attribute_info = AttributeInfo {
        column_name: "expired_time".to_owned(),
        name: "expiredTime".to_owned(),
        display_name: "过期时间".to_owned(),
        data_type: "DateTime".to_owned(),
        ..Default::default()
    };
    let user_info_string_attribute_info = AttributeInfo {
        column_name: "user_info_string".to_owned(),
        name: "userInfoString".to_owned(),
        display_name: "用户信息序列化".to_owned(),
        data_type: "String".to_owned(),
        ..Default::default()
    };
    let entity_desc = EntityDesc {
      entity_info: entity_info,
      pk_attribute_info: id_sys_token_attribute_info.clone(),
      normal_fk_id_attribute_infos: vec![
      ],
      normal_fk_attribute_infos: vec![
      ],
      normal_children: vec![
      ],
      normal_one_2_one_children: vec![
      ],
      attribute_info_map: HashMap::from([
          ("idSysToken".to_owned(), id_sys_token_attribute_info),
          ("username".to_owned(), username_attribute_info),
          ("nickName".to_owned(), nick_name_attribute_info),
          ("createTime".to_owned(), create_time_attribute_info),
          ("token".to_owned(), token_attribute_info),
          ("expiredTime".to_owned(), expired_time_attribute_info),
          ("userInfoString".to_owned(), user_info_string_attribute_info),
      ]),
    };

    entity_desc
}
