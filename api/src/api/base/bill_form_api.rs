use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::bill_form_po::BillFormPO, vo::base::bill_form_vo::BillFormVO},
    service::base::bill_form_service::{BillFormMutation, BillFormQuery},
};
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/billForm/add")]
pub async fn add(
    data: web::Data<AppState>,
    bill_form_form: web::Json<BillFormPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = bill_form_form.into_inner();

    let bill_form_save = BillFormMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let bill_form_vo = BillFormVO::convert(conn, Some(bill_form_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(bill_form_vo))
}

#[tcdt_route(update)]
#[post("/billForm/update")]
pub async fn update(
    data: web::Data<AppState>,
    bill_form_form: web::Json<BillFormPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = bill_form_form.into_inner();

    let bill_form_save = BillFormMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let bill_form_vo = BillFormVO::convert(conn, Some(bill_form_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(bill_form_vo))
}

#[tcdt_route(remove)]
#[post("/billForm/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    bill_form_form: web::Json<BillFormPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = bill_form_form.into_inner();

    let delete_result = BillFormMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/billForm/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let bill_form_entity = BillFormQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let bill_form_vo = BillFormVO::convert(conn, Some(bill_form_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(bill_form_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/billForm/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let bill_form_list = BillFormQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<BillFormVO> = vec![];
    for bill_form_entity in bill_form_list {
        let bill_form_vo = BillFormVO::convert(conn, Some(bill_form_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(bill_form_vo) = bill_form_vo {
            vos.push(bill_form_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/billForm/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (bill_forms, num_items) = BillFormQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<BillFormVO> = vec![];
    for bill_form_entity in bill_forms {
        let bill_form_vo = BillFormVO::convert(conn, Some(bill_form_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(bill_form_vo) = bill_form_vo {
            vos.push(bill_form_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/billForm/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let bill_form_list = BillFormQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for bill_form_entity in bill_form_list {
        let bill_form_vo = BillFormVO::convert(conn, Some(bill_form_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(bill_form_vo) = bill_form_vo {
            vos.push(bill_form_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}