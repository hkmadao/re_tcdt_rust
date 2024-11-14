use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::role_po::RolePO, vo::base::role_vo::RoleVO},
    service::base::role_service::{RoleMutation, RoleQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/role/add")]
pub async fn add(
    data: web::Data<AppState>,
    role_form: web::Json<RolePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = role_form.into_inner();

    let role_save = RoleMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let role_vo = RoleVO::convert(conn, Some(role_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(role_vo))
}

#[tcdt_route(update)]
#[post("/role/update")]
pub async fn update(
    data: web::Data<AppState>,
    role_form: web::Json<RolePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = role_form.into_inner();

    let role_save = RoleMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let role_vo = RoleVO::convert(conn, Some(role_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(role_vo))
}

#[tcdt_route(remove)]
#[post("/role/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    role_form: web::Json<RolePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = role_form.into_inner();

    let delete_result = RoleMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/role/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    role_form: web::Json<Vec<RolePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = role_form.into_inner();

    let delete_result = RoleMutation::batch_delete(conn, po_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/role/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let role_entity = RoleQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let role_vo = RoleVO::convert(conn, Some(role_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(role_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/role/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let role_list = RoleQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<RoleVO> = vec![];
    for role_entity in role_list {
        let role_vo = RoleVO::convert(conn, Some(role_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(role_vo) = role_vo {
            vos.push(role_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/role/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (roles, num_items) = RoleQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<RoleVO> = vec![];
    for role_entity in roles {
        let role_vo = RoleVO::convert(conn, Some(role_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(role_vo) = role_vo {
            vos.push(role_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/role/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let role_list = RoleQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for role_entity in role_list {
        let role_vo = RoleVO::convert(conn, Some(role_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(role_vo) = role_vo {
            vos.push(role_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}