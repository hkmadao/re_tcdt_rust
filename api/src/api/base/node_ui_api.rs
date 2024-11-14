use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::node_ui_po::NodeUiPO, vo::base::node_ui_vo::NodeUiVO},
    service::base::node_ui_service::{NodeUiMutation, NodeUiQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/nodeUi/add")]
pub async fn add(
    data: web::Data<AppState>,
    node_ui_form: web::Json<NodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = node_ui_form.into_inner();

    let node_ui_save = NodeUiMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let node_ui_vo = NodeUiVO::convert(conn, Some(node_ui_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(node_ui_vo))
}

#[tcdt_route(update)]
#[post("/nodeUi/update")]
pub async fn update(
    data: web::Data<AppState>,
    node_ui_form: web::Json<NodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = node_ui_form.into_inner();

    let node_ui_save = NodeUiMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let node_ui_vo = NodeUiVO::convert(conn, Some(node_ui_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(node_ui_vo))
}

#[tcdt_route(remove)]
#[post("/nodeUi/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    node_ui_form: web::Json<NodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = node_ui_form.into_inner();

    let delete_result = NodeUiMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/nodeUi/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let node_ui_entity = NodeUiQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let node_ui_vo = NodeUiVO::convert(conn, Some(node_ui_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(node_ui_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/nodeUi/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let node_ui_list = NodeUiQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<NodeUiVO> = vec![];
    for node_ui_entity in node_ui_list {
        let node_ui_vo = NodeUiVO::convert(conn, Some(node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(node_ui_vo) = node_ui_vo {
            vos.push(node_ui_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/nodeUi/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (node_uis, num_items) = NodeUiQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<NodeUiVO> = vec![];
    for node_ui_entity in node_uis {
        let node_ui_vo = NodeUiVO::convert(conn, Some(node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(node_ui_vo) = node_ui_vo {
            vos.push(node_ui_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/node_ui/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let node_ui_list = NodeUiQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for node_ui_entity in node_ui_list {
        let node_ui_vo = NodeUiVO::convert(conn, Some(node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(node_ui_vo) = node_ui_vo {
            vos.push(node_ui_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}