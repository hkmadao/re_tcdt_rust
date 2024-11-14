use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*,
    dto::vo::{
        base::dd_entity_vo::DdEntityVO as DefaultEntityVO,
        ext::entity::detail::DdEntityVO as DetailEntityVO,
    },
    service::base::dd_entity_service::DdEntityQuery,
};

use crate::app::AppState;

#[tcdt_route(get_detail_by_entity_id)]
#[get("/entity/getDetailByEntityId")]
pub async fn get_detail_by_entity_id(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let ids_str = query_params.get("idEntity");
    if ids_str.is_none() {
        log::error!("idEntity not found");
        return Err(error::ErrorInternalServerError("idEntity not found"));
    }
    let id = ids_str.unwrap().to_string();

    let dd_entity_entity = DdEntityQuery::find_by_id(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let dd_entity_vo = DetailEntityVO::convert(conn, Some(dd_entity_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(dd_entity_vo))
}

#[tcdt_route(get_detail_by_entity_ids)]
#[get("/entity/getDetailByEntityIds")]
pub async fn get_detail_by_entity_ids(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let ids_str = query_params.get("idEntityList");
    if ids_str.is_none() {
        log::error!("idEntityList not found");
        return Err(error::ErrorInternalServerError("idEntityList not found"));
    }
    let ids = ids_str
        .unwrap()
        .split(",")
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();

    let dd_entity_list = DdEntityQuery::find_by_ids(conn, ids).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let mut vos: Vec<DetailEntityVO> = vec![];
    for dd_entity_entity in dd_entity_list {
        let dd_entity_vo = DetailEntityVO::convert(conn, Some(dd_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_entity_vo) = dd_entity_vo {
            vos.push(dd_entity_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(aq_detail)]
#[post("/entity/aqDetail")]
pub async fn aq_detail(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let entities = DdEntityQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DefaultEntityVO> = vec![];
    for entity_entity in entities {
        let entity_vo = DefaultEntityVO::convert(conn, Some(entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_vo) = entity_vo {
            vos.push(entity_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}
