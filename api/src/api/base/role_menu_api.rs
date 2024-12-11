use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::role_menu_po::RoleMenuPO, vo::base::role_menu_vo::RoleMenuVO},
    service::base::role_menu_service::{RoleMenuMutation, RoleMenuQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/roleMenu/add")]
pub async fn add(
    data: web::Data<AppState>,
    role_menu_form: web::Json<RoleMenuPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = role_menu_form.into_inner();

    let role_menu_save = RoleMenuMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let role_menu_vo = RoleMenuVO::convert(conn, Some(role_menu_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(role_menu_vo))
}

#[tcdt_route(update)]
#[post("/roleMenu/update")]
pub async fn update(
    data: web::Data<AppState>,
    role_menu_form: web::Json<RoleMenuPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = role_menu_form.into_inner();

    let role_menu_save = RoleMenuMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let role_menu_vo = RoleMenuVO::convert(conn, Some(role_menu_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(role_menu_vo))
}

#[tcdt_route(remove)]
#[post("/roleMenu/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    role_menu_form: web::Json<RoleMenuPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = role_menu_form.into_inner();

    let delete_result = RoleMenuMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/roleMenu/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    role_menu_form: web::Json<Vec<RoleMenuPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = role_menu_form.into_inner();

    let delete_result = RoleMenuMutation::batch_delete(conn, po_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/roleMenu/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let role_menu_entity = RoleMenuQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let role_menu_vo = RoleMenuVO::convert(conn, Some(role_menu_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(role_menu_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/roleMenu/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let role_menu_list = RoleMenuQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<RoleMenuVO> = vec![];
    for role_menu_entity in role_menu_list {
        let role_menu_vo = RoleMenuVO::convert(conn, Some(role_menu_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(role_menu_vo) = role_menu_vo {
            vos.push(role_menu_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/roleMenu/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (role_menus, num_items) = RoleMenuQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<RoleMenuVO> = vec![];
    for role_menu_entity in role_menus {
        let role_menu_vo = RoleMenuVO::convert(conn, Some(role_menu_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(role_menu_vo) = role_menu_vo {
            vos.push(role_menu_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/roleMenu/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let role_menu_list = RoleMenuQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for role_menu_entity in role_menu_list {
        let role_menu_vo = RoleMenuVO::convert(conn, Some(role_menu_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(role_menu_vo) = role_menu_vo {
            vos.push(role_menu_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}