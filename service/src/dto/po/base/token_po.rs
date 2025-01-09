use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{
    token,
};
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
// use sea_orm::prelude::Json;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tcdt_macro::ParamObjectCud;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ParamObjectCud)]
#[serde(rename_all = "camelCase")]
pub struct TokenPO {
    #[tcdt_po(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_po(po_primary_key)]
    #[serde(default)]
    pub id_sys_token: String,
    /// 用户名称:
    #[serde(default)]
    pub username: Option<String>,
    /// 昵称:
    #[serde(default)]
    pub nick_name: Option<String>,
    /// 创建时间:
    #[serde(default)]
    pub create_time: Option<tcdt_common::chrono::DateTime<tcdt_common::chrono::Local>>,
    /// 令牌:
    #[serde(default)]
    pub token: Option<String>,
    /// 过期时间:
    #[serde(default)]
    pub expired_time: Option<tcdt_common::chrono::DateTime<tcdt_common::chrono::Local>>,
    /// 用户信息序列化:
    #[serde(default)]
    pub user_info_string: Option<String>,
}
