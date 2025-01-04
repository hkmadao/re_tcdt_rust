use crate::api::common::param::IdsParam;
use crate::app::AppState;
use actix_web::body::BoxBody;
use actix_web::http::header;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_macro::tcdt_route;
use tcdt_service::dto::vo::ext::generate::generate_result::GenerateResult;
use tcdt_service::service::ext::generator::{
    component_comp::combination_generator as CombinationGenerator,
    component_enum::enum_generator as EnumGenerator,
    component_single::single_generator as SingleGenerator,
    data_transfer_object::data_transfer_object_generator as DtoGenerator,
    entity_coll::entity_coll_generator as EntityGenerator,
    ui_factory::ui_code_generator as UiCodeGenerator,
};
use url::form_urlencoded;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_service::common::aq::AqCondition;
use tcdt_service::dto::vo::ext::template_file::template_file::TemplateFileStat;
use tcdt_service::service::ext::template_file_ext_service::{TemplateFileExtMutation, TemplateFileExtQuery};

#[tcdt_route(get_tree_by_project_id)]
#[get("/templateFile/getTreeByProjectId")]
pub async fn get_tree_by_project_id(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let id_project = query_params
        .get("idProject")
        .ok_or(error::ErrorInternalServerError("idProject not found"))?
        .to_string();

    let template_file_result = TemplateFileExtQuery::fetch_tree_by_project_id(conn, id_project).await;
    match template_file_result {
        Ok(template_file) => {
            Ok(HttpResponse::Ok().json(template_file))
        }
        Err(err) => {
            match err {
                TcdtServiceError::TcdtInternal(internal_err) => {
                    log::error!("{:?}", internal_err);
                    Err(error::ErrorInternalServerError("internal server error"))
                }
                TcdtServiceError::Custom(custom_err) => {
                    log::error!("{:?}", custom_err);
                    Err(error::ErrorInternalServerError(custom_err.get_message()))
                }
            }
        }
    }
}

#[tcdt_route(get_file_by_path)]
#[get("/templateFile/getFileByPath")]
pub async fn get_file_by_path(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let id_project = query_params
        .get("idProject")
        .ok_or(error::ErrorInternalServerError("idProject not found"))?
        .to_string();
    let file_path = query_params
        .get("filePath")
        .ok_or(error::ErrorInternalServerError("filePath not found"))?
        .to_string();

    if file_path.contains("../") || file_path.contains("..\\") {
        log::error!("file_path illegal: {}", file_path);
        return Err(error::ErrorInternalServerError("internal server error"));
    }

    let template_file = TemplateFileExtQuery::fetch_file_by_path(conn, id_project, file_path)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(template_file))
}

#[tcdt_route(add)]
#[post("/templateFile/add")]
pub async fn add(
    req: HttpRequest,
    data: web::Data<AppState>,
    template_file_option: Option<web::Json<TemplateFileStat>>,
) -> Result<HttpResponse, Error> {
    if !TCDT_CONF.enable_code_template_edit {
        return Err(error::ErrorInternalServerError("modify template file api already disabled"));
    }

    let conn = &data.conn;
    let template_file_po = template_file_option.ok_or(error::ErrorInternalServerError("param is empty"))?.into_inner();

    if let Some(value) = check_path(&template_file_po) {
        return value;
    }

    let template_file = TemplateFileExtMutation::add_file(conn, template_file_po)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(template_file))
}


#[tcdt_route(update_stat)]
#[post("/templateFile/updateStat")]
pub async fn update_stat(
    req: HttpRequest,
    data: web::Data<AppState>,
    template_file_option: Option<web::Json<TemplateFileStat>>,
) -> Result<HttpResponse, Error> {
    if !TCDT_CONF.enable_code_template_edit {
        return Err(error::ErrorInternalServerError("modify template file api already disabled"));
    }

    let conn = &data.conn;
    let template_file_po = template_file_option.ok_or(error::ErrorInternalServerError("param is empty"))?.into_inner();

    if let Some(value) = check_path(&template_file_po) {
        return value;
    }

    let template_file = TemplateFileExtMutation::update_stat(conn, template_file_po)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(template_file))
}

#[tcdt_route(update_content)]
#[post("/templateFile/updateContent")]
pub async fn update_content(
    req: HttpRequest,
    data: web::Data<AppState>,
    template_file_option: Option<web::Json<TemplateFileStat>>,
) -> Result<HttpResponse, Error> {
    if !TCDT_CONF.enable_code_template_edit {
        return Err(error::ErrorInternalServerError("modify template file api already disabled"));
    }

    let conn = &data.conn;
    let template_file_po = template_file_option.ok_or(error::ErrorInternalServerError("param is empty"))?.into_inner();

    if let Some(value) = check_path(&template_file_po) {
        return value;
    }

    let template_file = TemplateFileExtMutation::update_content(conn, template_file_po)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(template_file))
}

#[tcdt_route(remove_file)]
#[post("/templateFile/remove")]
pub async fn remove_file(
    req: HttpRequest,
    data: web::Data<AppState>,
    template_file_option: Option<web::Json<TemplateFileStat>>,
) -> Result<HttpResponse, Error> {
    if !TCDT_CONF.enable_code_template_edit {
        return Err(error::ErrorInternalServerError("modify template file api already disabled"));
    }

    let conn = &data.conn;
    let template_file_po = template_file_option.ok_or(error::ErrorInternalServerError("param is empty"))?.into_inner();

    if let Some(value) = check_path(&template_file_po) {
        return value;
    }

    let template_file = TemplateFileExtMutation::remove_file(conn, template_file_po)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(template_file))
}

fn check_path(template_file_po: &TemplateFileStat) -> Option<Result<HttpResponse>> {
    if let Some(parent_path_name) = template_file_po.parent_path_name.clone() {
        if contains_illegal_char(&parent_path_name) {
            log::error!("parent_path_name illegal: {}", parent_path_name);
            return Some(Err(error::ErrorInternalServerError("internal server error")));
        }
    }

    if let Some(file_path_name) = template_file_po.file_path_name.clone() {
        if contains_illegal_char(&file_path_name) {
            log::error!("file_path_name illegal: {}", file_path_name);
            return Some(Err(error::ErrorInternalServerError("internal server error")));
        }
    }

    if contains_illegal_char(&template_file_po.file_name.clone()) {
        log::error!("file_name illegal: {}", template_file_po.file_name);
        return Some(Err(error::ErrorInternalServerError("internal server error")));
    }

    None
}

fn contains_illegal_char(parent_path_name: &String) -> bool {
    parent_path_name.contains("../")
        || parent_path_name.contains("..\\")
        || parent_path_name.contains("*")
        || parent_path_name.contains("?")
}