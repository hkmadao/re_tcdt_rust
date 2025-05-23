use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_node_ui_po::ComponentNodeUiPO, vo::base::component_node_ui_vo::ComponentNodeUiVO},
    service::base::component_node_ui_service::{ComponentNodeUiMutation, ComponentNodeUiQuery},
};
use entity::entity::component_node_ui;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/componentNodeUi/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_node_ui_form: web::Json<ComponentNodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_node_ui_form.into_inner();

    let component_node_ui_model = ComponentNodeUiPO::convert_po_to_model(form);

    let component_node_ui_save = ComponentNodeUiMutation::create(conn, component_node_ui_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_node_ui_vo = ComponentNodeUiVO::convert(conn, Some(component_node_ui_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_node_ui_vo))
}

#[tcdt_route(update)]
#[post("/componentNodeUi/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_node_ui_form: web::Json<ComponentNodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_node_ui_form.into_inner();

    let component_node_ui_model = ComponentNodeUiPO::convert_po_to_model(form);

    let component_node_ui_save = ComponentNodeUiMutation::update_by_id(conn, component_node_ui_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_node_ui_vo = ComponentNodeUiVO::convert(conn, Some(component_node_ui_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_node_ui_vo))
}

#[tcdt_route(remove)]
#[post("/componentNodeUi/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_node_ui_form: web::Json<ComponentNodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_node_ui_form.into_inner();

    let component_node_ui_model = ComponentNodeUiPO::convert_po_to_model(form);

    let delete_result = ComponentNodeUiMutation::delete(conn, component_node_ui_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/componentNodeUi/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    component_node_ui_form: web::Json<Vec<ComponentNodeUiPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = component_node_ui_form.into_inner();

    let mut model_list:Vec<component_node_ui::Model>  = vec![];
    for po in po_list {
        model_list.push(ComponentNodeUiPO::convert_po_to_model(po));
    }
    
    let delete_result = ComponentNodeUiMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/componentNodeUi/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_node_ui_entity = ComponentNodeUiQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_node_ui_vo = ComponentNodeUiVO::convert(conn, Some(component_node_ui_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_node_ui_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/componentNodeUi/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let component_node_ui_list = ComponentNodeUiQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentNodeUiVO> = vec![];
    for component_node_ui_entity in component_node_ui_list {
        let component_node_ui_vo = ComponentNodeUiVO::convert(conn, Some(component_node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_node_ui_vo) = component_node_ui_vo {
            vos.push(component_node_ui_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/componentNodeUi/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_node_uis, num_items) = ComponentNodeUiQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentNodeUiVO> = vec![];
    for component_node_ui_entity in component_node_uis {
        let component_node_ui_vo = ComponentNodeUiVO::convert(conn, Some(component_node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_node_ui_vo) = component_node_ui_vo {
            vos.push(component_node_ui_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/componentNodeUi/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_node_ui_list = ComponentNodeUiQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_node_ui_entity in component_node_ui_list {
        let component_node_ui_vo = ComponentNodeUiVO::convert(conn, Some(component_node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_node_ui_vo) = component_node_ui_vo {
            vos.push(component_node_ui_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}