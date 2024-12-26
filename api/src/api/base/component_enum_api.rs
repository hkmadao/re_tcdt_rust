use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::component_enum_po::ComponentEnumPO, vo::base::component_enum_vo::ComponentEnumVO},
    service::base::component_enum_service::{ComponentEnumMutation, ComponentEnumQuery},
};
use entity::entity::component_enum;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/componentEnum/add")]
pub async fn add(
    data: web::Data<AppState>,
    component_enum_form: web::Json<ComponentEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = component_enum_form.into_inner();

    let component_enum_model = ComponentEnumPO::convert_po_to_model(form);

    let component_enum_save = ComponentEnumMutation::create(conn, component_enum_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let component_enum_vo = ComponentEnumVO::convert(conn, Some(component_enum_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_enum_vo))
}

#[tcdt_route(update)]
#[post("/componentEnum/update")]
pub async fn update(
    data: web::Data<AppState>,
    component_enum_form: web::Json<ComponentEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_enum_form.into_inner();

    let component_enum_model = ComponentEnumPO::convert_po_to_model(form);

    let component_enum_save = ComponentEnumMutation::update_by_id(conn, component_enum_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let component_enum_vo = ComponentEnumVO::convert(conn, Some(component_enum_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(component_enum_vo))
}

#[tcdt_route(remove)]
#[post("/componentEnum/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    component_enum_form: web::Json<ComponentEnumPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_enum_form.into_inner();

    let component_enum_model = ComponentEnumPO::convert_po_to_model(form);

    let delete_result = ComponentEnumMutation::delete(conn, component_enum_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/componentEnum/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    component_enum_form: web::Json<Vec<ComponentEnumPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = component_enum_form.into_inner();

    let mut model_list:Vec<component_enum::Model>  = vec![];
    for po in po_list {
        model_list.push(ComponentEnumPO::convert_po_to_model(po));
    }
    
    let delete_result = ComponentEnumMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/componentEnum/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_enum_entity = ComponentEnumQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_enum_vo = ComponentEnumVO::convert(conn, Some(component_enum_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_enum_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/componentEnum/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let component_enum_list = ComponentEnumQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ComponentEnumVO> = vec![];
    for component_enum_entity in component_enum_list {
        let component_enum_vo = ComponentEnumVO::convert(conn, Some(component_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_enum_vo) = component_enum_vo {
            vos.push(component_enum_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/componentEnum/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_enums, num_items) = ComponentEnumQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ComponentEnumVO> = vec![];
    for component_enum_entity in component_enums {
        let component_enum_vo = ComponentEnumVO::convert(conn, Some(component_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_enum_vo) = component_enum_vo {
            vos.push(component_enum_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/componentEnum/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let component_enum_list = ComponentEnumQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for component_enum_entity in component_enum_list {
        let component_enum_vo = ComponentEnumVO::convert(conn, Some(component_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_enum_vo) = component_enum_vo {
            vos.push(component_enum_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}