use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::sub_project_po::SubProjectPO, vo::base::sub_project_vo::SubProjectVO},
    service::base::sub_project_service::{SubProjectMutation, SubProjectQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/subProject/add")]
pub async fn add(
    data: web::Data<AppState>,
    sub_project_form: web::Json<SubProjectPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = sub_project_form.into_inner();

    let sub_project_save = SubProjectMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let sub_project_vo = SubProjectVO::convert(conn, Some(sub_project_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(sub_project_vo))
}

#[tcdt_route(update)]
#[post("/subProject/update")]
pub async fn update(
    data: web::Data<AppState>,
    sub_project_form: web::Json<SubProjectPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = sub_project_form.into_inner();

    let sub_project_save = SubProjectMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let sub_project_vo = SubProjectVO::convert(conn, Some(sub_project_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(sub_project_vo))
}

#[tcdt_route(remove)]
#[post("/subProject/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    sub_project_form: web::Json<SubProjectPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = sub_project_form.into_inner();

    let delete_result = SubProjectMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/subProject/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let sub_project_entity = SubProjectQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let sub_project_vo = SubProjectVO::convert(conn, Some(sub_project_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(sub_project_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/subProject/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let sub_project_list = SubProjectQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<SubProjectVO> = vec![];
    for sub_project_entity in sub_project_list {
        let sub_project_vo = SubProjectVO::convert(conn, Some(sub_project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(sub_project_vo) = sub_project_vo {
            vos.push(sub_project_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/subProject/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (sub_projects, num_items) = SubProjectQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<SubProjectVO> = vec![];
    for sub_project_entity in sub_projects {
        let sub_project_vo = SubProjectVO::convert(conn, Some(sub_project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(sub_project_vo) = sub_project_vo {
            vos.push(sub_project_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/sub_project/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let sub_project_list = SubProjectQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for sub_project_entity in sub_project_list {
        let sub_project_vo = SubProjectVO::convert(conn, Some(sub_project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(sub_project_vo) = sub_project_vo {
            vos.push(sub_project_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}