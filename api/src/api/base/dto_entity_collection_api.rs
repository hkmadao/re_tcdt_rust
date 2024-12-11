use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::dto_entity_collection_po::DtoEntityCollectionPO, vo::base::dto_entity_collection_vo::DtoEntityCollectionVO},
    service::base::dto_entity_collection_service::{DtoEntityCollectionMutation, DtoEntityCollectionQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/dtoEntityCollection/add")]
pub async fn add(
    data: web::Data<AppState>,
    dto_entity_collection_form: web::Json<DtoEntityCollectionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = dto_entity_collection_form.into_inner();

    let dto_entity_collection_save = DtoEntityCollectionMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let dto_entity_collection_vo = DtoEntityCollectionVO::convert(conn, Some(dto_entity_collection_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_entity_collection_vo))
}

#[tcdt_route(update)]
#[post("/dtoEntityCollection/update")]
pub async fn update(
    data: web::Data<AppState>,
    dto_entity_collection_form: web::Json<DtoEntityCollectionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_entity_collection_form.into_inner();

    let dto_entity_collection_save = DtoEntityCollectionMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dto_entity_collection_vo = DtoEntityCollectionVO::convert(conn, Some(dto_entity_collection_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_entity_collection_vo))
}

#[tcdt_route(remove)]
#[post("/dtoEntityCollection/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    dto_entity_collection_form: web::Json<DtoEntityCollectionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_entity_collection_form.into_inner();

    let delete_result = DtoEntityCollectionMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/dtoEntityCollection/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let dto_entity_collection_entity = DtoEntityCollectionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dto_entity_collection_vo = DtoEntityCollectionVO::convert(conn, Some(dto_entity_collection_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(dto_entity_collection_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/dtoEntityCollection/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let dto_entity_collection_list = DtoEntityCollectionQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<DtoEntityCollectionVO> = vec![];
    for dto_entity_collection_entity in dto_entity_collection_list {
        let dto_entity_collection_vo = DtoEntityCollectionVO::convert(conn, Some(dto_entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_entity_collection_vo) = dto_entity_collection_vo {
            vos.push(dto_entity_collection_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/dtoEntityCollection/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (dto_entity_collections, num_items) = DtoEntityCollectionQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DtoEntityCollectionVO> = vec![];
    for dto_entity_collection_entity in dto_entity_collections {
        let dto_entity_collection_vo = DtoEntityCollectionVO::convert(conn, Some(dto_entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_entity_collection_vo) = dto_entity_collection_vo {
            vos.push(dto_entity_collection_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/dtoEntityCollection/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let dto_entity_collection_list = DtoEntityCollectionQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for dto_entity_collection_entity in dto_entity_collection_list {
        let dto_entity_collection_vo = DtoEntityCollectionVO::convert(conn, Some(dto_entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_entity_collection_vo) = dto_entity_collection_vo {
            vos.push(dto_entity_collection_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}