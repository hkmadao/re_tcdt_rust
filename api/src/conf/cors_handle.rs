use std::future::{ready, Future, Ready};
use std::rc::Rc;
use std::sync::Arc;
use actix_http::{header, HttpMessage, StatusCode};
use actix_web::{body::{BoxBody, MessageBody}, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, web, Error};
use futures_util::future::LocalBoxFuture;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_service::common::result::AppResult;
use tcdt_service::sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};
use crate::app::AppState;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct CorsHandler;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S> Transform<S, ServiceRequest> for CorsHandler
where
    S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsHandlerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorsHandlerMiddleware { service }))
    }
}

pub struct CorsHandlerMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for CorsHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<BoxBody>, Error=Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let fut = self.service.call(req);

        Box::pin(async move {

            let mut res = fut.await?;

            //跨域配置
            res.response_mut().headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::HeaderValue::from_static("*"),
            );
            res.response_mut().headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                header::HeaderValue::from_static("false"),
            );
            res.response_mut().headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                header::HeaderValue::from_static("*"),
            );
            res.response_mut().headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                header::HeaderValue::from_static("*"),
            );
            res.response_mut().headers_mut().insert(
                header::ACCESS_CONTROL_EXPOSE_HEADERS,
                header::HeaderValue::from_static("*"),
            );

            Ok(res)
        })
    }
}

