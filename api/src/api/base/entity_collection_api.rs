use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::entity_collection_po::EntityCollectionPO, vo::base::entity_collection_vo::EntityCollectionVO},
    service::base::entity_collection_service::{EntityCollectionMutation, EntityCollectionQuery},
};
use entity::entity::entity_collection;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/entityCollection/add")]
pub async fn add(
    data: web::Data<AppState>,
    entity_collection_form: web::Json<EntityCollectionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = entity_collection_form.into_inner();

    let entity_collection_model = EntityCollectionPO::convert_po_to_model(form);

    let entity_collection_save = EntityCollectionMutation::create(conn, entity_collection_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let entity_collection_vo = EntityCollectionVO::convert(conn, Some(entity_collection_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(entity_collection_vo))
}

#[tcdt_route(update)]
#[post("/entityCollection/update")]
pub async fn update(
    data: web::Data<AppState>,
    entity_collection_form: web::Json<EntityCollectionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = entity_collection_form.into_inner();

    let entity_collection_model = EntityCollectionPO::convert_po_to_model(form);

    let entity_collection_save = EntityCollectionMutation::update_by_id(conn, entity_collection_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let entity_collection_vo = EntityCollectionVO::convert(conn, Some(entity_collection_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(entity_collection_vo))
}

#[tcdt_route(remove)]
#[post("/entityCollection/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    entity_collection_form: web::Json<EntityCollectionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = entity_collection_form.into_inner();

    let entity_collection_model = EntityCollectionPO::convert_po_to_model(form);

    let delete_result = EntityCollectionMutation::delete(conn, entity_collection_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/entityCollection/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    entity_collection_form: web::Json<Vec<EntityCollectionPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = entity_collection_form.into_inner();

    let mut model_list:Vec<entity_collection::Model>  = vec![];
    for po in po_list {
        model_list.push(EntityCollectionPO::convert_po_to_model(po));
    }
    
    let delete_result = EntityCollectionMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/entityCollection/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let entity_collection_entity = EntityCollectionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let entity_collection_vo = EntityCollectionVO::convert(conn, Some(entity_collection_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(entity_collection_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/entityCollection/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let entity_collection_list = EntityCollectionQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<EntityCollectionVO> = vec![];
    for entity_collection_entity in entity_collection_list {
        let entity_collection_vo = EntityCollectionVO::convert(conn, Some(entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_collection_vo) = entity_collection_vo {
            vos.push(entity_collection_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/entityCollection/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (entity_collections, num_items) = EntityCollectionQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<EntityCollectionVO> = vec![];
    for entity_collection_entity in entity_collections {
        let entity_collection_vo = EntityCollectionVO::convert(conn, Some(entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_collection_vo) = entity_collection_vo {
            vos.push(entity_collection_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/entityCollection/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let entity_collection_list = EntityCollectionQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for entity_collection_entity in entity_collection_list {
        let entity_collection_vo = EntityCollectionVO::convert(conn, Some(entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_collection_vo) = entity_collection_vo {
            vos.push(entity_collection_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}