use crate::api::common::param::IdsParam;
use crate::app::AppState;
use actix_web::{error, get, post, web, Error, HttpRequest, HttpResponse, Result};
use tcdt_service::sea_orm::DatabaseConnection;
use std::collections::HashMap;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::tcdt_route;
use tcdt_service::common::aq_const::{DO_NEW, ORDER_DIRECTION_ASC};
use tcdt_service::dto::po::ext::entity_collection::collection_po;
use tcdt_service::dto::po::ext::entity_collection::collection_po::{
    DdEntityPO, DdEnumPO, EntityAssociatePO,
};
use tcdt_service::service::base::entity_attribute_service::EntityAttributeQuery;
use tcdt_service::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_EQUAL},
        result::{AppResult, SaveResult},
    },
    dto::{
        po::ext::entity_collection::collection_po::EntityCollectionPO as SaveCollVO,
        vo::ext::entity_collection::{
            entity_collection_vo::{
                CommonAttributeVO, DataTypeVO, DdEntityVO as CollectionEntityVO, DdEntityVO,
                DdEnumVO as CollectionEnumVO, DdEnumVO, EntityAssociateVO, EntityAttributeVO,
                EntityCollectionVO as CollectionVO,
            },
            simple_coll_vo::EntityCollectionVO as SimpleCollVO,
        },
    },
    service::{
        base::{
            common_attribute_service::CommonAttributeQuery, data_type_service::DataTypeQuery,
            dd_entity_service::DdEntityQuery, dd_enum_service::DdEnumQuery,
            entity_collection_service::EntityCollectionQuery,
        },
        ext::entity_collection_ext_service::EntityCollectionExtMutation,
    },
};
use tcdt_service::dto::po::ext::entity_collection::join_entity_po::JoinEntityPO;

#[tcdt_route(ext_get_by_id)]
#[get("/entityCollection/extGetById")]
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

    let entity_collection_entity =
        EntityCollectionQuery::find_by_id(conn, id)
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

    let coll_vo = convert_dto(conn, entity_collection_entity).await?;

    Ok(HttpResponse::Ok().json(coll_vo))
}

#[tcdt_route(get_full_coll)]
#[get("/entityCollection/getFullColl")]
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

    let entity_collection_entity =
        EntityCollectionQuery::find_by_id(conn, id)
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

    let mut coll_vo = convert_dto(conn, entity_collection_entity).await?;

    fill_attribute(conn, &mut coll_vo).await?;

    Ok(HttpResponse::Ok().json(coll_vo))
}

#[tcdt_route(get_simple_collection)]
#[get("/entityCollection/getSimpleCollection")]
pub async fn get_simple_collection(
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
        .get("idEntityCollection")
        .ok_or(error::ErrorInternalServerError(
            "idEntityCollection not found",
        ))?
        .to_string();

    let entity_collection_entity =
        EntityCollectionQuery::find_by_id(conn, id)
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

    let coll_vo = SimpleCollVO::convert(conn, Some(entity_collection_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| {
            log::error!("can not get entity_collection_vo");
            error::ErrorInternalServerError("internal server error")
        })?;

    Ok(HttpResponse::Ok().json(coll_vo))
}

#[tcdt_route(save_by_action)]
#[post("/entityCollection/saveByAction")]
pub async fn save_by_action(
    data: web::Data<AppState>,
    save_po: web::Json<SaveCollVO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let save_po = save_po.into_inner();

    let entity_collection_save = EntityCollectionExtMutation::save(conn, save_po)
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
    entity_collection_entity: entity::entity::entity_collection::Model,
) -> Result<CollectionVO, Error> {
    let mut collection_vo = CollectionVO::convert(conn, Some(entity_collection_entity))
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?
        .ok_or_else(|| {
            log::error!("can not get entity_collection_vo");
            error::ErrorInternalServerError("internal server error")
        })?;

    let out_entities = get_out_entities(&collection_vo, conn).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;
    collection_vo.out_entities = out_entities;

    let out_enums = get_out_enums(&collection_vo, conn).await.map_err(|e| {
        log::error!("{:?}", e);
        error::ErrorInternalServerError("internal server error")
    })?;
    collection_vo.out_enums = out_enums;

    let id_project = collection_vo
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

    collection_vo.sys_data_types = data_type_vos;

    let common_attribute_vos = get_common_attributes(id_project.clone(), conn)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    collection_vo.common_attributes = common_attribute_vos;
    return Ok(collection_vo);
}

async fn get_out_entities(
    collection_vo: &CollectionVO,
    conn: &DatabaseConnection,
) -> Result<Vec<CollectionEntityVO>, TcdtServiceError> {
    let entity_ids: Vec<String> = collection_vo
        .entities
        .iter()
        .map(|enti| enti.id_entity.clone())
        .collect();
    let out_ids: Vec<String> = collection_vo
        .entity_associates
        .iter()
        .filter(|asso| !entity_ids.contains(&asso.id_up.clone().unwrap()))
        .map(|asso| asso.id_up.clone().unwrap())
        .collect();
    let entities = DdEntityQuery::find_by_ids(conn, out_ids).await?;
    let mut out_entities: Vec<CollectionEntityVO> = vec![];
    for enti in entities {
        let out_entity = CollectionEntityVO::convert(conn, Some(enti))
            .await?
            .ok_or_else(|| {
                log::error!("can not get out entity");
                TcdtServiceError::build_internal_msg("can not get out entity")
            })?;
        out_entities.push(out_entity);
    }
    Ok(out_entities)
}

async fn get_out_enums(
    collection_vo: &CollectionVO,
    conn: &DatabaseConnection,
) -> Result<Vec<CollectionEnumVO>, TcdtServiceError> {
    let entity_ids: Vec<String> = collection_vo
        .enums
        .iter()
        .map(|enti| enti.id_enum.clone())
        .collect();
    let out_ids: Vec<String> = collection_vo
        .enum_associates
        .iter()
        .filter(|asso| !entity_ids.contains(&asso.id_enum.clone().unwrap()))
        .map(|asso| asso.id_enum.clone().unwrap())
        .collect();
    let entities = DdEnumQuery::find_by_ids(conn, out_ids).await?;
    let mut out_entities: Vec<CollectionEnumVO> = vec![];
    for enti in entities {
        let out_entity = CollectionEnumVO::convert(conn, Some(enti))
            .await?
            .ok_or_else(|| {
                log::error!("can not get out enum");
                TcdtServiceError::build_internal_msg("can not get out enum")
            })?;
        out_entities.push(out_entity);
    }
    Ok(out_entities)
}

async fn get_common_attributes(
    id_project: String,
    conn: &DatabaseConnection,
) -> Result<Vec<CommonAttributeVO>, TcdtServiceError> {
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
            direction: ORDER_DIRECTION_ASC.to_string(),
            property: "sn".to_string(),
            ignore_case: false,
        }],
    };

    let common_attributes =
        CommonAttributeQuery::find_collection_by_condition(conn, aq_condition).await?;

    let mut common_attribute_vos: Vec<CommonAttributeVO> = vec![];
    for common_attribute in common_attributes {
        let data_type_vo = CommonAttributeVO::convert(conn, Some(common_attribute))
            .await?
            .ok_or_else(|| {
                log::error!("can not get data_type_vo");
                TcdtServiceError::build_internal_msg("can not get data_type_vo")
            })?;
        common_attribute_vos.push(data_type_vo);
    }
    Ok(common_attribute_vos)
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
            direction: ORDER_DIRECTION_ASC.to_string(),
            property: "sn".to_string(),
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

async fn fill_attribute(
    conn: &DatabaseConnection,
    entity_collection_vo: &mut CollectionVO,
) -> Result<(), Error> {
    for entity_vo in &mut entity_collection_vo.entities {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_string(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idEntity".to_string(),
                    operator_code: OPERATOR_CODE_EQUAL.to_string(),
                    filter_params: vec![EFilterParam::String(Some(Box::new(
                        entity_vo.id_entity.clone(),
                    )))],
                }],
            })),
            orders: vec![],
        };
        let attr_list =
            EntityAttributeQuery::find_collection_by_condition(conn, aq_condition.clone())
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
            let attr_vo = EntityAttributeVO::convert(conn, Some(attr_entity))
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    error::ErrorInternalServerError("internal server error")
                })?
                .unwrap();
            attr_vo_list.push(attr_vo);
        }
        entity_vo.attributes = attr_vo_list;
    }
    Ok(())
}

#[tcdt_route(copy_by_id)]
#[get("/entityCollection/copyById")]
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

    let entity_collection_entity =
        EntityCollectionQuery::find_by_id(conn, id)
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

    let mut coll_vo = convert_dto(conn, entity_collection_entity).await?;
    coll_vo.display_name = Some(format!("{}-copy", coll_vo.display_name.unwrap_or_default()));
    fill_attribute(conn, &mut coll_vo).await?;

    let save_po = convert_vo_to_save_po(coll_vo);

    let entity_collection_save =
        EntityCollectionExtMutation::insert_or_update_by_action(conn, save_po)
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
            let app_result = AppResult::<i32>::success_msg(String::from("copy success"));
            Ok(HttpResponse::Ok().json(app_result))
        }
    }
}

fn convert_vo_to_save_po(coll_vo: CollectionVO) -> collection_po::EntityCollectionPO {
    let mut save_po = collection_po::EntityCollectionPO {
        action: DO_NEW,
        id_entity_collection: coll_vo.id_entity_collection.clone(),
        package_name: coll_vo.package_name.clone(),
        display_name: coll_vo.display_name.clone(),
        id_sub_project: coll_vo.id_sub_project.clone(),
        entity_associates: vec![],
        node_uis: vec![],
        enum_associates: vec![],
        entities: vec![],
        enums: vec![],
    };
    save_po.entities = coll_vo
        .entities
        .iter()
        .map(|entity_vo| {
            let entity_po = convert_to_entity_po(entity_vo);
            return entity_po;
        })
        .collect();
    save_po.enums = coll_vo
        .enums
        .iter()
        .map(|enum_vo| {
            let enum_po = convert_to_enum_po(enum_vo);
            return enum_po;
        })
        .collect();
    save_po.node_uis = coll_vo
        .node_uis
        .iter()
        .map(|node_ui_vo| collection_po::NodeUiPO {
            action: DO_NEW,
            id_node_ui: node_ui_vo.id_node_ui.clone(),
            x: node_ui_vo.x.clone(),
            y: node_ui_vo.y.clone(),
            width: node_ui_vo.width.clone(),
            height: node_ui_vo.height.clone(),
            id_element: node_ui_vo.id_element.clone(),
            id_entity_collection: node_ui_vo.id_entity_collection.clone(),
        })
        .collect();
    save_po.entity_associates = coll_vo
        .entity_associates
        .iter()
        .map(|asso_vo| convert_to_entity_assosiciate_po(asso_vo))
        .collect();
    save_po.enum_associates = coll_vo
        .enum_associates
        .iter()
        .map(|enum_asso_vo| collection_po::EnumAssociatePO {
            action: DO_NEW,
            id_enum_associate: enum_asso_vo.id_enum_associate.clone(),
            group_order: enum_asso_vo.group_order.clone(),
            id_enum: enum_asso_vo.id_enum.clone(),
            id_entity: enum_asso_vo.id_entity.clone(),
            id_entity_collection: enum_asso_vo.id_entity_collection.clone(),
            id_attribute: enum_asso_vo.id_attribute.clone(),
        })
        .collect();
    save_po
}

fn convert_to_entity_assosiciate_po(asso_vo: &EntityAssociateVO) -> EntityAssociatePO {
    collection_po::EntityAssociatePO {
        action: DO_NEW,
        id_entity_associate: asso_vo.id_entity_associate.clone(),
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
        fg_foreign_key: asso_vo.fg_foreign_key.clone(),
        down_order_str: asso_vo.down_order_str.clone(),
        down_batch_size: asso_vo.down_batch_size.clone(),
        id_up: asso_vo.id_up.clone(),
        id_entity_collection: asso_vo.id_entity_collection.clone(),
        id_down: asso_vo.id_down.clone(),
    }
}

fn convert_to_enum_po(enum_vo: &DdEnumVO) -> DdEnumPO {
    let mut enum_po = collection_po::DdEnumPO {
        action: DO_NEW,
        id_enum: enum_vo.id_enum.clone(),
        class_name: enum_vo.class_name.clone(),
        display_name: enum_vo.display_name.clone(),
        enum_value_type: enum_vo.enum_value_type.clone(),
        id_entity_collection: enum_vo.id_entity_collection.clone(),
        attributes: vec![],
    };
    enum_po.attributes = enum_vo
        .attributes
        .iter()
        .map(|attr_vo| collection_po::EnumAttributePO {
            action: DO_NEW,
            id_enum_attribute: attr_vo.id_enum_attribute.clone(),
            display_name: attr_vo.display_name.clone(),
            code: attr_vo.code.clone(),
            enum_value: attr_vo.enum_value.clone(),
            sn: attr_vo.sn.clone(),
            id_enum: attr_vo.id_enum.clone(),
        })
        .collect();
    enum_po
}

fn convert_to_entity_po(entity_vo: &DdEntityVO) -> DdEntityPO {
    let mut entity_po = collection_po::DdEntityPO {
        action: DO_NEW,
        id_entity: entity_vo.id_entity.clone(),
        display_name: entity_vo.display_name.clone(),
        class_name: entity_vo.class_name.clone(),
        table_name: entity_vo.table_name.clone(),
        pk_attribute_code: entity_vo.pk_attribute_code.clone(),
        pk_attribute_name: entity_vo.pk_attribute_name.clone(),
        pk_attribute_type_name: entity_vo.pk_attribute_type_name.clone(),
        id_entity_collection: entity_vo.id_entity_collection.clone(),
        attributes: vec![],
    };
    entity_po.attributes = entity_vo
        .attributes
        .iter()
        .map(|attr_vo| collection_po::EntityAttributePO {
            action: DO_NEW,
            id_attribute: attr_vo.id_attribute.clone(),
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
            id_entity: attr_vo.id_entity.clone(),
            id_attribute_type: attr_vo.id_attribute_type.clone(),
        })
        .collect();
    entity_po
}

/// join entity from other collection
#[tcdt_route(join_entities)]
#[post("/entityCollection/joinEntities")]
pub async fn join_entities(
    data: web::Data<AppState>,
    join_po: web::Json<JoinEntityPO>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let join_po = join_po.into_inner();
    let id_entity_collection = join_po.entity_collection.id_entity_collection.clone();
    EntityCollectionExtMutation::join_entities(conn, join_po)
        .await
        .map_err(|e| {
            log::error!("join_entities error: {:?}", e);
            error::ErrorInternalServerError("internal server error")
        })?;

    let entity_collection_entity =
        EntityCollectionQuery::find_by_id(conn, id_entity_collection)
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

    let coll_vo = convert_dto(conn, entity_collection_entity).await?;

    Ok(HttpResponse::Ok().json(coll_vo))
}
