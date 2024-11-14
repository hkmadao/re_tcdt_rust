// use actix_http::{header, StatusCode};
// use actix_web::{body::BoxBody, dev, middleware::ErrorHandlerResponse, Result};
// use tcdt_service::common::result::AppNotDataResult;
// 
// pub fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
//     res.response_mut().headers_mut().insert(
//         header::CONTENT_TYPE,
//         header::HeaderValue::from_static("application/json"),
//     );
//     let st = res.response_mut().status_mut();
//     *st = StatusCode::OK;
// 
//     let res = res.map_body(|_res_head, _body| {
//         let result = AppNotDataResult::failed("".to_owned());
//         let result_bytes = serde_json::to_vec(&result).unwrap();
//         let result_body = BoxBody::new(result_bytes);
//         return result_body;
//     });
//     Ok(ErrorHandlerResponse::Response(res.map_into_right_body()))
// }
