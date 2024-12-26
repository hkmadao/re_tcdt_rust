use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::dto_node_ui_po::DtoNodeUiPO, vo::base::dto_node_ui_vo::DtoNodeUiVO},
    service::base::dto_node_ui_service::{DtoNodeUiMutation, DtoNodeUiQuery},
};
use entity::entity::dto_node_ui;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/dtoNodeUi/add")]
pub async fn add(
    data: web::Data<AppState>,
    dto_node_ui_form: web::Json<DtoNodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = dto_node_ui_form.into_inner();

    let dto_node_ui_model = DtoNodeUiPO::convert_po_to_model(form);

    let dto_node_ui_save = DtoNodeUiMutation::create(conn, dto_node_ui_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let dto_node_ui_vo = DtoNodeUiVO::convert(conn, Some(dto_node_ui_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_node_ui_vo))
}

#[tcdt_route(update)]
#[post("/dtoNodeUi/update")]
pub async fn update(
    data: web::Data<AppState>,
    dto_node_ui_form: web::Json<DtoNodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_node_ui_form.into_inner();

    let dto_node_ui_model = DtoNodeUiPO::convert_po_to_model(form);

    let dto_node_ui_save = DtoNodeUiMutation::update_by_id(conn, dto_node_ui_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dto_node_ui_vo = DtoNodeUiVO::convert(conn, Some(dto_node_ui_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(dto_node_ui_vo))
}

#[tcdt_route(remove)]
#[post("/dtoNodeUi/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    dto_node_ui_form: web::Json<DtoNodeUiPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = dto_node_ui_form.into_inner();

    let dto_node_ui_model = DtoNodeUiPO::convert_po_to_model(form);

    let delete_result = DtoNodeUiMutation::delete(conn, dto_node_ui_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/dtoNodeUi/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    dto_node_ui_form: web::Json<Vec<DtoNodeUiPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = dto_node_ui_form.into_inner();

    let mut model_list:Vec<dto_node_ui::Model>  = vec![];
    for po in po_list {
        model_list.push(DtoNodeUiPO::convert_po_to_model(po));
    }
    
    let delete_result = DtoNodeUiMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/dtoNodeUi/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let dto_node_ui_entity = DtoNodeUiQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dto_node_ui_vo = DtoNodeUiVO::convert(conn, Some(dto_node_ui_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(dto_node_ui_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/dtoNodeUi/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let dto_node_ui_list = DtoNodeUiQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<DtoNodeUiVO> = vec![];
    for dto_node_ui_entity in dto_node_ui_list {
        let dto_node_ui_vo = DtoNodeUiVO::convert(conn, Some(dto_node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_node_ui_vo) = dto_node_ui_vo {
            vos.push(dto_node_ui_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/dtoNodeUi/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (dto_node_uis, num_items) = DtoNodeUiQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<DtoNodeUiVO> = vec![];
    for dto_node_ui_entity in dto_node_uis {
        let dto_node_ui_vo = DtoNodeUiVO::convert(conn, Some(dto_node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_node_ui_vo) = dto_node_ui_vo {
            vos.push(dto_node_ui_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/dtoNodeUi/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let dto_node_ui_list = DtoNodeUiQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for dto_node_ui_entity in dto_node_ui_list {
        let dto_node_ui_vo = DtoNodeUiVO::convert(conn, Some(dto_node_ui_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(dto_node_ui_vo) = dto_node_ui_vo {
            vos.push(dto_node_ui_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}