use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_entity_associate_po::ComponentEntityAssociatePO, vo::base::component_entity_associate_vo::ComponentEntityAssociateVO},
    service::base::component_entity_associate_service::{ComponentEntityAssociateMutation, ComponentEntityAssociateQuery},
};
use entity::entity::component_entity_associate;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/componentEntityAssociate/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_entity_associate_form: web::Json<ComponentEntityAssociatePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_entity_associate_form.into_inner();

    let component_entity_associate_model = ComponentEntityAssociatePO::convert_po_to_model(form);

    let component_entity_associate_save = ComponentEntityAssociateMutation::create(conn, component_entity_associate_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_entity_associate_vo = ComponentEntityAssociateVO::convert(conn, Some(component_entity_associate_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_entity_associate_vo))
}

#[tcdt_route(update)]
#[post("/componentEntityAssociate/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_entity_associate_form: web::Json<ComponentEntityAssociatePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_entity_associate_form.into_inner();

    let component_entity_associate_model = ComponentEntityAssociatePO::convert_po_to_model(form);

    let component_entity_associate_save = ComponentEntityAssociateMutation::update_by_id(conn, component_entity_associate_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_entity_associate_vo = ComponentEntityAssociateVO::convert(conn, Some(component_entity_associate_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_entity_associate_vo))
}

#[tcdt_route(remove)]
#[post("/componentEntityAssociate/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_entity_associate_form: web::Json<ComponentEntityAssociatePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_entity_associate_form.into_inner();

    let component_entity_associate_model = ComponentEntityAssociatePO::convert_po_to_model(form);

    let delete_result = ComponentEntityAssociateMutation::delete(conn, component_entity_associate_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/componentEntityAssociate/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    component_entity_associate_form: web::Json<Vec<ComponentEntityAssociatePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = component_entity_associate_form.into_inner();

    let mut model_list:Vec<component_entity_associate::Model>  = vec![];
    for po in po_list {
        model_list.push(ComponentEntityAssociatePO::convert_po_to_model(po));
    }
    
    let delete_result = ComponentEntityAssociateMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/componentEntityAssociate/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_entity_associate_entity = ComponentEntityAssociateQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_entity_associate_vo = ComponentEntityAssociateVO::convert(conn, Some(component_entity_associate_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_entity_associate_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/componentEntityAssociate/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let component_entity_associate_list = ComponentEntityAssociateQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentEntityAssociateVO> = vec![];
    for component_entity_associate_entity in component_entity_associate_list {
        let component_entity_associate_vo = ComponentEntityAssociateVO::convert(conn, Some(component_entity_associate_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_entity_associate_vo) = component_entity_associate_vo {
            vos.push(component_entity_associate_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/componentEntityAssociate/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_entity_associates, num_items) = ComponentEntityAssociateQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentEntityAssociateVO> = vec![];
    for component_entity_associate_entity in component_entity_associates {
        let component_entity_associate_vo = ComponentEntityAssociateVO::convert(conn, Some(component_entity_associate_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_entity_associate_vo) = component_entity_associate_vo {
            vos.push(component_entity_associate_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/componentEntityAssociate/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_entity_associate_list = ComponentEntityAssociateQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_entity_associate_entity in component_entity_associate_list {
        let component_entity_associate_vo = ComponentEntityAssociateVO::convert(conn, Some(component_entity_associate_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_entity_associate_vo) = component_entity_associate_vo {
            vos.push(component_entity_associate_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}