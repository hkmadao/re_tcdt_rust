use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::user_po::UserPO, vo::base::user_vo::UserVO},
    service::base::user_service::{UserMutation, UserQuery},
};

use crate::app::AppState;

#[tcdt_route(add)]
#[post("/user/add")]
pub async fn add(
    data: web::Data<AppState>,
    user_form: web::Json<UserPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = user_form.into_inner();

    let user_save = UserMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let user_vo = UserVO::convert(conn, Some(user_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(user_vo))
}

#[tcdt_route(update)]
#[post("/user/update")]
pub async fn update(
    data: web::Data<AppState>,
    user_form: web::Json<UserPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = user_form.into_inner();

    let user_save = UserMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let user_vo = UserVO::convert(conn, Some(user_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(user_vo))
}

#[tcdt_route(remove)]
#[post("/user/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    user_form: web::Json<UserPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = user_form.into_inner();

    let delete_result = UserMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/user/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    user_form: web::Json<Vec<UserPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = user_form.into_inner();

    let delete_result = UserMutation::batch_delete(conn, po_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/user/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let user_entity = UserQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let user_vo = UserVO::convert(conn, Some(user_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(user_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/user/getByIds/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids = ids.into_inner();

    let ids = ids.split(",").map(|id| id.to_owned()).collect();

    let user_list = UserQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<UserVO> = vec![];
    for user_entity in user_list {
        let user_vo = UserVO::convert(conn, Some(user_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(user_vo) = user_vo {
            vos.push(user_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/user/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (users, num_items) = UserQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<UserVO> = vec![];
    for user_entity in users {
        let user_vo = UserVO::convert(conn, Some(user_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(user_vo) = user_vo {
            vos.push(user_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/user/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let user_list = UserQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for user_entity in user_list {
        let user_vo = UserVO::convert(conn, Some(user_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(user_vo) = user_vo {
            vos.push(user_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}