use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*, dto::vo::ext::menu::menu_vo::MenuVO, service::base::menu_service::MenuQuery,
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(ext_get_by_id)]
#[get("/menu/extGetById/{id}")]
pub async fn ext_get_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let menu_entity = MenuQuery::find_by_id(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let menu_vo = MenuVO::convert(conn, Some(menu_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(menu_vo))
}
