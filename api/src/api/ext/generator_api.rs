use crate::app::AppState;
use actix_web::body::BoxBody;
use actix_web::http::header;
use actix_web::{error, get, web, Error, HttpRequest, HttpResponse, Result};
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

#[tcdt_route(generate_single_file)]
#[get("/entityCollection/generateSingleFile")]
pub async fn generate_single_file(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = EntityGenerator::generate_single_file(conn, id).await;

    match generate_result {
        Ok(generate_result) => {
            let mut file = File::open(generate_result.file_full_name).map_err(|err| {
                log::error!("{:?}", err);
                error::ErrorInternalServerError("internal server error")
            })?;
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer);
            HttpResponse::Ok()
                .append_header((header::CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8))
                .message_body(BoxBody::new(buffer))
        }
        Err(err) => match err {
            TcdtServiceError::Custom(e) => {
                log::error!("{:?}", e);
                Err(error::ErrorInternalServerError(e.get_message()))
            }
            _ => {
                log::error!("{:?}", err);
                Err(error::ErrorInternalServerError("internal server error"))
            }
        },
    }
}

#[tcdt_route(entity_coll_generate_full)]
#[get("/entityCollection/generateFull")]
pub async fn entity_coll_generate_full(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = EntityGenerator::generate_full(conn, id).await;

    generate_result_handle(generate_result)
}

#[tcdt_route(entity_coll_generate_part)]
#[get("/entityCollection/generatePart")]
pub async fn entity_coll_generate_part(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = EntityGenerator::generate_part(conn, id).await;

    generate_result_handle(generate_result)
}

#[tcdt_route(factory_generate)]
#[get("/factory/generate")]
pub async fn factory_generate(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = UiCodeGenerator::ui_code_generate(conn, id).await;

    generate_result_handle(generate_result)
}

#[tcdt_route(generate_enum_part)]
#[get("/component/generateEnumPart")]
pub async fn generate_enum_part(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = EnumGenerator::generate_part(conn, id).await;

    generate_result_handle(generate_result)
}

#[tcdt_route(generate_enum_full)]
#[get("/component/generateEnumFull")]
pub async fn generate_enum_full(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = EnumGenerator::generate_full(conn, id).await;

    generate_result_handle(generate_result)
}

#[tcdt_route(generate_combination)]
#[get("/component/generateCombination")]
pub async fn generate_combination(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = CombinationGenerator::generate(conn, id).await;

    generate_result_handle(generate_result)
}

#[tcdt_route(generate_single)]
#[get("/component/generateSingle")]
pub async fn generate_single(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = SingleGenerator::generate(conn, id).await;

    generate_result_handle(generate_result)
}
#[tcdt_route(generate_input_full)]
#[get("/dtoEntityCollection/generateInputFull")]
pub async fn generate_input_full(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = DtoGenerator::generate_input_full(conn, id).await;

    generate_result_handle(generate_result)
}
#[tcdt_route(generate_input_part)]
#[get("/dtoEntityCollection/generateInputPart")]
pub async fn generate_input_part(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = DtoGenerator::generate_input_part(conn, id).await;

    generate_result_handle(generate_result)
}
#[tcdt_route(generate_output_full)]
#[get("/dtoEntityCollection/generateOutputFull")]
pub async fn generate_output_full(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = DtoGenerator::generate_output_full(conn, id).await;

    generate_result_handle(generate_result)
}
#[tcdt_route(generate_output_part)]
#[get("/dtoEntityCollection/generateOutputPart")]
pub async fn generate_output_part(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = parse_id_from_query(req)?;

    let generate_result = DtoGenerator::generate_output_part(conn, id).await;

    generate_result_handle(generate_result)
}

fn parse_id_from_query(req: HttpRequest) -> Result<String, Error> {
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let id = query_params
        .get("id")
        .ok_or(error::ErrorInternalServerError("id not found"))?
        .to_string();
    Ok(id)
}

fn generate_result_handle(
    generate_result: Result<GenerateResult, TcdtServiceError>,
) -> Result<HttpResponse, Error> {
    match generate_result {
        Ok(generate_result) => {
            let mut file = File::open(generate_result.zip_file_full_name).map_err(|err| {
                log::error!("{:?}", err);
                error::ErrorInternalServerError("internal server error")
            })?;
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer);
            HttpResponse::Ok()
                .append_header((
                    header::CONTENT_TYPE,
                    format!(
                        "{};{}={}",
                        mime::APPLICATION_OCTET_STREAM,
                        mime::CHARSET,
                        mime::UTF_8
                    ),
                ))
                .append_header((
                    header::CONTENT_DISPOSITION,
                    format!(
                        "attachment;filename={}",
                        form_urlencoded::Serializer::new(String::new())
                            .append_key_only(&generate_result.zip_file_name)
                            .finish()
                    ),
                ))
                .message_body(BoxBody::new(buffer))
        }
        Err(err) => match err {
            TcdtServiceError::Custom(e) => {
                log::error!("{:?}", e);
                Err(error::ErrorInternalServerError(e.get_message()))
            }
            _ => {
                log::error!("{:?}", err);
                Err(error::ErrorInternalServerError("internal server error"))
            }
        },
    }
}
