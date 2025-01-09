use std::ops::Add;
use actix_http::header;
use actix_web::{error, post, web, Error, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use entity::entity::token;
use tcdt_service::{
    sea_orm::{EntityTrait, QueryFilter},
};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*,
    service::base::{user_service::{UserQuery, UserMutation}, token_service::{TokenQuery, TokenMutation}},
};
use tcdt_service::sea_orm::prelude::Expr;
use tcdt_service::util::dyn_query::md5;
use tcdt_service::util::id_util::{generate_id, generate_password};
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct LoginParams {
    username: String,
    password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct LoginResultParams {
    nick_name: String,
    username: String,
    token: String,
    expired_time: tcdt_common::chrono::DateTime<tcdt_common::chrono::Local>,
}

#[tcdt_route(login)]
#[post("/login")]
pub async fn login(
    _req: HttpRequest,
    data: web::Data<AppState>,
    login_params: web::Json<LoginParams>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let login_params = login_params.into_inner();
    if login_params.username == "" || login_params.password == "" {
        return Err(error::ErrorInternalServerError("param is empty"));
    }

    let aq_condition = AqCondition::build_equal_condition("account", EFilterParam::String(Some(Box::new(login_params.username.clone()))));

    let user = UserQuery::find_one_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or(error::ErrorInternalServerError("internal server error"))?;
    if md5(&login_params.password.to_uppercase()).to_uppercase() != user.user_pwd.unwrap_or_default() {
        return Err(error::ErrorInternalServerError("username or password error"));
    }
    let token = generate_password(32);
    let expired_time = tcdt_common::chrono::Local::now() + tcdt_common::chrono::Duration::hours(1);
    let token_model = token::Model {
        id_sys_token: generate_id(),
        username: user.account.clone(),
        nick_name: user.nick_name.clone(),
        create_time: Some(tcdt_common::chrono::Local::now()),
        token: Some(token.clone()),
        expired_time: Some(expired_time),
        user_info_string: None,
    };
    TokenMutation::create(conn, token_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let result = LoginResultParams {
        nick_name: user.nick_name.unwrap_or_default(),
        username: user.account.unwrap_or_default(),
        token: token.clone(),
        expired_time: tcdt_common::chrono::Local::now() + tcdt_common::chrono::Duration::hours(1),
    };
    Ok(HttpResponse::Ok().json(result))
}

#[tcdt_route(logout)]
#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if req.headers().get(header::AUTHORIZATION).is_none() {
        log::info!("header authorization is empty");
        return Err(error::ErrorUnauthorized("header authorization is empty"));
    }
    let authorization_header = req.headers().get(header::AUTHORIZATION).clone().unwrap().clone();
    if authorization_header.is_empty() {
        log::info!("header authorization is empty");
        let err = error::ErrorUnauthorized("header authorization is empty");
        return Err(err);
    }
    let token = authorization_header.to_str().unwrap();
    let aq_condition = AqCondition::build_equal_condition("token", EFilterParam::String(Some(Box::new(token.to_string()))));
    let db_conn = &data.conn;
    let token_entity = TokenQuery::find_one_by_condition(db_conn, aq_condition)
        .await
        .map_err(|err| {
            log::error!("{:?}", err);
            error::ErrorInternalServerError("internal server error")
        })?;
    if let Some(token_entity) = token_entity {
        TokenMutation::delete(db_conn, token_entity)
            .await
            .map_err(|err| {
                log::error!("{:?}", err);
                error::ErrorInternalServerError("internal server error")
            })?;
    }
    Ok(HttpResponse::Ok().body(""))
}
