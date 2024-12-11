use crate::api::common::param::IdsParam;
use crate::app::AppState;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::agg::user_agg_po::UserAggPO, vo::agg::user_agg_vo::UserVO as UserAggVO},
    service::agg::user_agg_service::{UserAggMutation, UserAggQuery},
};

#[tcdt_route(save)]
#[post("/userAgg/save")]
pub async fn save(
    data: web::Data<AppState>,
    user_form: web::Json<UserAggPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = user_form.into_inner();

    let user_entity = UserAggMutation::save(conn, form).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;

    let user_vo = UserAggVO::convert(conn, Some(user_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(user_vo))
}

#[tcdt_route(get_by_id)]
#[get("/userAgg/getById/{id}")]
pub async fn get_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let user_entity = UserAggQuery::find_by_id(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let user_vo = UserAggVO::convert(conn, Some(user_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(user_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/userAgg/getById/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let user_list = UserAggQuery::find_by_ids(conn, ids).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let mut vos = vec![];
    for user_entity in user_list {
        let user_vo = UserAggVO::convert(conn, Some(user_entity))
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

#[tcdt_route(aq_page)]
#[post("/userAgg/aqPage")]
pub async fn aq_page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (users, num_items) = UserAggQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for user_entity in users {
        let user_vo = UserAggVO::convert(conn, Some(user_entity))
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

#[post("/userAgg/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let user_list = UserAggQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for user_entity in user_list {
        let user_vo = UserAggVO::convert(conn, Some(user_entity))
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
