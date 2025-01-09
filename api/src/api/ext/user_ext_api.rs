use actix_web::{error, post, web, Error, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use entity::entity::{
    component, component_module, dto_entity_collection, dto_module, entity_collection, sub_project,
};
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_service::{
    sea_orm::{EntityTrait, QueryFilter},
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*,
    dto::vo::{
        base::project_vo::ProjectVO,
        ext::project::{
            component::ProjectTreeVO as ComponentProectTreeVO,
            dto_collection::ProjectTreeVO as DtoProjectTreeVO, entity_collection::ProjectTreeVO,
        },
    },
    service::base::user_service::{UserQuery, UserMutation},
};
use tcdt_service::sea_orm::prelude::Expr;
use tcdt_service::util::dyn_query::md5;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ModifyPasswordParams {
    username: String,
    old_password: String,
    password: String,
}

#[tcdt_route(update_password)]
#[post("/user/updatePw")]
pub async fn update_password(
    _req: HttpRequest,
    data: web::Data<AppState>,
    modify_password_param: web::Json<ModifyPasswordParams>,
) -> Result<HttpResponse, Error> {
    let enable_change_password = TCDT_CONF.enable_change_password;
    if !enable_change_password {
        log::info!("enable_change_password value is false");
        let err = error::ErrorUnauthorized("enable_change_password value is false");
        return Err(err);
    }
    let conn = &data.conn;
    let modify_password_param = modify_password_param.into_inner();
    if modify_password_param.username == "" || modify_password_param.password == "" || modify_password_param.old_password == "" {
        return Err(error::ErrorInternalServerError("param is empty"));
    }

    let aq_condition = AqCondition::build_equal_condition("account", EFilterParam::String(Some(Box::new(modify_password_param.username.clone()))));

    let mut user = UserQuery::find_one_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or(error::ErrorInternalServerError("internal server error"))?;
    let security = TCDT_CONF.security;
    if security {
        if md5(&modify_password_param.old_password.to_uppercase()).to_uppercase() != user.user_pwd.unwrap_or_default() {
            return Err(error::ErrorInternalServerError("username or oldPassword error"));
        }
    }
    user.user_pwd = Some(md5(&modify_password_param.password).to_uppercase());
    let mut after_save = UserMutation::update_by_id(conn, user)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    after_save.user_pwd = None;
    Ok(HttpResponse::Ok().json(after_save))
}
