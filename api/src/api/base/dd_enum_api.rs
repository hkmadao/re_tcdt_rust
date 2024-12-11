use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::dd_enum_po::DdEnumPO, vo::base::dd_enum_vo::DdEnumVO},
    service::base::dd_enum_service::{DdEnumMutation, DdEnumQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/ddEnum/add")]
pub async fn add(
    data: web::Data<AppState>,
    dd_enum_form: web::Json<DdEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = dd_enum_form.into_inner();

    let dd_enum_save = DdEnumMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let dd_enum_vo = DdEnumVO::convert(conn, Some(dd_enum_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dd_enum_vo))
}

#[tcdt_route(update)]
#[post("/ddEnum/update")]
pub async fn update(
    data: web::Data<AppState>,
    dd_enum_form: web::Json<DdEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dd_enum_form.into_inner();

    let dd_enum_save = DdEnumMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dd_enum_vo = DdEnumVO::convert(conn, Some(dd_enum_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dd_enum_vo))
}

#[tcdt_route(remove)]
#[post("/ddEnum/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    dd_enum_form: web::Json<DdEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dd_enum_form.into_inner();

    let delete_result = DdEnumMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/ddEnum/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let dd_enum_entity = DdEnumQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dd_enum_vo = DdEnumVO::convert(conn, Some(dd_enum_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(dd_enum_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/ddEnum/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let dd_enum_list = DdEnumQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<DdEnumVO> = vec![];
    for dd_enum_entity in dd_enum_list {
        let dd_enum_vo = DdEnumVO::convert(conn, Some(dd_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_enum_vo) = dd_enum_vo {
            vos.push(dd_enum_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/ddEnum/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (dd_enums, num_items) = DdEnumQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DdEnumVO> = vec![];
    for dd_enum_entity in dd_enums {
        let dd_enum_vo = DdEnumVO::convert(conn, Some(dd_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_enum_vo) = dd_enum_vo {
            vos.push(dd_enum_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/ddEnum/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let dd_enum_list = DdEnumQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for dd_enum_entity in dd_enum_list {
        let dd_enum_vo = DdEnumVO::convert(conn, Some(dd_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_enum_vo) = dd_enum_vo {
            vos.push(dd_enum_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}