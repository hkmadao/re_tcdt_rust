use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_entity_po::ComponentEntityPO, vo::base::component_entity_vo::ComponentEntityVO},
    service::base::component_entity_service::{ComponentEntityMutation, ComponentEntityQuery},
};

use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/componentEntity/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_entity_form: web::Json<ComponentEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_entity_form.into_inner();

    let component_entity_save = ComponentEntityMutation::create(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_entity_vo = ComponentEntityVO::convert(conn, Some(component_entity_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_entity_vo))
}

#[tcdt_route(update)]
#[post("/componentEntity/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_entity_form: web::Json<ComponentEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_entity_form.into_inner();

    let component_entity_save = ComponentEntityMutation::update_by_id(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_entity_vo = ComponentEntityVO::convert(conn, Some(component_entity_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_entity_vo))
}

#[tcdt_route(remove)]
#[post("/componentEntity/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_entity_form: web::Json<ComponentEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_entity_form.into_inner();

    let delete_result = ComponentEntityMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/componentEntity/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_entity_entity = ComponentEntityQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_entity_vo = ComponentEntityVO::convert(conn, Some(component_entity_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_entity_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/componentEntity/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let component_entity_list = ComponentEntityQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentEntityVO> = vec![];
    for component_entity_entity in component_entity_list {
        let component_entity_vo = ComponentEntityVO::convert(conn, Some(component_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_entity_vo) = component_entity_vo {
            vos.push(component_entity_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/componentEntity/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_entitys, num_items) = ComponentEntityQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentEntityVO> = vec![];
    for component_entity_entity in component_entitys {
        let component_entity_vo = ComponentEntityVO::convert(conn, Some(component_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_entity_vo) = component_entity_vo {
            vos.push(component_entity_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/componentEntity/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_entity_list = ComponentEntityQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_entity_entity in component_entity_list {
        let component_entity_vo = ComponentEntityVO::convert(conn, Some(component_entity_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_entity_vo) = component_entity_vo {
            vos.push(component_entity_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}