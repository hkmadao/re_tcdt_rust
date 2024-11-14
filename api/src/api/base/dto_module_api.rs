use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::dto_module_po::DtoModulePO, vo::base::dto_module_vo::DtoModuleVO},
    service::base::dto_module_service::{DtoModuleMutation, DtoModuleQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/dtoModule/add")]
pub async fn add(
    data: web::Data<AppState>,
    dto_module_form: web::Json<DtoModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = dto_module_form.into_inner();

    let dto_module_save = DtoModuleMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let dto_module_vo = DtoModuleVO::convert(conn, Some(dto_module_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_module_vo))
}

#[tcdt_route(update)]
#[post("/dtoModule/update")]
pub async fn update(
    data: web::Data<AppState>,
    dto_module_form: web::Json<DtoModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_module_form.into_inner();

    let dto_module_save = DtoModuleMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dto_module_vo = DtoModuleVO::convert(conn, Some(dto_module_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_module_vo))
}

#[tcdt_route(remove)]
#[post("/dtoModule/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    dto_module_form: web::Json<DtoModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_module_form.into_inner();

    let delete_result = DtoModuleMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/dtoModule/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let dto_module_entity = DtoModuleQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dto_module_vo = DtoModuleVO::convert(conn, Some(dto_module_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(dto_module_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/dtoModule/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let dto_module_list = DtoModuleQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<DtoModuleVO> = vec![];
    for dto_module_entity in dto_module_list {
        let dto_module_vo = DtoModuleVO::convert(conn, Some(dto_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_module_vo) = dto_module_vo {
            vos.push(dto_module_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/dtoModule/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (dto_modules, num_items) = DtoModuleQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DtoModuleVO> = vec![];
    for dto_module_entity in dto_modules {
        let dto_module_vo = DtoModuleVO::convert(conn, Some(dto_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_module_vo) = dto_module_vo {
            vos.push(dto_module_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/dto_module/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let dto_module_list = DtoModuleQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for dto_module_entity in dto_module_list {
        let dto_module_vo = DtoModuleVO::convert(conn, Some(dto_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_module_vo) = dto_module_vo {
            vos.push(dto_module_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}