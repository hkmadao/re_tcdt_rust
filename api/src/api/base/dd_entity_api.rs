use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::dd_entity_po::DdEntityPO, vo::base::dd_entity_vo::DdEntityVO},
    service::base::dd_entity_service::{DdEntityMutation, DdEntityQuery},
};
use entity::entity::dd_entity;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/ddEntity/add")]
pub async fn add(
    data: web::Data<AppState>,
    dd_entity_form: web::Json<DdEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = dd_entity_form.into_inner();

    let dd_entity_model = DdEntityPO::convert_po_to_model(form);

    let dd_entity_save = DdEntityMutation::create(conn, dd_entity_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let dd_entity_vo = DdEntityVO::convert(conn, Some(dd_entity_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dd_entity_vo))
}

#[tcdt_route(update)]
#[post("/ddEntity/update")]
pub async fn update(
    data: web::Data<AppState>,
    dd_entity_form: web::Json<DdEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dd_entity_form.into_inner();

    let dd_entity_model = DdEntityPO::convert_po_to_model(form);

    let dd_entity_save = DdEntityMutation::update_by_id(conn, dd_entity_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dd_entity_vo = DdEntityVO::convert(conn, Some(dd_entity_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dd_entity_vo))
}

#[tcdt_route(remove)]
#[post("/ddEntity/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    dd_entity_form: web::Json<DdEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dd_entity_form.into_inner();

    let dd_entity_model = DdEntityPO::convert_po_to_model(form);

    let delete_result = DdEntityMutation::delete(conn, dd_entity_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/ddEntity/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    dd_entity_form: web::Json<Vec<DdEntityPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = dd_entity_form.into_inner();

    let mut model_list:Vec<dd_entity::Model>  = vec![];
    for po in po_list {
        model_list.push(DdEntityPO::convert_po_to_model(po));
    }
    
    let delete_result = DdEntityMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/ddEntity/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let dd_entity_entity = DdEntityQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dd_entity_vo = DdEntityVO::convert(conn, Some(dd_entity_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(dd_entity_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/ddEntity/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let dd_entity_list = DdEntityQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<DdEntityVO> = vec![];
    for dd_entity_entity in dd_entity_list {
        let dd_entity_vo = DdEntityVO::convert(conn, Some(dd_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_entity_vo) = dd_entity_vo {
            vos.push(dd_entity_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/ddEntity/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (dd_entitys, num_items) = DdEntityQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DdEntityVO> = vec![];
    for dd_entity_entity in dd_entitys {
        let dd_entity_vo = DdEntityVO::convert(conn, Some(dd_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_entity_vo) = dd_entity_vo {
            vos.push(dd_entity_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/ddEntity/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let dd_entity_list = DdEntityQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for dd_entity_entity in dd_entity_list {
        let dd_entity_vo = DdEntityVO::convert(conn, Some(dd_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dd_entity_vo) = dd_entity_vo {
            vos.push(dd_entity_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}