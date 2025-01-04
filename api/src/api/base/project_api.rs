use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::{TcdtCudParamObjectTrait, TcdtViewObjectTrait};
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::{po::base::project_po::ProjectPO, vo::base::project_vo::ProjectVO},
    service::base::project_service::{ProjectMutation, ProjectQuery},
};
use entity::entity::project;
use tcdt_service::util::file_util::illegal_folder_name;
use crate::api::common::param::IdsParam;
use crate::app::AppState;

#[tcdt_route(add)]
#[post("/project/add")]
pub async fn add(
    data: web::Data<AppState>,
    project_form: web::Json<ProjectPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = project_form.into_inner();

    if form.template_code == None {
        return Err(error::ErrorInternalServerError("template_code is empty"));
    }

    if illegal_folder_name(&form.template_code.clone().unwrap()) {
        return Err(error::ErrorInternalServerError("template_code illegal"));
    }

    let project_model = ProjectPO::convert_po_to_model(form);

    let project_save = ProjectMutation::create(conn, project_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let project_vo = ProjectVO::convert(conn, Some(project_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(project_vo))
}

#[tcdt_route(update)]
#[post("/project/update")]
pub async fn update(
    data: web::Data<AppState>,
    project_form: web::Json<ProjectPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = project_form.into_inner();

    if form.template_code == None {
        return Err(error::ErrorInternalServerError("template_code is empty"));
    }

    if illegal_folder_name(&form.template_code.clone().unwrap()) {
        return Err(error::ErrorInternalServerError("template_code illegal"));
    }

    let project_model = ProjectPO::convert_po_to_model(form);

    let project_save = ProjectMutation::update_by_id(conn, project_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let project_vo = ProjectVO::convert(conn, Some(project_save))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;
    Ok(HttpResponse::Ok().json(project_vo))
}

#[tcdt_route(remove)]
#[post("/project/remove")]
pub async fn remove(
    data: web::Data<AppState>,
    project_form: web::Json<ProjectPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = project_form.into_inner();

    let project_model = ProjectPO::convert_po_to_model(form);

    let delete_result = ProjectMutation::delete(conn, project_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(batch_remove)]
#[post("/project/batchRemove")]
pub async fn batch_remove(
    data: web::Data<AppState>,
    project_form: web::Json<Vec<ProjectPO>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let po_list = project_form.into_inner();

    let mut model_list: Vec<project::Model> = vec![];
    for po in po_list {
        model_list.push(ProjectPO::convert_po_to_model(po));
    }

    let delete_result = ProjectMutation::batch_delete(conn, model_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(delete_result.rows_affected))
}

#[tcdt_route(get_by_id)]
#[get("/project/getById/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let project_entity = ProjectQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let project_vo = ProjectVO::convert(conn, Some(project_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(project_vo))
}

#[tcdt_route(get_by_ids)]
#[get("/project/getByIds")]
pub async fn get_by_ids(
    data: web::Data<AppState>,
    ids_param_form: web::Form<IdsParam>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let ids_param = ids_param_form.into_inner();

    let ids = ids_param.ids.split(",").map(|id| id.to_owned()).collect();

    let project_list = ProjectQuery::find_by_ids(conn, ids)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut vos: Vec<ProjectVO> = vec![];
    for project_entity in project_list {
        let project_vo = ProjectVO::convert(conn, Some(project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(project_vo) = project_vo {
            vos.push(project_vo);
        }
    }

    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(page)]
#[post("/project/aqPage")]
pub async fn page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (projects, num_items) = ProjectQuery::find_page_by_page_condition(conn, aq_page)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<ProjectVO> = vec![];
    for project_entity in projects {
        let project_vo = ProjectVO::convert(conn, Some(project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(project_vo) = project_vo {
            vos.push(project_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[post("/project/aq")]
pub async fn aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_json: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq = aq_json.into_inner();

    let project_list = ProjectQuery::find_collection_by_condition(conn, aq)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos = vec![];
    for project_entity in project_list {
        let project_vo = ProjectVO::convert(conn, Some(project_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(project_vo) = project_vo {
            vos.push(project_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}