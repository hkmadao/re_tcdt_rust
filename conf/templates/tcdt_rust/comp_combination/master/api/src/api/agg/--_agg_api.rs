use crate::api::common::param::IdsParam;
use crate::app::AppState;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::agg::{{ rootInfo.snakeCaseName }}_agg_po::{{ rootInfo.pascalCaseName }}AggPO, vo::agg::{{ rootInfo.snakeCaseName }}_agg_vo::{{ rootInfo.pascalCaseName }}VO as {{ rootInfo.pascalCaseName }}AggVO},
    service::agg::{{ rootInfo.snakeCaseName }}_agg_service::{
        {{ rootInfo.pascalCaseName }}AggMutation, 
        {{ rootInfo.pascalCaseName }}AggQuery
    },
};

#[tcdt_route(save)]
#[post("/{{ rootInfo.snakeCaseName }}Agg/save")]
pub async fn save(
    data: web::Data<AppState>,
    {{ rootInfo.snakeCaseName }}_form: web::Json<{{ rootInfo.pascalCaseName }}AggPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = {{ rootInfo.snakeCaseName }}_form.into_inner();

    let {{ rootInfo.snakeCaseName }}_entity = {{ rootInfo.pascalCaseName }}AggMutation::save(conn, form).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;

    let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}AggVO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json({{ rootInfo.snakeCaseName }}_vo))
}

#[tcdt_route(get_by_id)]
#[get("/{{ rootInfo.snakeCaseName }}Agg/getById/{id}")]
pub async fn get_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let {{ rootInfo.snakeCaseName }}_entity = {{ rootInfo.pascalCaseName }}AggQuery::find_by_id(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}AggVO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json({{ rootInfo.snakeCaseName }}_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/{{ rootInfo.snakeCaseName }}Agg/getById/{ids}")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let {{ rootInfo.snakeCaseName }}_list = {{ rootInfo.pascalCaseName }}AggQuery::find_by_ids(conn, ids).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let mut vos = vec![];
    for {{ rootInfo.snakeCaseName }}_entity in {{ rootInfo.snakeCaseName }}_list {
        let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}AggVO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some({{ rootInfo.snakeCaseName }}_vo) = {{ rootInfo.snakeCaseName }}_vo {
            vos.push({{ rootInfo.snakeCaseName }}_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(aq_page)]
#[post("/{{ rootInfo.snakeCaseName }}Agg/aqPage")]
pub async fn aq_page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let ({{ rootInfo.snakeCaseName }}s, num_items) = {{ rootInfo.pascalCaseName }}AggQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for {{ rootInfo.snakeCaseName }}_entity in {{ rootInfo.snakeCaseName }}s {
        let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}AggVO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some({{ rootInfo.snakeCaseName }}_vo) = {{ rootInfo.snakeCaseName }}_vo {
            vos.push({{ rootInfo.snakeCaseName }}_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/{{ rootInfo.snakeCaseName }}Agg/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let {{ rootInfo.snakeCaseName }}_list = {{ rootInfo.pascalCaseName }}AggQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for {{ rootInfo.snakeCaseName }}_entity in {{ rootInfo.snakeCaseName }}_list {
        let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}AggVO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some({{ rootInfo.snakeCaseName }}_vo) = {{ rootInfo.snakeCaseName }}_vo {
            vos.push({{ rootInfo.snakeCaseName }}_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}
