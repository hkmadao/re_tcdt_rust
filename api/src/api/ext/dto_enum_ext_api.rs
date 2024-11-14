use actix_web::{error, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::aq::*, dto::vo::ext::dto_enum::simple::DtoEnumVO as SimpleEnumVO,
    service::base::dto_enum_service::DtoEnumQuery,
};

use crate::app::AppState;

#[tcdt_route(aq_detail)]
#[post("/dtoEnum/aqDetail")]
pub async fn aq_detail(
    _req: HttpRequest,
    data: web::Data<AppState>,
    aq_condition: web::Json<AqCondition>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let aq_condition = aq_condition.into_inner();

    let enums = DtoEnumQuery::find_collection_by_condition(conn, aq_condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut vos: Vec<SimpleEnumVO> = vec![];
    for enum_entity in enums {
        let enum_vo = SimpleEnumVO::convert(conn, Some(enum_entity))
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
