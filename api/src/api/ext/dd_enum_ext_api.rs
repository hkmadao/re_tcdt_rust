use std::collections::HashMap;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*,
    dto::vo::ext::dd_enum::{detail::DdEnumVO as DetailEnumVO, simple::DdEnumVO as SimpleEnumVO},
    service::base::dd_enum_service::DdEnumQuery,
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(get_detail_by_enum_ids)]
#[get("/ddEnum/getDetailByEnumIds")]
pub async fn get_detail_by_enum_ids(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let ids_str = query_params.get("idEnumList");
    if ids_str.is_none() {
        log::error!("idEnumList not found");
        return Err(error::ErrorInternalServerError("idEnumList not found"));
    }
    let ids = ids_str
        .unwrap()
        .split(",")
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();

    let dd_enum_list = DdEnumQuery::find_by_ids(conn, ids).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let mut vos: Vec<DetailEnumVO> = vec![];
    for dd_enum_entity in dd_enum_list {
        let dd_entity_vo = DetailEnumVO::convert(conn, Some(dd_enum_entity))
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
#[post("/ddEnum/aqDetail")]
pub async fn aq_detail(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let enums = DdEnumQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<SimpleEnumVO> = vec![];
    for enum_entity in enums {
        let enum_vo = SimpleEnumVO::convert(conn, Some(enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_vo) = enum_vo {
            vos.push(entity_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(ed)]
#[post("/ddEnum/ed")]
pub async fn ed(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let enums = DdEnumQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DetailEnumVO> = vec![];
    for enum_entity in enums {
        let enum_vo = DetailEnumVO::convert(conn, Some(enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_vo) = enum_vo {
            vos.push(entity_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}
