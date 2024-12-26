use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::query_po::QueryPO, vo::base::query_vo::QueryVO},
    service::base::query_service::{QueryMutation, QueryQuery},
};
use entity::entity::query;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/query/add")]
pub async fn add(
    data: web::Data<AppState>,
    query_form: web::Json<QueryPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = query_form.into_inner();

    let query_model = QueryPO::convert_po_to_model(form);

    let query_save = QueryMutation::create(conn, query_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let query_vo = QueryVO::convert(conn, Some(query_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(query_vo))
}

#[tcdt_route(update)]
#[post("/query/update")]
pub async fn update(
    data: web::Data<AppState>,
    query_form: web::Json<QueryPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = query_form.into_inner();

    let query_model = QueryPO::convert_po_to_model(form);

    let query_save = QueryMutation::update_by_id(conn, query_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let query_vo = QueryVO::convert(conn, Some(query_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(query_vo))
}

#[tcdt_route(remove)]
#[post("/query/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    query_form: web::Json<QueryPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = query_form.into_inner();

    let query_model = QueryPO::convert_po_to_model(form);

    let delete_result = QueryMutation::delete(conn, query_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/query/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    query_form: web::Json<Vec<QueryPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = query_form.into_inner();

    let mut model_list:Vec<query::Model>  = vec![];
    for po in po_list {
        model_list.push(QueryPO::convert_po_to_model(po));
    }
    
    let delete_result = QueryMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/query/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let query_entity = QueryQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let query_vo = QueryVO::convert(conn, Some(query_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(query_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/query/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let query_list = QueryQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<QueryVO> = vec![];
    for query_entity in query_list {
        let query_vo = QueryVO::convert(conn, Some(query_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(query_vo) = query_vo {
            vos.push(query_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/query/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (querys, num_items) = QueryQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<QueryVO> = vec![];
    for query_entity in querys {
        let query_vo = QueryVO::convert(conn, Some(query_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(query_vo) = query_vo {
            vos.push(query_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/query/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let query_list = QueryQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for query_entity in query_list {
        let query_vo = QueryVO::convert(conn, Some(query_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(query_vo) = query_vo {
            vos.push(query_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}