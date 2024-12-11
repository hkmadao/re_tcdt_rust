use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::dto_enum_po::DtoEnumPO, vo::base::dto_enum_vo::DtoEnumVO},
    service::base::dto_enum_service::{DtoEnumMutation, DtoEnumQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/dtoEnum/add")]
pub async fn add(
    data: web::Data<AppState>,
    dto_enum_form: web::Json<DtoEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = dto_enum_form.into_inner();

    let dto_enum_save = DtoEnumMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let dto_enum_vo = DtoEnumVO::convert(conn, Some(dto_enum_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_enum_vo))
}

#[tcdt_route(update)]
#[post("/dtoEnum/update")]
pub async fn update(
    data: web::Data<AppState>,
    dto_enum_form: web::Json<DtoEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_enum_form.into_inner();

    let dto_enum_save = DtoEnumMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dto_enum_vo = DtoEnumVO::convert(conn, Some(dto_enum_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_enum_vo))
}

#[tcdt_route(remove)]
#[post("/dtoEnum/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    dto_enum_form: web::Json<DtoEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_enum_form.into_inner();

    let delete_result = DtoEnumMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/dtoEnum/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let dto_enum_entity = DtoEnumQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dto_enum_vo = DtoEnumVO::convert(conn, Some(dto_enum_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(dto_enum_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/dtoEnum/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let dto_enum_list = DtoEnumQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<DtoEnumVO> = vec![];
    for dto_enum_entity in dto_enum_list {
        let dto_enum_vo = DtoEnumVO::convert(conn, Some(dto_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_enum_vo) = dto_enum_vo {
            vos.push(dto_enum_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/dtoEnum/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (dto_enums, num_items) = DtoEnumQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DtoEnumVO> = vec![];
    for dto_enum_entity in dto_enums {
        let dto_enum_vo = DtoEnumVO::convert(conn, Some(dto_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_enum_vo) = dto_enum_vo {
            vos.push(dto_enum_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/dtoEnum/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let dto_enum_list = DtoEnumQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for dto_enum_entity in dto_enum_list {
        let dto_enum_vo = DtoEnumVO::convert(conn, Some(dto_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_enum_vo) = dto_enum_vo {
            vos.push(dto_enum_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}