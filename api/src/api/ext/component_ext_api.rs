use crate::api::common::param::IdsParam;
use crate::app::AppState;
use actix_web::{error, get, post, web, Error, HttpResponse, Result};
use tcdt_service::sea_orm::DatabaseConnection;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_EQUAL, ORDER_DIRECTION_DESC},
        result::{AppResult, SaveResult},
    },
    dto::{
        po::ext::component::collection::ComponentPO as SavePO,
        vo::ext::component::collection::{ComponentVO as CollComponentVO, DataTypeVO},
    },
    service::{
        base::{component_service::ComponentQuery, data_type_service::DataTypeQuery,
               component_entity_service::ComponentEntityQuery, component_enum_service::ComponentEnumQuery,
               component_node_ui_service::ComponentNodeUiMutation,
               component_node_ui_service::ComponentNodeUiQuery},
        ext::component_ext_service::ComponentExtMutation,
    },
};
use tcdt_service::common::result::DeleteRefErrorMessageVO;
use tcdt_service::dto::po::base::component_po::ComponentPO;
use tcdt_service::service::base::component_service::ComponentMutation;
use tcdt_service::util::id_util::generate_id;

#[tcdt_route(ext_get_by_id)]
#[get("/component/extGetById/{id}")]
pub async fn ext_get_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let component_entity = ComponentQuery::find_by_id(conn, id).await.map_err(|e| {
        log::error!("{:?}", e);
        match e {
            TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
            _ => error::ErrorInternalServerError("internal server error"),
        }
    })?;

    let component_vo = convert_dto(conn, component_entity).await?;

    Ok(HttpResponse::Ok().json(component_vo))
}

#[tcdt_route(save_by_action)]
#[post("/component/saveByAction")]
pub async fn save_by_action(
    data: web::Data<AppState>,
    save_po: web::Json<SavePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let save_po = save_po.into_inner();

    let entity_collection_save = ComponentExtMutation::save(conn, save_po)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    match entity_collection_save {
        SaveResult::Ok(entity_collection_save) => {
            let coll_vo = convert_dto(conn, entity_collection_save).await?;
            let app_result = AppResult::success(coll_vo);
            Ok(HttpResponse::Ok().json(app_result))
        }
        SaveResult::ErrMsg(delete_error_msg_list) => {
            let err_msg_vec: Vec<String> = delete_error_msg_list
                .iter()
                .map(|m| m.message.to_owned())
                .collect();
            let err_msg = err_msg_vec.join(";\r\n");
            let app_result = AppResult::failed_msg_and_data(err_msg, delete_error_msg_list);

            Ok(HttpResponse::Ok().json(app_result))
        }
        SaveResult::None() => {
            let app_result = AppResult::<i32>::success_msg(String::from("delete success"));
            Ok(HttpResponse::Ok().json(app_result))
        }
    }
}

async fn convert_dto(
    conn: &DatabaseConnection,
    component_entity: entity::entity::component::Model,
) -> Result<CollComponentVO, Error> {
    let mut component_vo = CollComponentVO::convert(conn, Some(component_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    let id_project = component_vo
        .component_module
        .clone()
        .ok_or_else(|| {
            log::error!("can not get component_module_vo");
            error::ErrorInternalServerError("internal server error")
        })?
        .sub_project
        .clone()
        .ok_or_else(|| {
            log::error!("can not get sub_project_vo");
            error::ErrorInternalServerError("internal server error")
        })?
        .id_project
        .ok_or_else(|| {
            log::error!("can not get id_project");
            error::ErrorInternalServerError("internal server error")
        })?;

    let data_type_vos = get_data_types(id_project.clone(), conn)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    component_vo.sys_data_types = data_type_vos;

    Ok(component_vo)
}

async fn get_data_types(
    id_project: String,
    conn: &DatabaseConnection,
) -> Result<Vec<DataTypeVO>, TcdtServiceError> {
    let aq_condition: AqCondition = AqCondition {
        logic_node: Some(Box::new(AqLogicNode {
            logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_string(),
            logic_node: None,
            filter_nodes: vec![AqFilterNode {
                name: "idProject".to_string(),
                operator_code: OPERATOR_CODE_EQUAL.to_string(),
                filter_params: vec![EFilterParam::String(Some(Box::new(id_project)))],
            }],
        })),
        orders: vec![AqOrder {
            direction: ORDER_DIRECTION_DESC.to_string(),
            property: "idDataType".to_string(),
            ignore_case: false,
        }],
    };

    let data_types = DataTypeQuery::find_collection_by_condition(conn, aq_condition).await?;

    let mut data_type_vos: Vec<DataTypeVO> = vec![];
    for data_type in data_types {
        let data_type_vo = DataTypeVO::convert(conn, Some(data_type))
            .await?
            .ok_or_else(|| {
                log::error!("can not get data_type_vo");
                TcdtServiceError::build_internal_msg("can not get data_type_vo")
            })?;
        data_type_vos.push(data_type_vo);
    }
    Ok(data_type_vos)
}

#[tcdt_route(get_description_data)]
#[get("/component/getDescriptionData/{id}")]
pub async fn get_description_data(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let description = ComponentExtMutation::get_description(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    Ok(HttpResponse::Ok().json(description))
}

#[tcdt_route(remove_on_error_tip)]
#[post("/component/removeOnErrorTip")]
pub async fn remove_on_error_tip(
    data: web::Data<AppState>,
    component_form: web::Json<ComponentPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = component_form.into_inner();

    let condition = AqCondition::build_equal_condition("idComponent", EFilterParam::String(Some(Box::new(form.id_component.clone()))));
    let exists = ComponentEnumQuery::exists_by_condition(conn, condition.clone())
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut err_msg_list: Vec<DeleteRefErrorMessageVO> = vec![];
    if exists {
        let err_msg_vo = DeleteRefErrorMessageVO {
            id_data: generate_id(),
            message: "ref by ComponentEnum".to_string(),
            source_class_name: "".to_string(),
            ref_class_name: "".to_string(),
        };
        err_msg_list.push(err_msg_vo);
    }
    let exists = ComponentEntityQuery::exists_by_condition(conn, condition.clone())
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    if exists {
        let err_msg_vo = DeleteRefErrorMessageVO {
            id_data: generate_id(),
            message: "ref by ComponentEntity".to_string(),
            source_class_name: "".to_string(),
            ref_class_name: "".to_string(),
        };
        err_msg_list.push(err_msg_vo);
    }
    if !err_msg_list.is_empty() {
        let result = AppResult::<Vec<DeleteRefErrorMessageVO>>::failed_msg_and_data("".to_string(), err_msg_list);
        return Ok(HttpResponse::Ok().json(result));
    }
    let node_ui_list = ComponentNodeUiQuery::find_collection_by_condition(conn, condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    ComponentNodeUiMutation::batch_delete(conn, node_ui_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    ComponentMutation::delete(conn, form)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let result = AppResult::<i32>::success_not_data();
    Ok(HttpResponse::Ok().json(result))
}