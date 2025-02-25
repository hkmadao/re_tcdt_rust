use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::user_role_po::UserRolePO, vo::base::user_role_vo::UserRoleVO},
    service::base::user_role_service::{UserRoleMutation, UserRoleQuery},
};
use entity::entity::user_role;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/userRole/add")]
pub async fn add(
    data: web::Data<AppState>,
    user_role_form: web::Json<UserRolePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = user_role_form.into_inner();

    let user_role_model = UserRolePO::convert_po_to_model(form);

    let user_role_save = UserRoleMutation::create(conn, user_role_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let user_role_vo = UserRoleVO::convert(conn, Some(user_role_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(user_role_vo))
}

#[tcdt_route(update)]
#[post("/userRole/update")]
pub async fn update(
    data: web::Data<AppState>,
    user_role_form: web::Json<UserRolePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = user_role_form.into_inner();

    let user_role_model = UserRolePO::convert_po_to_model(form);

    let user_role_save = UserRoleMutation::update_by_id(conn, user_role_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let user_role_vo = UserRoleVO::convert(conn, Some(user_role_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(user_role_vo))
}

#[tcdt_route(remove)]
#[post("/userRole/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    user_role_form: web::Json<UserRolePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = user_role_form.into_inner();

    let user_role_model = UserRolePO::convert_po_to_model(form);

    let delete_result = UserRoleMutation::delete(conn, user_role_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/userRole/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    user_role_form: web::Json<Vec<UserRolePO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = user_role_form.into_inner();

    let mut model_list:Vec<user_role::Model>  = vec![];
    for po in po_list {
        model_list.push(UserRolePO::convert_po_to_model(po));
    }
    
    let delete_result = UserRoleMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/userRole/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let user_role_entity = UserRoleQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let user_role_vo = UserRoleVO::convert(conn, Some(user_role_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(user_role_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/userRole/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let user_role_list = UserRoleQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<UserRoleVO> = vec![];
    for user_role_entity in user_role_list {
        let user_role_vo = UserRoleVO::convert(conn, Some(user_role_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(user_role_vo) = user_role_vo {
            vos.push(user_role_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/userRole/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (user_roles, num_items) = UserRoleQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<UserRoleVO> = vec![];
    for user_role_entity in user_roles {
        let user_role_vo = UserRoleVO::convert(conn, Some(user_role_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(user_role_vo) = user_role_vo {
            vos.push(user_role_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/userRole/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let user_role_list = UserRoleQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for user_role_entity in user_role_list {
        let user_role_vo = UserRoleVO::convert(conn, Some(user_role_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(user_role_vo) = user_role_vo {
            vos.push(user_role_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}