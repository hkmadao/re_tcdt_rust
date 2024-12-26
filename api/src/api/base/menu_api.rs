use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::menu_po::MenuPO, vo::base::menu_vo::MenuVO},
    service::base::menu_service::{MenuMutation, MenuQuery},
};
use entity::entity::menu;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/menu/add")]
pub async fn add(
    data: web::Data<AppState>,
    menu_form: web::Json<MenuPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = menu_form.into_inner();

    let menu_model = MenuPO::convert_po_to_model(form);

    let menu_save = MenuMutation::create(conn, menu_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let menu_vo = MenuVO::convert(conn, Some(menu_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(menu_vo))
}

#[tcdt_route(update)]
#[post("/menu/update")]
pub async fn update(
    data: web::Data<AppState>,
    menu_form: web::Json<MenuPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = menu_form.into_inner();

    let menu_model = MenuPO::convert_po_to_model(form);

    let menu_save = MenuMutation::update_by_id(conn, menu_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let menu_vo = MenuVO::convert(conn, Some(menu_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(menu_vo))
}

#[tcdt_route(remove)]
#[post("/menu/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    menu_form: web::Json<MenuPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = menu_form.into_inner();

    let menu_model = MenuPO::convert_po_to_model(form);

    let delete_result = MenuMutation::delete(conn, menu_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/menu/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    menu_form: web::Json<Vec<MenuPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = menu_form.into_inner();

    let mut model_list:Vec<menu::Model>  = vec![];
    for po in po_list {
        model_list.push(MenuPO::convert_po_to_model(po));
    }
    
    let delete_result = MenuMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/menu/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let menu_entity = MenuQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let menu_vo = MenuVO::convert(conn, Some(menu_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(menu_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/menu/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let menu_list = MenuQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<MenuVO> = vec![];
    for menu_entity in menu_list {
        let menu_vo = MenuVO::convert(conn, Some(menu_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(menu_vo) = menu_vo {
            vos.push(menu_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/menu/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (menus, num_items) = MenuQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<MenuVO> = vec![];
    for menu_entity in menus {
        let menu_vo = MenuVO::convert(conn, Some(menu_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(menu_vo) = menu_vo {
            vos.push(menu_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/menu/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let menu_list = MenuQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for menu_entity in menu_list {
        let menu_vo = MenuVO::convert(conn, Some(menu_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(menu_vo) = menu_vo {
            vos.push(menu_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}