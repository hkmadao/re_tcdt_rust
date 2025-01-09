use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "sys_token")]
pub struct Model {
    #[sea_orm(primary_key, comment = "令牌主属性")]
    pub id_sys_token: String,
    /// 用户名称:
    #[sea_orm(comment = "用户名称")]
    pub username: Option<String>,
    /// 昵称:
    #[sea_orm(comment = "昵称")]
    pub nick_name: Option<String>,
    /// 创建时间:
    #[sea_orm(comment = "创建时间")]
    pub create_time: Option<tcdt_common::chrono::DateTime<tcdt_common::chrono::Local>>,
    /// 令牌:
    #[sea_orm(comment = "令牌")]
    pub token: Option<String>,
    /// 过期时间:
    #[sea_orm(comment = "过期时间")]
    pub expired_time: Option<tcdt_common::chrono::DateTime<tcdt_common::chrono::Local>>,
    /// 用户信息序列化:
    #[sea_orm(comment = "用户信息序列化")]
    pub user_info_string: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(token_model: Model) -> ActiveModel {
    ActiveModel {
        id_sys_token: Set(token_model.id_sys_token.clone()),
        username: Set(token_model.username.clone()),
        nick_name: Set(token_model.nick_name.clone()),
        create_time: Set(token_model.create_time.clone()),
        token: Set(token_model.token.clone()),
        expired_time: Set(token_model.expired_time.clone()),
        user_info_string: Set(token_model.user_info_string.clone()),
    }
}