use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_module_po::ComponentModulePO, vo::base::component_module_vo::ComponentModuleVO},
    service::base::component_module_service::{ComponentModuleMutation, ComponentModuleQuery},
};
use entity::entity::component_module;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/componentModule/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_module_form: web::Json<ComponentModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_module_form.into_inner();

    let component_module_model = ComponentModulePO::convert_po_to_model(form);

    let component_module_save = ComponentModuleMutation::create(conn, component_module_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_module_vo))
}

#[tcdt_route(update)]
#[post("/componentModule/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_module_form: web::Json<ComponentModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_module_form.into_inner();

    let component_module_model = ComponentModulePO::convert_po_to_model(form);

    let component_module_save = ComponentModuleMutation::update_by_id(conn, component_module_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_module_vo))
}

#[tcdt_route(remove)]
#[post("/componentModule/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_module_form: web::Json<ComponentModulePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_module_form.into_inner();

    let component_module_model = ComponentModulePO::convert_po_to_model(form);

    let delete_result = ComponentModuleMutation::delete(conn, component_module_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/componentModule/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    component_module_form: web::Json<Vec<ComponentModulePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = component_module_form.into_inner();

    let mut model_list:Vec<component_module::Model>  = vec![];
    for po in po_list {
        model_list.push(ComponentModulePO::convert_po_to_model(po));
    }
    
    let delete_result = ComponentModuleMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/componentModule/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_module_entity = ComponentModuleQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_module_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/componentModule/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let component_module_list = ComponentModuleQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentModuleVO> = vec![];
    for component_module_entity in component_module_list {
        let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_module_vo) = component_module_vo {
            vos.push(component_module_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/componentModule/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_modules, num_items) = ComponentModuleQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentModuleVO> = vec![];
    for component_module_entity in component_modules {
        let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_module_vo) = component_module_vo {
            vos.push(component_module_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/componentModule/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_module_list = ComponentModuleQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_module_entity in component_module_list {
        let component_module_vo = ComponentModuleVO::convert(conn, Some(component_module_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_module_vo) = component_module_vo {
            vos.push(component_module_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}