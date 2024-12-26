use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_po::ComponentPO, vo::base::component_vo::ComponentVO},
    service::base::component_service::{ComponentMutation, ComponentQuery},
};
use entity::entity::component;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/component/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_form: web::Json<ComponentPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_form.into_inner();

    let component_model = ComponentPO::convert_po_to_model(form);

    let component_save = ComponentMutation::create(conn, component_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_vo = ComponentVO::convert(conn, Some(component_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_vo))
}

#[tcdt_route(update)]
#[post("/component/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_form: web::Json<ComponentPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_form.into_inner();

    let component_model = ComponentPO::convert_po_to_model(form);

    let component_save = ComponentMutation::update_by_id(conn, component_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_vo = ComponentVO::convert(conn, Some(component_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_vo))
}

#[tcdt_route(remove)]
#[post("/component/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_form: web::Json<ComponentPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_form.into_inner();

    let component_model = ComponentPO::convert_po_to_model(form);

    let delete_result = ComponentMutation::delete(conn, component_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/component/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    component_form: web::Json<Vec<ComponentPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = component_form.into_inner();

    let mut model_list:Vec<component::Model>  = vec![];
    for po in po_list {
        model_list.push(ComponentPO::convert_po_to_model(po));
    }
    
    let delete_result = ComponentMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/component/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_entity = ComponentQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_vo = ComponentVO::convert(conn, Some(component_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/component/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let component_list = ComponentQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentVO> = vec![];
    for component_entity in component_list {
        let component_vo = ComponentVO::convert(conn, Some(component_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_vo) = component_vo {
            vos.push(component_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/component/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (components, num_items) = ComponentQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentVO> = vec![];
    for component_entity in components {
        let component_vo = ComponentVO::convert(conn, Some(component_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_vo) = component_vo {
            vos.push(component_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/component/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_list = ComponentQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_entity in component_list {
        let component_vo = ComponentVO::convert(conn, Some(component_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_vo) = component_vo {
            vos.push(component_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}