use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::computation_attribute_po::ComputationAttributePO, vo::base::computation_attribute_vo::ComputationAttributeVO},
    service::base::computation_attribute_service::{ComputationAttributeMutation, ComputationAttributeQuery},
};
use entity::entity::computation_attribute;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/computationAttribute/add")]
pub async fn add(
    data: web::Data<AppState>,
    computation_attribute_form: web::Json<ComputationAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = computation_attribute_form.into_inner();

    let computation_attribute_model = ComputationAttributePO::convert_po_to_model(form);

    let computation_attribute_save = ComputationAttributeMutation::create(conn, computation_attribute_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let computation_attribute_vo = ComputationAttributeVO::convert(conn, Some(computation_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(computation_attribute_vo))
}

#[tcdt_route(update)]
#[post("/computationAttribute/update")]
pub async fn update(
    data: web::Data<AppState>,
    computation_attribute_form: web::Json<ComputationAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = computation_attribute_form.into_inner();

    let computation_attribute_model = ComputationAttributePO::convert_po_to_model(form);

    let computation_attribute_save = ComputationAttributeMutation::update_by_id(conn, computation_attribute_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let computation_attribute_vo = ComputationAttributeVO::convert(conn, Some(computation_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(computation_attribute_vo))
}

#[tcdt_route(remove)]
#[post("/computationAttribute/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    computation_attribute_form: web::Json<ComputationAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = computation_attribute_form.into_inner();

    let computation_attribute_model = ComputationAttributePO::convert_po_to_model(form);

    let delete_result = ComputationAttributeMutation::delete(conn, computation_attribute_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/computationAttribute/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    computation_attribute_form: web::Json<Vec<ComputationAttributePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = computation_attribute_form.into_inner();

    let mut model_list:Vec<computation_attribute::Model>  = vec![];
    for po in po_list {
        model_list.push(ComputationAttributePO::convert_po_to_model(po));
    }
    
    let delete_result = ComputationAttributeMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/computationAttribute/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let computation_attribute_entity = ComputationAttributeQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let computation_attribute_vo = ComputationAttributeVO::convert(conn, Some(computation_attribute_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(computation_attribute_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/computationAttribute/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let computation_attribute_list = ComputationAttributeQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComputationAttributeVO> = vec![];
    for computation_attribute_entity in computation_attribute_list {
        let computation_attribute_vo = ComputationAttributeVO::convert(conn, Some(computation_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(computation_attribute_vo) = computation_attribute_vo {
            vos.push(computation_attribute_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/computationAttribute/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (computation_attributes, num_items) = ComputationAttributeQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComputationAttributeVO> = vec![];
    for computation_attribute_entity in computation_attributes {
        let computation_attribute_vo = ComputationAttributeVO::convert(conn, Some(computation_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(computation_attribute_vo) = computation_attribute_vo {
            vos.push(computation_attribute_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/computationAttribute/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let computation_attribute_list = ComputationAttributeQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for computation_attribute_entity in computation_attribute_list {
        let computation_attribute_vo = ComputationAttributeVO::convert(conn, Some(computation_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(computation_attribute_vo) = computation_attribute_vo {
            vos.push(computation_attribute_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}