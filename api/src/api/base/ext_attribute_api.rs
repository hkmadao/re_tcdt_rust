use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::ext_attribute_po::ExtAttributePO, vo::base::ext_attribute_vo::ExtAttributeVO},
    service::base::ext_attribute_service::{ExtAttributeMutation, ExtAttributeQuery},
};
use entity::entity::ext_attribute;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/extAttribute/add")]
pub async fn add(
    data: web::Data<AppState>,
    ext_attribute_form: web::Json<ExtAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = ext_attribute_form.into_inner();

    let ext_attribute_model = ExtAttributePO::convert_po_to_model(form);

    let ext_attribute_save = ExtAttributeMutation::create(conn, ext_attribute_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let ext_attribute_vo = ExtAttributeVO::convert(conn, Some(ext_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(ext_attribute_vo))
}

#[tcdt_route(update)]
#[post("/extAttribute/update")]
pub async fn update(
    data: web::Data<AppState>,
    ext_attribute_form: web::Json<ExtAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = ext_attribute_form.into_inner();

    let ext_attribute_model = ExtAttributePO::convert_po_to_model(form);

    let ext_attribute_save = ExtAttributeMutation::update_by_id(conn, ext_attribute_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let ext_attribute_vo = ExtAttributeVO::convert(conn, Some(ext_attribute_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(ext_attribute_vo))
}

#[tcdt_route(remove)]
#[post("/extAttribute/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    ext_attribute_form: web::Json<ExtAttributePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = ext_attribute_form.into_inner();

    let ext_attribute_model = ExtAttributePO::convert_po_to_model(form);

    let delete_result = ExtAttributeMutation::delete(conn, ext_attribute_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/extAttribute/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    ext_attribute_form: web::Json<Vec<ExtAttributePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = ext_attribute_form.into_inner();

    let mut model_list:Vec<ext_attribute::Model>  = vec![];
    for po in po_list {
        model_list.push(ExtAttributePO::convert_po_to_model(po));
    }
    
    let delete_result = ExtAttributeMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/extAttribute/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let ext_attribute_entity = ExtAttributeQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let ext_attribute_vo = ExtAttributeVO::convert(conn, Some(ext_attribute_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(ext_attribute_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/extAttribute/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let ext_attribute_list = ExtAttributeQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ExtAttributeVO> = vec![];
    for ext_attribute_entity in ext_attribute_list {
        let ext_attribute_vo = ExtAttributeVO::convert(conn, Some(ext_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(ext_attribute_vo) = ext_attribute_vo {
            vos.push(ext_attribute_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/extAttribute/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (ext_attributes, num_items) = ExtAttributeQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ExtAttributeVO> = vec![];
    for ext_attribute_entity in ext_attributes {
        let ext_attribute_vo = ExtAttributeVO::convert(conn, Some(ext_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(ext_attribute_vo) = ext_attribute_vo {
            vos.push(ext_attribute_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/extAttribute/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let ext_attribute_list = ExtAttributeQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for ext_attribute_entity in ext_attribute_list {
        let ext_attribute_vo = ExtAttributeVO::convert(conn, Some(ext_attribute_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(ext_attribute_vo) = ext_attribute_vo {
            vos.push(ext_attribute_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}