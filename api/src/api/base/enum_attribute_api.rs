use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::enum_attribute_po::EnumAttributePO, vo::base::enum_attribute_vo::EnumAttributeVO},
    service::base::enum_attribute_service::{EnumAttributeMutation, EnumAttributeQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/enumAttribute/add")]
pub async fn add(
    data: web::Data<AppState>,
    enum_attribute_form: web::Json<EnumAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = enum_attribute_form.into_inner();

    let enum_attribute_save = EnumAttributeMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let enum_attribute_vo = EnumAttributeVO::convert(conn, Some(enum_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(enum_attribute_vo))
}

#[tcdt_route(update)]
#[post("/enumAttribute/update")]
pub async fn update(
    data: web::Data<AppState>,
    enum_attribute_form: web::Json<EnumAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = enum_attribute_form.into_inner();

    let enum_attribute_save = EnumAttributeMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let enum_attribute_vo = EnumAttributeVO::convert(conn, Some(enum_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(enum_attribute_vo))
}

#[tcdt_route(remove)]
#[post("/enumAttribute/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    enum_attribute_form: web::Json<EnumAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = enum_attribute_form.into_inner();

    let delete_result = EnumAttributeMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/enumAttribute/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let enum_attribute_entity = EnumAttributeQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let enum_attribute_vo = EnumAttributeVO::convert(conn, Some(enum_attribute_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(enum_attribute_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/enumAttribute/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let enum_attribute_list = EnumAttributeQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<EnumAttributeVO> = vec![];
    for enum_attribute_entity in enum_attribute_list {
        let enum_attribute_vo = EnumAttributeVO::convert(conn, Some(enum_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(enum_attribute_vo) = enum_attribute_vo {
            vos.push(enum_attribute_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/enumAttribute/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (enum_attributes, num_items) = EnumAttributeQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<EnumAttributeVO> = vec![];
    for enum_attribute_entity in enum_attributes {
        let enum_attribute_vo = EnumAttributeVO::convert(conn, Some(enum_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(enum_attribute_vo) = enum_attribute_vo {
            vos.push(enum_attribute_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/enum_attribute/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let enum_attribute_list = EnumAttributeQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for enum_attribute_entity in enum_attribute_list {
        let enum_attribute_vo = EnumAttributeVO::convert(conn, Some(enum_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(enum_attribute_vo) = enum_attribute_vo {
            vos.push(enum_attribute_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}