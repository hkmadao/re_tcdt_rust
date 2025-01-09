use std::future::{ready, Future, Ready};
use std::rc::Rc;
use std::sync::Arc;
use actix_http::{header, HttpMessage, StatusCode};
use actix_web::{body::{BoxBody, MessageBody}, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error, web, Error};
use futures_util::future::LocalBoxFuture;
use tcdt_common::chrono::Utc;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_service::common::aq::{AqCondition, EFilterParam};
use tcdt_service::common::result::AppResult;
use tcdt_service::sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};
use tcdt_service::service::base::token_service::{TokenQuery, TokenMutation};
use crate::app::AppState;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SecurityHandler;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S> Transform<S, ServiceRequest> for SecurityHandler
where
    S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHandlerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHandlerMiddleware { service }))
    }
}

pub struct SecurityHandlerMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for SecurityHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        if req.headers().get(header::AUTHORIZATION).is_none() {
            log::info!("header authorization is empty");
            return Box::pin(async move {
                Err(error::ErrorUnauthorized("header authorization is empty"))
            });
        }
        let authorization_header = req.headers().get(header::AUTHORIZATION).clone().unwrap().clone();

        log::info!("{:?}", path);

        let web_data = req.app_data::<web::Data<AppState>>().unwrap().clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            if path == "/login" {
                let res = fut.await?;
                return Ok(res);
            }
            let security = TCDT_CONF.security;
            if !security {
                if path == "/user/updatePw" {
                    let res = fut.await?;
                    return Ok(res);
                }
            }
            if authorization_header.is_empty() {
                log::info!("header authorization is empty");
                let err = error::ErrorUnauthorized("header authorization is empty");
                return Err(err);
            }
            let token = authorization_header.to_str().unwrap();
            let aq_condition = AqCondition::build_equal_condition("token", EFilterParam::String(Some(Box::new(token.to_string()))));
            let db_conn = &web_data.conn;
            let mut token_entity = TokenQuery::find_one_by_condition(db_conn, aq_condition)
                .await
                .map_err(|err| {
                    log::error!("{:?}", err);
                    error::ErrorUnauthorized("Unauthorized")
                })?
                .ok_or(error::ErrorUnauthorized("Unauthorized"))?;

            if token_entity.expired_time.unwrap().lt(&tcdt_common::chrono::Local::now()) {
                log::info!("token expired");
                let err = error::ErrorUnauthorized("token expired");
                return Err(err);
            }
            token_entity.expired_time = Some(tcdt_common::chrono::Local::now() + tcdt_common::chrono::Duration::hours(1));
            TokenMutation::update_by_id(db_conn, token_entity)
                .await
                .map_err(|err| {
                    log::error!("{:?}", err);
                    error::ErrorUnauthorized("Unauthorized")
                })?;

            let res = fut.await?;
            return Ok(res);
        })
    }
}

