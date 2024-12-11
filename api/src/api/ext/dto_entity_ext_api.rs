use std::collections::HashMap;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*,
    dto::vo::ext::dto_entity::{
        detail::DtoEntityVO as DetailDtoEntityVO, simple::DtoEntityVO as SimpleDtoEntityVO,
    },
    service::base::dto_entity_service::DtoEntityQuery,
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(get_dto_attr_by_ids)]
#[get("/dtoEntity/getDtoAttrByIds")]
pub async fn get_dto_attr_by_ids(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let ids_str = query_params.get("idDtoEntityList");
    if ids_str.is_none() {
        log::error!("idDtoEntityList not found");
        return Err(error::ErrorInternalServerError("idDtoEntityList not found"));
    }
    let ids = ids_str
        .unwrap()
        .split(",")
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();
    let dto_entity_list = DtoEntityQuery::find_by_ids(conn, ids).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let mut vos: Vec<DetailDtoEntityVO> = vec![];
    for dto_entity_entity in dto_entity_list {
        let dto_entity_vo = DetailDtoEntityVO::convert(conn, Some(dto_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_entity_vo) = dto_entity_vo {
            vos.push(dto_entity_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(aq_detail)]
#[post("/dtoEntity/aqDetail")]
pub async fn aq_detail(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let entities = DtoEntityQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<SimpleDtoEntityVO> = vec![];
    for entity_entity in entities {
        let entity_vo = SimpleDtoEntityVO::convert(conn, Some(entity_entity))
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
