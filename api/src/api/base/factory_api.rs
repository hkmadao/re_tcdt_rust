use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::factory_po::FactoryPO, vo::base::factory_vo::FactoryVO},
    service::base::factory_service::{FactoryMutation, FactoryQuery},
};
use entity::entity::factory;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/factory/add")]
pub async fn add(
    data: web::Data<AppState>,
    factory_form: web::Json<FactoryPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = factory_form.into_inner();

    let factory_model = FactoryPO::convert_po_to_model(form);

    let factory_save = FactoryMutation::create(conn, factory_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let factory_vo = FactoryVO::convert(conn, Some(factory_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(factory_vo))
}

#[tcdt_route(update)]
#[post("/factory/update")]
pub async fn update(
    data: web::Data<AppState>,
    factory_form: web::Json<FactoryPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = factory_form.into_inner();

    let factory_model = FactoryPO::convert_po_to_model(form);

    let factory_save = FactoryMutation::update_by_id(conn, factory_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let factory_vo = FactoryVO::convert(conn, Some(factory_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(factory_vo))
}

#[tcdt_route(remove)]
#[post("/factory/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    factory_form: web::Json<FactoryPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = factory_form.into_inner();

    let factory_model = FactoryPO::convert_po_to_model(form);

    let delete_result = FactoryMutation::delete(conn, factory_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/factory/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    factory_form: web::Json<Vec<FactoryPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = factory_form.into_inner();

    let mut model_list:Vec<factory::Model>  = vec![];
    for po in po_list {
        model_list.push(FactoryPO::convert_po_to_model(po));
    }
    
    let delete_result = FactoryMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/factory/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let factory_entity = FactoryQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let factory_vo = FactoryVO::convert(conn, Some(factory_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(factory_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/factory/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let factory_list = FactoryQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<FactoryVO> = vec![];
    for factory_entity in factory_list {
        let factory_vo = FactoryVO::convert(conn, Some(factory_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(factory_vo) = factory_vo {
            vos.push(factory_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/factory/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (factorys, num_items) = FactoryQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<FactoryVO> = vec![];
    for factory_entity in factorys {
        let factory_vo = FactoryVO::convert(conn, Some(factory_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(factory_vo) = factory_vo {
            vos.push(factory_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/factory/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let factory_list = FactoryQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for factory_entity in factory_list {
        let factory_vo = FactoryVO::convert(conn, Some(factory_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(factory_vo) = factory_vo {
            vos.push(factory_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}