use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::button_action_po::ButtonActionPO, vo::base::button_action_vo::ButtonActionVO},
    service::base::button_action_service::{ButtonActionMutation, ButtonActionQuery},
};
use entity::entity::button_action;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/buttonAction/add")]
pub async fn add(
    data: web::Data<AppState>,
    button_action_form: web::Json<ButtonActionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = button_action_form.into_inner();

    let button_action_model = ButtonActionPO::convert_po_to_model(form);

    let button_action_save = ButtonActionMutation::create(conn, button_action_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let button_action_vo = ButtonActionVO::convert(conn, Some(button_action_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(button_action_vo))
}

#[tcdt_route(update)]
#[post("/buttonAction/update")]
pub async fn update(
    data: web::Data<AppState>,
    button_action_form: web::Json<ButtonActionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = button_action_form.into_inner();

    let button_action_model = ButtonActionPO::convert_po_to_model(form);

    let button_action_save = ButtonActionMutation::update_by_id(conn, button_action_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let button_action_vo = ButtonActionVO::convert(conn, Some(button_action_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(button_action_vo))
}

#[tcdt_route(remove)]
#[post("/buttonAction/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    button_action_form: web::Json<ButtonActionPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = button_action_form.into_inner();

    let button_action_model = ButtonActionPO::convert_po_to_model(form);

    let delete_result = ButtonActionMutation::delete(conn, button_action_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/buttonAction/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    button_action_form: web::Json<Vec<ButtonActionPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = button_action_form.into_inner();

    let mut model_list:Vec<button_action::Model>  = vec![];
    for po in po_list {
        model_list.push(ButtonActionPO::convert_po_to_model(po));
    }
    
    let delete_result = ButtonActionMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/buttonAction/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let button_action_entity = ButtonActionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let button_action_vo = ButtonActionVO::convert(conn, Some(button_action_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(button_action_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/buttonAction/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let button_action_list = ButtonActionQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ButtonActionVO> = vec![];
    for button_action_entity in button_action_list {
        let button_action_vo = ButtonActionVO::convert(conn, Some(button_action_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(button_action_vo) = button_action_vo {
            vos.push(button_action_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/buttonAction/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (button_actions, num_items) = ButtonActionQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ButtonActionVO> = vec![];
    for button_action_entity in button_actions {
        let button_action_vo = ButtonActionVO::convert(conn, Some(button_action_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(button_action_vo) = button_action_vo {
            vos.push(button_action_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/buttonAction/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let button_action_list = ButtonActionQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for button_action_entity in button_action_list {
        let button_action_vo = ButtonActionVO::convert(conn, Some(button_action_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(button_action_vo) = button_action_vo {
            vos.push(button_action_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}