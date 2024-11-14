use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_module_po::ComponentModulePO, vo::base::component_module_vo::ComponentModuleVO},
    service::base::component_module_service::{ComponentModuleMutation, ComponentModuleQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/componentModule/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_module_form: web::Json<ComponentModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_module_form.into_inner();

    let component_module_save = ComponentModuleMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_module_vo))
}

#[tcdt_route(update)]
#[post("/componentModule/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_module_form: web::Json<ComponentModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_module_form.into_inner();

    let component_module_save = ComponentModuleMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_module_vo))
}

#[tcdt_route(remove)]
#[post("/componentModule/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_module_form: web::Json<ComponentModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_module_form.into_inner();

    let delete_result = ComponentModuleMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/componentModule/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_module_entity = ComponentModuleQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_module_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/componentModule/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let component_module_list = ComponentModuleQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentModuleVO> = vec![];
    for component_module_entity in component_module_list {
        let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_module_vo) = component_module_vo {
            vos.push(component_module_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/componentModule/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_modules, num_items) = ComponentModuleQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentModuleVO> = vec![];
    for component_module_entity in component_modules {
        let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_module_vo) = component_module_vo {
            vos.push(component_module_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/component_module/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_module_list = ComponentModuleQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_module_entity in component_module_list {
        let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_module_vo) = component_module_vo {
            vos.push(component_module_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}