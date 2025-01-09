use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::token_po::TokenPO, vo::base::token_vo::TokenVO},
    service::base::token_service::{TokenMutation, TokenQuery},
};
use entity::entity::token;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/token/add")]
pub async fn add(
    data: web::Data<AppState>,
    token_form: web::Json<TokenPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = token_form.into_inner();

    let token_model = TokenPO::convert_po_to_model(form);

    let token_save = TokenMutation::create(conn, token_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let token_vo = TokenVO::convert(conn, Some(token_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(token_vo))
}

#[tcdt_route(update)]
#[post("/token/update")]
pub async fn update(
    data: web::Data<AppState>,
    token_form: web::Json<TokenPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = token_form.into_inner();

    let token_model = TokenPO::convert_po_to_model(form);

    let token_save = TokenMutation::update_by_id(conn, token_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let token_vo = TokenVO::convert(conn, Some(token_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(token_vo))
}

#[tcdt_route(remove)]
#[post("/token/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    token_form: web::Json<TokenPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = token_form.into_inner();

    let token_model = TokenPO::convert_po_to_model(form);

    let delete_result = TokenMutation::delete(conn, token_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/token/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    token_form: web::Json<Vec<TokenPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = token_form.into_inner();

    let mut model_list:Vec<token::Model>  = vec![];
    for po in po_list {
        model_list.push(TokenPO::convert_po_to_model(po));
    }
    
    let delete_result = TokenMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/token/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let token_entity = TokenQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let token_vo = TokenVO::convert(conn, Some(token_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(token_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/token/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let token_list = TokenQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<TokenVO> = vec![];
    for token_entity in token_list {
        let token_vo = TokenVO::convert(conn, Some(token_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(token_vo) = token_vo {
            vos.push(token_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/token/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (tokens, num_items) = TokenQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<TokenVO> = vec![];
    for token_entity in tokens {
        let token_vo = TokenVO::convert(conn, Some(token_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(token_vo) = token_vo {
            vos.push(token_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/token/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let token_list = TokenQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for token_entity in token_list {
        let token_vo = TokenVO::convert(conn, Some(token_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(token_vo) = token_vo {
            vos.push(token_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}