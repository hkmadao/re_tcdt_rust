use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::{{ rootInfo.snakeCaseName }}_po::{{ rootInfo.pascalCaseName }}PO, vo::base::{{ rootInfo.snakeCaseName }}_vo::{{ rootInfo.pascalCaseName }}VO},
    service::base::{{ rootInfo.snakeCaseName }}_service::{{ "{" }}{{ rootInfo.pascalCaseName }}Mutation, {{ rootInfo.pascalCaseName }}Query},
};
use entity::entity::{{ rootInfo.snakeCaseName }};
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/{{ rootInfo.camelCaseName }}/add")]
pub async fn add(
    data: web::Data<AppState>,
    {{ rootInfo.snakeCaseName }}_form: web::Json<{{ rootInfo.pascalCaseName }}PO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = {{ rootInfo.snakeCaseName }}_form.into_inner();

    let {{ rootInfo.snakeCaseName }}_model = {{ rootInfo.pascalCaseName }}PO::convert_po_to_model(form);

    let {{ rootInfo.snakeCaseName }}_save = {{ rootInfo.pascalCaseName }}Mutation::create(conn, {{ rootInfo.snakeCaseName }}_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}VO::convert(conn, Some({{ rootInfo.snakeCaseName }}_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json({{ rootInfo.snakeCaseName }}_vo))
}

#[tcdt_route(update)]
#[post("/{{ rootInfo.camelCaseName }}/update")]
pub async fn update(
    data: web::Data<AppState>,
    {{ rootInfo.snakeCaseName }}_form: web::Json<{{ rootInfo.pascalCaseName }}PO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = {{ rootInfo.snakeCaseName }}_form.into_inner();

    let {{ rootInfo.snakeCaseName }}_model = {{ rootInfo.pascalCaseName }}PO::convert_po_to_model(form);

    let {{ rootInfo.snakeCaseName }}_save = {{ rootInfo.pascalCaseName }}Mutation::update_by_id(conn, {{ rootInfo.snakeCaseName }}_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}VO::convert(conn, Some({{ rootInfo.snakeCaseName }}_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json({{ rootInfo.snakeCaseName }}_vo))
}

#[tcdt_route(remove)]
#[post("/{{ rootInfo.camelCaseName }}/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    {{ rootInfo.snakeCaseName }}_form: web::Json<{{ rootInfo.pascalCaseName }}PO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = {{ rootInfo.snakeCaseName }}_form.into_inner();

    let {{ rootInfo.snakeCaseName }}_model = {{ rootInfo.pascalCaseName }}PO::convert_po_to_model(form);

    let delete_result = {{ rootInfo.pascalCaseName }}Mutation::delete(conn, {{ rootInfo.snakeCaseName }}_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/{{ rootInfo.camelCaseName }}/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    {{ rootInfo.snakeCaseName }}_form: web::Json<Vec<{{ rootInfo.pascalCaseName }}PO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = {{ rootInfo.snakeCaseName }}_form.into_inner();

    let mut model_list:Vec<{{ rootInfo.snakeCaseName }}::Model>  = vec![];
    for po in po_list {
        model_list.push({{ rootInfo.pascalCaseName }}PO::convert_po_to_model(po));
    }
    
    let delete_result = {{ rootInfo.pascalCaseName }}Mutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/{{ rootInfo.camelCaseName }}/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let {{ rootInfo.snakeCaseName }}_entity = {{ rootInfo.pascalCaseName }}Query::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}VO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json({{ rootInfo.snakeCaseName }}_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/{{ rootInfo.camelCaseName }}/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let {{ rootInfo.snakeCaseName }}_list = {{ rootInfo.pascalCaseName }}Query::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<{{ rootInfo.pascalCaseName }}VO> = vec![];
    for {{ rootInfo.snakeCaseName }}_entity in {{ rootInfo.snakeCaseName }}_list {
        let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}VO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some({{ rootInfo.snakeCaseName }}_vo) = {{ rootInfo.snakeCaseName }}_vo {
            vos.push({{ rootInfo.snakeCaseName }}_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/{{ rootInfo.camelCaseName }}/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let ({{ rootInfo.snakeCaseName }}s, num_items) = {{ rootInfo.pascalCaseName }}Query::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<{{ rootInfo.pascalCaseName }}VO> = vec![];
    for {{ rootInfo.snakeCaseName }}_entity in {{ rootInfo.snakeCaseName }}s {
        let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}VO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some({{ rootInfo.snakeCaseName }}_vo) = {{ rootInfo.snakeCaseName }}_vo {
            vos.push({{ rootInfo.snakeCaseName }}_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/{{ rootInfo.camelCaseName }}/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let {{ rootInfo.snakeCaseName }}_list = {{ rootInfo.pascalCaseName }}Query::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for {{ rootInfo.snakeCaseName }}_entity in {{ rootInfo.snakeCaseName }}_list {
        let {{ rootInfo.snakeCaseName }}_vo = {{ rootInfo.pascalCaseName }}VO::convert(conn, Some({{ rootInfo.snakeCaseName }}_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some({{ rootInfo.snakeCaseName }}_vo) = {{ rootInfo.snakeCaseName }}_vo {
            vos.push({{ rootInfo.snakeCaseName }}_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}