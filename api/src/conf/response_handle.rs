use std::future::{ready, Ready};

use actix_http::{header, StatusCode};
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use tcdt_service::common::result::AppResult;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct ResponseHandler;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S> Transform<S, ServiceRequest> for ResponseHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ResponseHandlerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ResponseHandlerMiddleware { service }))
    }
}

pub struct ResponseHandlerMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for ResponseHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("{:?}", req.path());
        log::info!("{:?}", req.method().clone());
        let _method = req.method().clone().to_string();

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

            if res.status() == StatusCode::INTERNAL_SERVER_ERROR {
                res.response_mut().headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/json"),
                );
                let st = res.response_mut().status_mut();
                *st = StatusCode::OK;

                let res = res.map_body(|_res_head, body| {
                    log::error!("{:?}", body);
                    let bytes_result = body.try_into_bytes();
                    if let Ok(bytes) = bytes_result {
                        let str_body = String::from_utf8_lossy(&bytes).into_owned();
                        let result = AppResult::<i32>::failed_msg(str_body);
                        let result_bytes = serde_json::to_vec(&result).unwrap();
                        let result_body = BoxBody::new(result_bytes);
                        return result_body;
                    }
                    let result = AppResult::<i32>::failed_msg("error".to_owned());
                    let result_bytes = serde_json::to_vec(&result).unwrap();
                    let result_body = BoxBody::new(result_bytes);
                    return result_body;
                });
                return Ok(res);
            }

            let res = res.map_body(|res_head, body| {
                if res_head.headers.get("content-type").is_some()
                    && res_head.headers.get("content-type").unwrap() == "application/json"
                {
                    let bytes_result = body.try_into_bytes();
                    if let Ok(bytes) = bytes_result {
                        let str_body = String::from_utf8_lossy(&bytes).into_owned();
                        let json_body: serde_json::Value = serde_json::from_str(&str_body).unwrap();
                        let result = AppResult::success(json_body);
                        let result_bytes = serde_json::to_vec(&result).unwrap();
                        let result_body = BoxBody::new(result_bytes);
                        return result_body;
                    }
                    let result = AppResult::<i32>::success_not_data();
                    let result_bytes = serde_json::to_vec(&result).unwrap();
                    let result_body = BoxBody::new(result_bytes);
                    return result_body;
                }
                return body;
            });
            Ok(res)
        })
    }
}
