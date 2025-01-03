use crate::api::common::param::IdsParam;
use crate::app::AppState;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_service::sea_orm::DatabaseConnection;
use std::collections::HashMap;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::common::aq_const::DO_NEW;
use tcdt_service::dto::po::ext::dto_collection::collection;
use tcdt_service::dto::vo::ext::dto_collection::entity_collection::DtoEntityCollectionVO;
use tcdt_service::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_EQUAL, ORDER_DIRECTION_DESC},
        result::{AppResult, SaveResult},
    },
    dto::{
        po::ext::dto_collection::collection::DtoEntityCollectionPO as SavePO,
        vo::ext::dto_collection::entity_collection::{
            DataTypeVO, DtoComputationAttributeVO, DtoEntityAssociateVO, DtoEntityAttributeVO,
            DtoEntityCollectionVO as CollCollectionVO, DtoEntityVO, DtoEnumVO,
        },
    },
    service::{
        base::{
            data_type_service::DataTypeQuery,
            dto_computation_attribute_service::DtoComputationAttributeQuery,
            dto_entity_attribute_service::DtoEntityAttributeQuery,
            dto_entity_collection_service::DtoEntityCollectionQuery,
        },
        ext::dto_entity_collection_ext_service::DtoEntityCollectionExtMutation,
    },
};
use tcdt_service::common::result::DeleteRefErrorMessageVO;
use tcdt_service::service::base::dto_entity_collection_service::DtoEntityCollectionMutation;
use tcdt_service::service::base::dto_entity_service::DtoEntityQuery;
use tcdt_service::service::base::dto_enum_service::DtoEnumQuery;
use tcdt_service::service::base::dto_node_ui_service::{DtoNodeUiMutation, DtoNodeUiQuery};
use tcdt_service::util::id_util::generate_id;
use ::entity::entity::dto_entity_collection;

#[tcdt_route(ext_get_by_id)]
#[get("/dtoEntityCollection/extGetById")]
pub async fn ext_get_by_id(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let id = query_params
        .get("id")
        .ok_or(error::ErrorInternalServerError("id not found"))?
        .to_string();

    let dto_entity_collection_entity = DtoEntityCollectionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let dto_entity_collection_vo = convert_dto(conn, dto_entity_collection_entity).await?;

    Ok(HttpResponse::Ok().json(dto_entity_collection_vo))
}

#[tcdt_route(get_full_coll)]
#[get("/dtoEntityCollection/getFullColl")]
pub async fn get_full_coll(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let id = query_params
        .get("id")
        .ok_or(error::ErrorInternalServerError("id not found"))?
        .to_string();

    let dto_entity_collection_entity = DtoEntityCollectionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut dto_entity_collection_vo = convert_dto(conn, dto_entity_collection_entity).await?;
    dto_entity_collection_vo.display_name = Some(format!(
        "{}{}",
        dto_entity_collection_vo.display_name.unwrap_or_default(),
        "-copy"
    ));

    fill_attribute(conn, &mut dto_entity_collection_vo).await?;

    Ok(HttpResponse::Ok().json(dto_entity_collection_vo))
}

async fn fill_attribute(
    conn: &DatabaseConnection,
    dto_entity_collection_vo: &mut DtoEntityCollectionVO,
) -> Result<(), Error> {
    for dto_entity_vo in &mut dto_entity_collection_vo.dto_entities {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_string(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idDtoEntity".to_string(),
                    operator_code: OPERATOR_CODE_EQUAL.to_string(),
                    filter_params: vec![EFilterParam::String(Some(Box::new(
                        dto_entity_vo.id_dto_entity.clone(),
                    )))],
                }],
            })),
            orders: vec![],
        };
        let attr_list =
            DtoEntityAttributeQuery::find_collection_by_condition(conn, aq_condition.clone())
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
        let mut attr_vo_list = vec![];
        for attr_entity in attr_list {
            let attr_vo = DtoEntityAttributeVO::convert(conn, Some(attr_entity))
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?
                .unwrap();
            attr_vo_list.push(attr_vo);
        }
        dto_entity_vo.de_attributes = attr_vo_list;
        let dc_attr_list =
            DtoComputationAttributeQuery::find_collection_by_condition(conn, aq_condition.clone())
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
        let mut dc_attr_vo_list = vec![];
        for dc_attr_entity in dc_attr_list {
            let dc_attr_vo = DtoComputationAttributeVO::convert(conn, Some(dc_attr_entity))
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?
                .unwrap();
            dc_attr_vo_list.push(dc_attr_vo);
        }
        dto_entity_vo.dc_attributes = dc_attr_vo_list;
    }
    Ok(())
}

#[tcdt_route(save_by_action)]
#[post("/dtoEntityCollection/saveByAction")]
pub async fn save_by_action(
    data: web::Data<AppState>,
    save_po: web::Json<SavePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let save_po = save_po.into_inner();

    let entity_collection_save: SaveResult<entity::entity::dto_entity_collection::Model> =
        DtoEntityCollectionExtMutation::save(conn, save_po)
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
    dto_entity_collection_entity: entity::entity::dto_entity_collection::Model,
) -> Result<CollCollectionVO, Error> {
    let mut dto_entity_collection_vo =
        CollCollectionVO::convert(conn, Some(dto_entity_collection_entity))
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                error::ErrorInternalServerError("internal server error")
            })?
            .ok_or_else(|| error::ErrorInternalServerError("internal server error"))?;

    let id_project = dto_entity_collection_vo
        .dto_module
        .clone()
        .ok_or_else(|| {
            log::error!("can not get dto_module");
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

    dto_entity_collection_vo.sys_data_types = data_type_vos;
    Ok(dto_entity_collection_vo)
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

#[tcdt_route(copy_by_id)]
#[get("/dtoEntityCollection/copyById")]
pub async fn copy_by_id(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let query_params = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .map_err(|e| {
            log::error!("QueryPayloadError: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let id = query_params
        .get("id")
        .ok_or(error::ErrorInternalServerError("id not found"))?
        .to_string();

    let entity_collection_entity = DtoEntityCollectionQuery::find_by_id(conn, id)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                TcdtServiceError::Custom(cus) => error::ErrorInternalServerError(cus.get_message()),
                _ => error::ErrorInternalServerError("internal server error"),
            }
        })?;

    let mut coll_vo = convert_dto(conn, entity_collection_entity).await?;
    coll_vo.display_name = Some(format!("{}-copy", coll_vo.display_name.unwrap_or_default()));

    fill_attribute(conn, &mut coll_vo).await?;

    let save_po = convert_vo_to_save_po(coll_vo);

    let entity_collection_save =
        DtoEntityCollectionExtMutation::insert_or_update_by_action(conn, save_po)
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

fn convert_vo_to_save_po(coll_vo: DtoEntityCollectionVO) -> collection::DtoEntityCollectionPO {
    let mut save_po = collection::DtoEntityCollectionPO {
        action: DO_NEW,
        id_dto_entity_collection: coll_vo.id_dto_entity_collection.clone(),
        package_name: coll_vo.package_name.clone(),
        display_name: coll_vo.display_name.clone(),
        id_main_dto_entity: coll_vo.id_main_dto_entity.clone(),
        id_dto_module: coll_vo.id_dto_module.clone(),
        de_associates: vec![],
        dto_enum_associates: vec![],
        dto_enums: vec![],
        dto_node_uis: vec![],
        dto_entities: vec![],
    };
    save_po.dto_entities = coll_vo
        .dto_entities
        .iter()
        .map(|entity_vo| {
            let entity_po = convert_to_entity_po(entity_vo);
            return entity_po;
        })
        .collect();
    save_po.dto_enums = coll_vo
        .dto_enums
        .iter()
        .map(|enum_vo| {
            let enum_po = convert_to_enum_po(enum_vo);
            return enum_po;
        })
        .collect();
    save_po.dto_node_uis = coll_vo
        .dto_node_uis
        .iter()
        .map(|node_ui_vo| collection::DtoNodeUiPO {
            action: DO_NEW,
            id_dto_node_ui: node_ui_vo.id_dto_node_ui.clone(),
            x: node_ui_vo.x.clone(),
            y: node_ui_vo.y.clone(),
            width: node_ui_vo.width.clone(),
            height: node_ui_vo.height.clone(),
            id_element: node_ui_vo.id_element.clone(),
            id_dto_entity_collection: node_ui_vo.id_dto_entity_collection.clone(),
        })
        .collect();
    save_po.de_associates = coll_vo
        .de_associates
        .iter()
        .map(|asso_vo| convert_to_entity_assosiciate_po(asso_vo))
        .collect();
    save_po.dto_enum_associates = coll_vo
        .dto_enum_associates
        .iter()
        .map(|enum_asso_vo| collection::DtoEnumAssociatePO {
            action: DO_NEW,
            id_dto_enum_associate: enum_asso_vo.id_dto_enum_associate.clone(),
            group_order: enum_asso_vo.group_order.clone(),
            id_dto_enum: enum_asso_vo.id_dto_enum.clone(),
            id_dto_entity: enum_asso_vo.id_dto_entity.clone(),
            id_dto_entity_collection: enum_asso_vo.id_dto_entity_collection.clone(),
            id_dto_entity_attribute: enum_asso_vo.id_dto_entity_attribute.clone(),
        })
        .collect();
    save_po
}

fn convert_to_entity_assosiciate_po(
    asso_vo: &DtoEntityAssociateVO,
) -> collection::DtoEntityAssociatePO {
    collection::DtoEntityAssociatePO {
        action: DO_NEW,
        id_dto_entity_associate: asso_vo.id_dto_entity_associate.clone(),
        group_order: asso_vo.group_order.clone(),
        up_associate_type: asso_vo.up_associate_type.clone(),
        down_associate_type: asso_vo.down_associate_type.clone(),
        down_attribute_name: asso_vo.down_attribute_name.clone(),
        down_attribute_display_name: asso_vo.down_attribute_display_name.clone(),
        ref_attribute_name: asso_vo.ref_attribute_name.clone(),
        ref_attribute_display_name: asso_vo.ref_attribute_display_name.clone(),
        fk_column_name: asso_vo.fk_column_name.clone(),
        fk_attribute_name: asso_vo.fk_attribute_name.clone(),
        fk_attribute_display_name: asso_vo.fk_attribute_display_name.clone(),
        id_up: asso_vo.id_up.clone(),
        id_dto_entity_collection: asso_vo.id_dto_entity_collection.clone(),
        id_down: asso_vo.id_down.clone(),
    }
}

fn convert_to_enum_po(enum_vo: &DtoEnumVO) -> collection::DtoEnumPO {
    let mut enum_po = collection::DtoEnumPO {
        action: DO_NEW,
        id_dto_enum: enum_vo.id_dto_enum.clone(),
        class_name: enum_vo.class_name.clone(),
        display_name: enum_vo.display_name.clone(),
        enum_value_type: enum_vo.enum_value_type.clone(),
        id_ref: enum_vo.id_ref.clone(),
        id_dto_entity_collection: enum_vo.id_dto_entity_collection.clone(),
        dto_enum_attributes: vec![],
    };
    enum_po.dto_enum_attributes = enum_vo
        .dto_enum_attributes
        .iter()
        .map(|attr_vo| collection::DtoEnumAttributePO {
            action: DO_NEW,
            id_dto_enum_attribute: attr_vo.id_dto_enum_attribute.clone(),
            display_name: attr_vo.display_name.clone(),
            code: attr_vo.code.clone(),
            enum_value: attr_vo.enum_value.clone(),
            sn: attr_vo.sn.clone(),
            id_dto_enum: attr_vo.id_dto_enum.clone(),
            id_ref: attr_vo.id_ref.clone(),
        })
        .collect();
    enum_po
}

fn convert_to_entity_po(entity_vo: &DtoEntityVO) -> collection::DtoEntityPO {
    let mut entity_po = collection::DtoEntityPO {
        action: DO_NEW,
        id_dto_entity: entity_vo.id_dto_entity.clone(),
        display_name: entity_vo.display_name.clone(),
        class_name: entity_vo.class_name.clone(),
        table_name: entity_vo.table_name.clone(),
        pk_attribute_code: entity_vo.pk_attribute_code.clone(),
        pk_attribute_name: entity_vo.pk_attribute_name.clone(),
        pk_attribute_type_name: entity_vo.pk_attribute_type_name.clone(),
        id_ref: entity_vo.id_ref.clone(),
        id_dto_entity_collection: entity_vo.id_dto_entity_collection.clone(),
        de_attributes: vec![],
        dc_attributes: vec![],
    };
    entity_po.de_attributes = entity_vo
        .de_attributes
        .iter()
        .map(|attr_vo| collection::DtoEntityAttributePO {
            action: DO_NEW,
            id_dto_entity_attribute: attr_vo.id_dto_entity_attribute.clone(),
            attribute_name: attr_vo.attribute_name.clone(),
            display_name: attr_vo.display_name.clone(),
            column_name: attr_vo.column_name.clone(),
            fg_primary_key: attr_vo.fg_primary_key.clone(),
            fg_mandatory: attr_vo.fg_mandatory.clone(),
            default_value: attr_vo.default_value.clone(),
            len: attr_vo.len.clone(),
            pcs: attr_vo.pcs.clone(),
            sn: attr_vo.sn.clone(),
            note: attr_vo.note.clone(),
            category: attr_vo.category.clone(),
            id_dto_entity: attr_vo.id_dto_entity.clone(),
            id_attribute_type: attr_vo.id_attribute_type.clone(),
            id_ref_attribute: attr_vo.id_ref_attribute.clone(),
        })
        .collect();
    entity_po.dc_attributes = entity_vo
        .dc_attributes
        .iter()
        .map(|attr_vo| collection::DtoComputationAttributePO {
            action: DO_NEW,
            id_dto_computation_attribute: attr_vo.id_dto_computation_attribute.clone(),
            attribute_name: attr_vo.attribute_name.clone(),
            display_name: attr_vo.display_name.clone(),
            note: attr_vo.note.clone(),
            len: attr_vo.len.clone(),
            fg_mandatory: attr_vo.fg_mandatory.clone(),
            default_value: attr_vo.default_value.clone(),
            pcs: attr_vo.pcs.clone(),
            sn: attr_vo.sn.clone(),
            id_attribute_type: attr_vo.id_attribute_type.clone(),
            id_dto_entity: attr_vo.id_dto_entity.clone(),
        })
        .collect();
    entity_po
}

#[tcdt_route(remove_on_error_tip)]
#[post("/dtoEntityCollection/removeOnErrorTip")]
pub async fn remove_on_error_tip(
    data: web::Data<AppState>,
    collection_form: web::Json<SavePO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = collection_form.into_inner();

    let condition = AqCondition::build_equal_condition("idDtoEntityCollection", EFilterParam::String(Some(Box::new(form.id_dto_entity_collection.clone()))));
    let exists = DtoEnumQuery::exists_by_condition(conn, condition.clone())
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let mut err_msg_list: Vec<DeleteRefErrorMessageVO> = vec![];
    if exists {
        let err_msg_vo = DeleteRefErrorMessageVO {
            id_data: generate_id(),
            message: "ref by DtoEnum".to_string(),
            source_class_name: "".to_string(),
            ref_class_name: "".to_string(),
        };
        err_msg_list.push(err_msg_vo);
    }
    let exists = DtoEntityQuery::exists_by_condition(conn, condition.clone())
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    if exists {
        let err_msg_vo = DeleteRefErrorMessageVO {
            id_data: generate_id(),
            message: "ref by DtoEntity".to_string(),
            source_class_name: "".to_string(),
            ref_class_name: "".to_string(),
        };
        err_msg_list.push(err_msg_vo);
    }
    if !err_msg_list.is_empty() {
        let result = AppResult::<Vec<DeleteRefErrorMessageVO>>::failed_msg_and_data("".to_string(), err_msg_list);
        return Ok(HttpResponse::Ok().json(result));
    }
    let node_ui_list = DtoNodeUiQuery::find_collection_by_condition(conn, condition)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    DtoNodeUiMutation::batch_delete(conn, node_ui_list)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let dto_entity_collection_model = dto_entity_collection::Model{
        id_dto_entity_collection: form.id_dto_entity_collection.clone(),
        package_name: form.package_name.clone(),
        display_name: form.display_name.clone(),
        id_main_dto_entity: form.id_main_dto_entity.clone(),
        id_dto_module: form.id_dto_module.clone(),
    };
    DtoEntityCollectionMutation::delete(conn, dto_entity_collection_model)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;
    let result = AppResult::<i32>::success_not_data();
    Ok(HttpResponse::Ok().json(result))
}
