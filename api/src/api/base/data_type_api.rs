use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::data_type_po::DataTypePO, vo::base::data_type_vo::DataTypeVO},
    service::base::data_type_service::{DataTypeMutation, DataTypeQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/dataType/add")]
pub async fn add(
    data: web::Data<AppState>,
    data_type_form: web::Json<DataTypePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = data_type_form.into_inner();

    let data_type_save = DataTypeMutation::create(conn, form).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;

    let data_type_vo = DataTypeVO::convert(conn, Some(data_type_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(data_type_vo))
}

#[tcdt_route(update)]
#[post("/dataType/update")]
pub async fn update(
    data: web::Data<AppState>,
    data_type_form: web::Json<DataTypePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = data_type_form.into_inner();

    let data_type_save = DataTypeMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let data_type_vo = DataTypeVO::convert(conn, Some(data_type_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(data_type_vo))
}

#[tcdt_route(remove)]
#[post("/dataType/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    data_type_form: web::Json<DataTypePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = data_type_form.into_inner();

    let delete_result = DataTypeMutation::delete(conn, form).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/dataType/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    data_type_form: web::Json<Vec<DataTypePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = data_type_form.into_inner();

    let delete_result = DataTypeMutation::batch_delete(conn, po_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/dataType/getById/{id}")]
pub async fn get_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let data_type_entity = DataTypeQuery::find_by_id(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let data_type_vo = DataTypeVO::convert(conn, Some(data_type_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(data_type_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/dataType/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let data_type_list = DataTypeQuery::find_by_ids(conn, ids).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let mut vos: Vec<DataTypeVO> = vec![];
    for data_type_entity in data_type_list {
        let data_type_vo = DataTypeVO::convert(conn, Some(data_type_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(data_type_vo) = data_type_vo {
            vos.push(data_type_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/dataType/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (data_types, num_items) = DataTypeQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DataTypeVO> = vec![];
    for data_type_entity in data_types {
        let data_type_vo = DataTypeVO::convert(conn, Some(data_type_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(data_type_vo) = data_type_vo {
            vos.push(data_type_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/dataType/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let data_type_list = DataTypeQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for data_type_entity in data_type_list {
        let data_type_vo = DataTypeVO::convert(conn, Some(data_type_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(data_type_vo) = data_type_vo {
            vos.push(data_type_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}
