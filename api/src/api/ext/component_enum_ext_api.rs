use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{aq::*, result::PageInfo},
    dto::vo::ext::component_enum::{
        detail::ComponentEnumVO as DetailComponentEnumVO,
        simple::ComponentEnumVO as SimpleComponentEnumVO,
    },
    service::base::component_enum_service::ComponentEnumQuery,
};

use crate::app::AppState;

#[tcdt_route(simple_aq)]
#[post("/componentEnum/simpleAq")]
pub async fn simple_aq(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let enums = ComponentEnumQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<SimpleComponentEnumVO> = vec![];
    for enum_entity in enums {
        let enum_vo = SimpleComponentEnumVO::convert(conn, Some(enum_entity))
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
#[get("/componentEnum/getDetail/{id}")]
pub async fn get_detail(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_enum_entity = ComponentEnumQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let component_enum_vo = DetailComponentEnumVO::convert(conn, Some(component_enum_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    Ok(HttpResponse::Ok().json(component_enum_vo))
}

#[tcdt_route(simple_aq_page)]
#[post("/componentEnum/simpleAqPage")]
pub async fn simple_aq_page(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_page_json: web::Json<AqPageInfoInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_page = aq_page_json.into_inner();
    let page_index = aq_page.page_index.clone();
    let page_size = aq_page.page_size.clone();

    let (component_enums, num_items) =
        ComponentEnumQuery::find_page_by_page_condition(conn, aq_page)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
    let mut vos: Vec<SimpleComponentEnumVO> = vec![];
    for component_enum_entity in component_enums {
        let component_enum_vo = SimpleComponentEnumVO::convert(conn, Some(component_enum_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?;
        if let Some(component_enum_vo) = component_enum_vo {
            vos.push(component_enum_vo);
        }
    }
    let page_info = PageInfo::new(page_index, page_size, num_items, vos);
    Ok(HttpResponse::Ok().json(page_info))
}
