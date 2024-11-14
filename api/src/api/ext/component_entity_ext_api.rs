use crate::app::AppState;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::vo::ext::component_entity::{
        detail::ComponentEntityVO as DetailComponentEntityVO,
        simple::ComponentEntityVO as SimpleComponentEntityVO,
    },
    service::{
        base::component_entity_service::ComponentEntityQuery,
        ext::component::description_util::DescriptionUtil,
    },
};

#[tcdt_route(simple_aq)]
#[post("/componentEntity/simpleAq")]
pub async fn simple_aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let enums = ComponentEntityQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<SimpleComponentEntityVO> = vec![];
    for enum_entity in enums {
        let enum_vo = SimpleComponentEntityVO::convert(conn, Some(enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(entity_vo) = enum_vo {
            vos.push(entity_vo);
        }
    }
    Ok(HttpResponse::Ok().json(vos))
}

#[tcdt_route(get_detail)]
#[get("/componentEntity/getDetail/{id}")]
pub async fn get_detail(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_entity_entity =
        ComponentEntityQuery::find_by_id(conn, id)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                match e {
                    TcdtServiceError::Custom(cus) => {
                        error::ErrorInternalServerError(cus.get_message())
                    }
                    _ => error::ErrorInternalServerError("internal server error"),
                }
            })?;

    let component_entity_vo = DetailComponentEntityVO::convert(conn, Some(component_entity_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_entity_vo))
}

#[tcdt_route(simple_aq_page)]
#[post("/componentEntity/simpleAqPage")]
pub async fn simple_aq_page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_entitys, num_items) =
        ComponentEntityQuery::find_page_by_page_condition(conn, aq_page)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
    let mut vos: Vec<DetailComponentEntityVO> = vec![];
    for component_entity_entity in component_entitys {
        let component_entity_vo =
            DetailComponentEntityVO::convert(conn, Some(component_entity_entity))
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?;
        if let Some(component_entity_vo) = component_entity_vo {
            vos.push(component_entity_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}

#[tcdt_route(get_description_data)]
#[get("/componentEntity/getDescriptionData/{id}")]
pub async fn get_description_data(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let description_util = DescriptionUtil::load_data(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;
    let description = description_util
        .build_description_info(None)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(description))
}
