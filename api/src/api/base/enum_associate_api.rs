use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::enum_associate_po::EnumAssociatePO, vo::base::enum_associate_vo::EnumAssociateVO},
    service::base::enum_associate_service::{EnumAssociateMutation, EnumAssociateQuery},
};
use entity::entity::enum_associate;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/enumAssociate/add")]
pub async fn add(
    data: web::Data<AppState>,
    enum_associate_form: web::Json<EnumAssociatePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = enum_associate_form.into_inner();

    let enum_associate_model = EnumAssociatePO::convert_po_to_model(form);

    let enum_associate_save = EnumAssociateMutation::create(conn, enum_associate_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let enum_associate_vo = EnumAssociateVO::convert(conn, Some(enum_associate_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(enum_associate_vo))
}

#[tcdt_route(update)]
#[post("/enumAssociate/update")]
pub async fn update(
    data: web::Data<AppState>,
    enum_associate_form: web::Json<EnumAssociatePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = enum_associate_form.into_inner();

    let enum_associate_model = EnumAssociatePO::convert_po_to_model(form);

    let enum_associate_save = EnumAssociateMutation::update_by_id(conn, enum_associate_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let enum_associate_vo = EnumAssociateVO::convert(conn, Some(enum_associate_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(enum_associate_vo))
}

#[tcdt_route(remove)]
#[post("/enumAssociate/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    enum_associate_form: web::Json<EnumAssociatePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = enum_associate_form.into_inner();

    let enum_associate_model = EnumAssociatePO::convert_po_to_model(form);

    let delete_result = EnumAssociateMutation::delete(conn, enum_associate_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/enumAssociate/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    enum_associate_form: web::Json<Vec<EnumAssociatePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = enum_associate_form.into_inner();

    let mut model_list:Vec<enum_associate::Model>  = vec![];
    for po in po_list {
        model_list.push(EnumAssociatePO::convert_po_to_model(po));
    }
    
    let delete_result = EnumAssociateMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/enumAssociate/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let enum_associate_entity = EnumAssociateQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let enum_associate_vo = EnumAssociateVO::convert(conn, Some(enum_associate_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(enum_associate_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/enumAssociate/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let enum_associate_list = EnumAssociateQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<EnumAssociateVO> = vec![];
    for enum_associate_entity in enum_associate_list {
        let enum_associate_vo = EnumAssociateVO::convert(conn, Some(enum_associate_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(enum_associate_vo) = enum_associate_vo {
            vos.push(enum_associate_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/enumAssociate/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (enum_associates, num_items) = EnumAssociateQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<EnumAssociateVO> = vec![];
    for enum_associate_entity in enum_associates {
        let enum_associate_vo = EnumAssociateVO::convert(conn, Some(enum_associate_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(enum_associate_vo) = enum_associate_vo {
            vos.push(enum_associate_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/enumAssociate/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let enum_associate_list = EnumAssociateQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for enum_associate_entity in enum_associate_list {
        let enum_associate_vo = EnumAssociateVO::convert(conn, Some(enum_associate_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(enum_associate_vo) = enum_associate_vo {
            vos.push(enum_associate_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}