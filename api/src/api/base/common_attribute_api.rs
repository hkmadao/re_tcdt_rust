use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::common_attribute_po::CommonAttributePO, vo::base::common_attribute_vo::CommonAttributeVO},
    service::base::common_attribute_service::{CommonAttributeMutation, CommonAttributeQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/commonAttribute/add")]
pub async fn add(
    data: web::Data<AppState>,
    common_attribute_form: web::Json<CommonAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = common_attribute_form.into_inner();

    let common_attribute_save = CommonAttributeMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let common_attribute_vo = CommonAttributeVO::convert(conn, Some(common_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(common_attribute_vo))
}

#[tcdt_route(update)]
#[post("/commonAttribute/update")]
pub async fn update(
    data: web::Data<AppState>,
    common_attribute_form: web::Json<CommonAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = common_attribute_form.into_inner();

    let common_attribute_save = CommonAttributeMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let common_attribute_vo = CommonAttributeVO::convert(conn, Some(common_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(common_attribute_vo))
}

#[tcdt_route(remove)]
#[post("/commonAttribute/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    common_attribute_form: web::Json<CommonAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = common_attribute_form.into_inner();

    let delete_result = CommonAttributeMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/commonAttribute/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    common_attribute_form: web::Json<Vec<CommonAttributePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = common_attribute_form.into_inner();

    let delete_result = CommonAttributeMutation::batch_delete(conn, po_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/commonAttribute/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let common_attribute_entity = CommonAttributeQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let common_attribute_vo = CommonAttributeVO::convert(conn, Some(common_attribute_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(common_attribute_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/commonAttribute/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let common_attribute_list = CommonAttributeQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<CommonAttributeVO> = vec![];
    for common_attribute_entity in common_attribute_list {
        let common_attribute_vo = CommonAttributeVO::convert(conn, Some(common_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(common_attribute_vo) = common_attribute_vo {
            vos.push(common_attribute_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/commonAttribute/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (common_attributes, num_items) = CommonAttributeQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<CommonAttributeVO> = vec![];
    for common_attribute_entity in common_attributes {
        let common_attribute_vo = CommonAttributeVO::convert(conn, Some(common_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(common_attribute_vo) = common_attribute_vo {
            vos.push(common_attribute_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/commonAttribute/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let common_attribute_list = CommonAttributeQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for common_attribute_entity in common_attribute_list {
        let common_attribute_vo = CommonAttributeVO::convert(conn, Some(common_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(common_attribute_vo) = common_attribute_vo {
            vos.push(common_attribute_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}