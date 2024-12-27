use crate::{
    common::{
        aq_const::{DO_DELETE, DO_NEW, DO_UPDATE},
        result::{DeleteRefErrorMessageVO, SaveResult},
    },
    dto::po::ext::entity_collection::collection_po::EntityCollectionPO as SavePO,
};
use ::entity::entity::{
    component_entity, component_entity_associate, component_enum, dd_entity, dd_enum,
    entity_associate, entity_attribute,
    entity_collection::{
        ActiveModel as EntityCollectionActiveModel, Entity as EntityCollectionEntity,
        Model as EntityCollectionModel,
    },
    enum_associate, enum_attribute, ext_attribute, node_ui,
};
use sea_orm::*;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::common::aq::{AqCondition, EFilterParam};
use crate::dto::po::ext::entity_collection::join_entity_po::JoinEntityPO;
use crate::service::base::{dd_entity_service::DdEntityQuery,
                           dd_enum_service::DdEnumQuery, entity_associate_service,
                           entity_associate_service::EntityAssociateQuery,
                           node_ui_service::NodeUiQuery,
                           enum_associate_service::EnumAssociateQuery};
use crate::util::id_util::generate_id;

pub struct EntityCollectionExtMutation;

impl EntityCollectionExtMutation {
    pub async fn save(
        db: &DbConn,
        save_po: SavePO,
    ) -> Result<SaveResult<EntityCollectionModel>, TcdtServiceError> {
        let error_list = delete_check_by_action(db, &save_po).await?;
        if error_list.len() > 0 {
            return Ok(SaveResult::ErrMsg(error_list));
        }
        let txn = db.begin().await.map_err(|err| {
            log::error!("tx begin failed");
            TcdtServiceError::build_internal_msg_error("tx begin failed", err)
        })?;
        let save_po = delete_by_action(&txn, save_po).await?;
        match save_po {
            Some(mut save_po) => {
                let after_save = insert_or_update_by_action(&txn, &mut save_po).await?;
                txn.commit().await.map_err(|err| {
                    log::error!("tx commit failed");
                    TcdtServiceError::build_internal_msg_error("tx commit failed", err)
                })?;
                Ok(SaveResult::Ok(after_save))
            }
            None => {
                txn.commit().await.map_err(|err| {
                    log::error!("tx commit failed");
                    TcdtServiceError::build_internal_msg_error("tx commit failed", err)
                })?;
                Ok(SaveResult::None())
            }
        }
    }

    pub async fn insert_or_update_by_action(
        db: &DbConn,
        save_po: SavePO,
    ) -> Result<SaveResult<EntityCollectionModel>, TcdtServiceError> {
        let txn = db.begin().await.map_err(|err| {
            log::error!("tx begin failed");
            TcdtServiceError::build_internal_msg_error("tx begin failed", err)
        })?;
        let mut save_po = save_po.clone();
        let after_save = insert_or_update_by_action(&txn, &mut save_po)
            .await
            .map_err(|err| {
                log::error!("tx commit failed");
                TcdtServiceError::build_internal_msg_error("tx commit failed", err)
            })?;
        txn.commit().await.map_err(|err| {
            log::error!("tx commit failed");
            TcdtServiceError::build_internal_msg_error("tx commit failed", err)
        })?;
        Ok(SaveResult::Ok(after_save))
    }

    /// join entity from other collection
    pub async fn join_entities(
        db: &DbConn,
        join_po: JoinEntityPO,
    ) -> Result<(), TcdtServiceError> {
        let txn = db.begin().await.map_err(|err| {
            log::error!("tx begin failed");
            TcdtServiceError::build_internal_msg_error("tx begin failed", err)
        })?;
        let mut element_count = 0;
        if !join_po.entity_ids.is_empty() {
            // entity
            let mut entity_model_list = DdEntityQuery::find_by_ids(db, join_po.entity_ids.clone()).await
                .map_err(|err| {
                    log::error!("join_entities entity find_by_ids failed");
                    TcdtServiceError::build_internal_msg_error("join_entities entity find_by_ids failed", err)
                })?;
            entity_model_list.iter_mut().for_each(|entity_model| {
                entity_model.id_entity_collection = Some(join_po.entity_collection.id_entity_collection.clone());
            });
            for entity_model in entity_model_list {
                dd_entity::Entity::update(dd_entity::convert_model_to_active_model(entity_model))
                    .exec(&txn)
                    .await
                    .map_err(|err| {
                        log::error!("join_entities update entity failed");
                        TcdtServiceError::build_internal_msg_error("join_entities update entity failed", err)
                    })?;
            }
            let filter_params = join_po.entity_ids.clone()
                .iter()
                .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
                .collect::<Vec<EFilterParam>>();
            let aq_condition = AqCondition::build_in_condition("idDown", filter_params.clone());
            // entity_associate
            let mut entity_associate_model_list = EntityAssociateQuery::find_collection_by_condition(db, aq_condition.clone()).await
                .map_err(|err| {
                    log::error!("join_entities find entity_associate failed");
                    TcdtServiceError::build_internal_msg_error("join_entities find entity_associate failed", err)
                })?;
            entity_associate_model_list.iter_mut().for_each(|entity_associate_model| {
                entity_associate_model.id_entity_collection = Some(join_po.entity_collection.id_entity_collection.clone());
            });
            let mut out_entity_id_list = entity_associate_model_list.iter().map(|entity_associate_model| {
                entity_associate_model.id_up.clone().unwrap_or_default()
            }).collect::<Vec<String>>();
            for entity_associate_model in entity_associate_model_list {
                entity_associate::Entity::update(entity_associate::convert_model_to_active_model(entity_associate_model))
                    .exec(&txn)
                    .await
                    .map_err(|err| {
                        log::error!("join_entities update entity_associate failed");
                        TcdtServiceError::build_internal_msg_error("join_entities update entity_associate failed", err)
                    })?;
            }
            // enum_associate
            let aq_condition = AqCondition::build_in_condition("idEntity", filter_params.clone());
            let mut enum_associate_model_list = EnumAssociateQuery::find_collection_by_condition(db, aq_condition).await
                .map_err(|err| {
                    log::error!("join_entities find enum_associate failed");
                    TcdtServiceError::build_internal_msg_error("join_entities find enum_associate failed", err)
                })?;
            enum_associate_model_list.iter_mut().for_each(|enum_associate_model| {
                enum_associate_model.id_entity_collection = Some(join_po.entity_collection.id_entity_collection.clone());
            });
            let mut out_enum_id_list = enum_associate_model_list.iter().map(|enum_associate_model| {
                enum_associate_model.id_enum.clone().unwrap_or_default()
            }).collect::<Vec<String>>();
            for enum_associate_model in enum_associate_model_list {
                enum_associate::Entity::update(enum_associate::convert_model_to_active_model(enum_associate_model))
                    .exec(&txn)
                    .await
                    .map_err(|err| {
                        log::error!("join_entities update enum_associate failed");
                        TcdtServiceError::build_internal_msg_error("join_entities update enum_associate failed", err)
                    })?;
            }
            let mut all_out_id_list: Vec<String> = join_po.entity_ids.clone();
            all_out_id_list.append(&mut out_entity_id_list);
            all_out_id_list.append(&mut out_enum_id_list);
            let mut node_ui_active_model_list: Vec<node_ui::ActiveModel> = vec![];
            for id_element in all_out_id_list {
                let node_ui_active_model = node_ui::ActiveModel {
                    id_node_ui: Set(generate_id()),
                    x: Set(Some(200 * element_count)),
                    y: Set(Some(300)),
                    width: Set(Some(300)),
                    height: Set(Some(200)),
                    id_element: Set(Some(id_element)),
                    id_entity_collection: Set(Some(join_po.entity_collection.id_entity_collection.clone())),
                };
                node_ui_active_model_list.push(node_ui_active_model);
                element_count = element_count + 1;
            }
            node_ui::Entity::insert_many(node_ui_active_model_list)
                .exec(&txn)
                .await
                .map_err(|err| {
                    log::error!("join_entities insert entity node_ui failed");
                    TcdtServiceError::build_internal_msg_error("join_entities insert entity node_ui failed", err)
                })?;
        }
        if !join_po.enum_ids.is_empty() {
            // enum
            let mut enum_model_list = DdEnumQuery::find_by_ids(db, join_po.enum_ids.clone()).await
                .map_err(|err| {
                    log::error!("join_entities enum find_by_ids failed");
                    TcdtServiceError::build_internal_msg_error("join_entities enum find_by_ids failed", err)
                })?;
            enum_model_list.iter_mut().for_each(|enum_model| {
                enum_model.id_entity_collection = Some(join_po.entity_collection.id_entity_collection.clone());
            });
            let mut node_ui_active_model_list: Vec<node_ui::ActiveModel> = vec![];
            for enum_model in enum_model_list {
                let id_enum = enum_model.id_enum.clone();
                dd_enum::Entity::update(dd_enum::convert_model_to_active_model(enum_model))
                    .exec(&txn)
                    .await
                    .map_err(|err| {
                        log::error!("join_entities update enum failed");
                        TcdtServiceError::build_internal_msg_error("join_entities update enum failed", err)
                    })?;
                let node_ui_active_model = node_ui::ActiveModel {
                    id_node_ui: Set(generate_id()),
                    x: Set(Some(200 * element_count)),
                    y: Set(Some(300)),
                    width: Set(Some(300)),
                    height: Set(Some(200)),
                    id_element: Set(Some(id_enum)),
                    id_entity_collection: Set(Some(join_po.entity_collection.id_entity_collection.clone())),
                };
                node_ui_active_model_list.push(node_ui_active_model);
                element_count = element_count + 1;
            }
            node_ui::Entity::insert_many(node_ui_active_model_list)
                .exec(&txn)
                .await
                .map_err(|err| {
                    log::error!("join_entities insert enum node_ui failed");
                    TcdtServiceError::build_internal_msg_error("join_entities insert enum node_ui failed", err)
                })?;
        }
        txn.commit().await.map_err(|err| {
            log::error!("tx commit failed");
            TcdtServiceError::build_internal_msg_error("tx commit failed", err)
        })?;
        Ok(())
    }
}

async fn delete_check_by_action<C: ConnectionTrait>(
    db: &C,
    save_po: &SavePO,
) -> Result<Vec<DeleteRefErrorMessageVO>, TcdtServiceError> {
    let mut messge_list: Vec<DeleteRefErrorMessageVO> = vec![];
    if save_po.action == DO_DELETE {
        let entity_count = dd_entity::Entity::find()
            .filter(dd_entity::Column::IdEntityCollection.eq(save_po.id_entity_collection.clone()))
            .count(db)
            .await
            .map_err(|err| {
                log::error!("entity_count failed");
                TcdtServiceError::build_internal_msg_error("entity_count failed", err)
            })?;
        if entity_count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: save_po.id_entity_collection.clone(),
                message: format!("entity collectin exist entity, can not delete", ),
                source_class_name: String::from(""),
                ref_class_name: String::from(""),
            };
            messge_list.push(msg);
        }
        let enum_count = dd_enum::Entity::find()
            .filter(dd_enum::Column::IdEntityCollection.eq(save_po.id_entity_collection.clone()))
            .count(db)
            .await
            .map_err(|err| {
                log::error!("enum_count failed");
                TcdtServiceError::build_internal_msg_error("enum_count failed", err)
            })?;
        if enum_count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: save_po.id_entity_collection.clone(),
                message: format!("entity collectin exist enum, can not delete", ),
                source_class_name: String::from(""),
                ref_class_name: String::from(""),
            };
            messge_list.push(msg);
        }
    }
    let entities = save_po.entities.clone();
    //实体连线
    let entity_associates = save_po.entity_associates.clone();
    for entity_associate in entity_associates {
        if entity_associate.action != DO_DELETE {
            continue;
        }
        let count = component_entity_associate::Entity::find()
            .filter(
                component_entity_associate::Column::IdEntityAssociate
                    .eq(entity_associate.id_entity_associate.clone()),
            )
            .count(db)
            .await
            .map_err(|err| {
                log::error!("component_entity_associate count failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_entity_associate count failed",
                    err,
                )
            })?;
        if count > 0 {
            let id_up = entity_associate
                .id_up
                .ok_or_else(|| TcdtServiceError::build_internal_msg("id_up is None"))?;
            let up_entity = entities
                .iter()
                .filter(|enti| enti.id_entity == id_up)
                .last()
                .ok_or_else(|| {
                    TcdtServiceError::build_internal_msg(&format!("up_entity is None"))
                })?;
            let msg = DeleteRefErrorMessageVO {
                id_data: entity_associate.id_entity_associate.clone(),
                message: format!(
                    "entity: [{}] associate line: [{}] be in use",
                    up_entity.display_name.clone().unwrap_or_default(),
                    entity_associate.id_entity_associate.clone()
                ),
                source_class_name: "entity_associate".to_owned(),
                ref_class_name: "component_entity_associate".to_owned(),
            };
            messge_list.push(msg);
        }
    }

    //枚举
    let dd_enums = save_po.enums.clone();
    for dd_enum_entity in dd_enums {
        if dd_enum_entity.action != DO_DELETE {
            continue;
        }
        let count = component_enum::Entity::find()
            .filter(component_enum::Column::IdEnum.eq(dd_enum_entity.id_enum.clone()))
            .count(db)
            .await
            .map_err(|err| {
                log::error!("dd_enum_entity count failed");
                TcdtServiceError::build_internal_msg_error("dd_enum_entity count failed", err)
            })?;
        if count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: dd_enum_entity.id_enum.clone(),
                message: format!(
                    "enum: [{}] be in use",
                    dd_enum_entity.display_name.clone().unwrap_or_default()
                ),
                source_class_name: "dd_enum".to_owned(),
                ref_class_name: "component_enum".to_owned(),
            };
            messge_list.push(msg);
        }
    }
    //实体
    let entities = save_po.entities.clone();
    for entity_entity in entities {
        let attributes = entity_entity.attributes.clone();
        for attr in attributes {
            if attr.action != DO_DELETE {
                continue;
            }
            let ext_attribute_count = ext_attribute::Entity::find()
                .filter(ext_attribute::Column::IdAttribute.eq(attr.id_attribute.clone()))
                .count(db)
                .await
                .map_err(|err| {
                    log::error!("ext_attribute count failed");
                    TcdtServiceError::build_internal_msg_error("ext_attribute count failed", err)
                })?;
            if ext_attribute_count > 0 {
                let msg = DeleteRefErrorMessageVO {
                    id_data: attr.id_attribute.clone(),
                    message: format!(
                        "entity: [{}] attribute: [{}] be in use",
                        entity_entity.display_name.clone().unwrap_or_default(),
                        attr.display_name.clone().unwrap_or_default()
                    ),
                    source_class_name: "dd_entity_attribute".to_owned(),
                    ref_class_name: "ext_attribute".to_owned(),
                };
                messge_list.push(msg);
            }
        }
        if entity_entity.action != DO_DELETE {
            continue;
        }
        let count = component_entity::Entity::find()
            .filter(component_entity::Column::IdEntity.eq(entity_entity.id_entity.clone()))
            .count(db)
            .await
            .map_err(|err| {
                log::error!("component_entity count failed");
                TcdtServiceError::build_internal_msg_error("component_entity count failed", err)
            })?;
        if count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: entity_entity.id_entity.clone(),
                message: format!(
                    "entity: [{}] be in use",
                    entity_entity.display_name.clone().unwrap_or_default()
                ),
                source_class_name: "dd_entity".to_owned(),
                ref_class_name: "component_entity".to_owned(),
            };
            messge_list.push(msg);
        }
    }
    Ok(messge_list)
}

async fn delete_by_action<C: ConnectionTrait>(
    db: &C,
    save_po: SavePO,
) -> Result<Option<SavePO>, TcdtServiceError> {
    //ui信息
    let delete_node_ui_ids: Vec<String> = save_po
        .node_uis
        .clone()
        .iter()
        .filter(|node_ui| node_ui.action == DO_DELETE)
        .map(|node_ui| node_ui.id_node_ui.to_owned())
        .collect();
    node_ui::Entity::delete_many()
        .filter(node_ui::Column::IdNodeUi.is_in(delete_node_ui_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("node_ui delete_many failed");
            TcdtServiceError::build_internal_msg_error("node_ui delete_many failed", err)
        })?;
    //实体连线
    let delete_entity_associate_ids: Vec<String> = save_po
        .entity_associates
        .clone()
        .iter()
        .filter(|entity_associate| entity_associate.action == DO_DELETE)
        .map(|entity_associate| entity_associate.id_entity_associate.to_owned())
        .collect();
    entity_associate::Entity::delete_many()
        .filter(entity_associate::Column::IdEntityAssociate.is_in(delete_entity_associate_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("entity_associate delete_many failed");
            TcdtServiceError::build_internal_msg_error("entity_associate delete_many failed", err)
        })?;
    //枚举连线
    let delete_enum_associate_ids: Vec<String> = save_po
        .enum_associates
        .clone()
        .iter()
        .filter(|enum_associate| enum_associate.action == DO_DELETE)
        .map(|enum_associate| enum_associate.id_enum_associate.to_owned())
        .collect();
    enum_associate::Entity::delete_many()
        .filter(enum_associate::Column::IdEnumAssociate.is_in(delete_enum_associate_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("enum_associate delete_many failed");
            TcdtServiceError::build_internal_msg_error("enum_associate delete_many failed", err)
        })?;
    //枚举属性
    let delete_enum_attr_ids: Vec<String> = save_po
        .enums
        .clone()
        .iter()
        .flat_map(|enum_po| enum_po.attributes.clone())
        .filter(|enum_attr| enum_attr.action == DO_DELETE)
        .map(|enum_attr| enum_attr.id_enum_attribute.to_owned())
        .collect();
    enum_attribute::Entity::delete_many()
        .filter(enum_attribute::Column::IdEnumAttribute.is_in(delete_enum_attr_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("enum_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error("enum_attribute delete_many failed", err)
        })?;
    //枚举
    let delete_enum_ids: Vec<String> = save_po
        .enums
        .clone()
        .iter()
        .filter(|enum_po| enum_po.action == DO_DELETE)
        .map(|enum_po| enum_po.id_enum.to_owned())
        .collect();
    dd_enum::Entity::delete_many()
        .filter(dd_enum::Column::IdEnum.is_in(delete_enum_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dd_enum delete_many failed");
            TcdtServiceError::build_internal_msg_error("dd_enum delete_many failed", err)
        })?;
    //实体属性
    let delete_entity_attr_ids: Vec<String> = save_po
        .entities
        .clone()
        .iter()
        .flat_map(|entity_po| entity_po.attributes.clone())
        .filter(|entity_attr| entity_attr.action == DO_DELETE)
        .map(|entity_attr| entity_attr.id_attribute.to_owned())
        .collect();
    entity_attribute::Entity::delete_many()
        .filter(entity_attribute::Column::IdAttribute.is_in(delete_entity_attr_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("entity_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error("entity_attribute delete_many failed", err)
        })?;
    //实体
    let delete_entity_ids: Vec<String> = save_po
        .entities
        .clone()
        .iter()
        .filter(|entity_po| entity_po.action == DO_DELETE)
        .map(|entity_po| entity_po.id_entity.to_owned())
        .collect();
    //因为前端数据分开加载，需要查找属性并删除
    entity_attribute::Entity::delete_many()
        .filter(entity_attribute::Column::IdEntity.is_in(delete_entity_ids.clone()))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("entity_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error("entity_attribute delete_many failed", err)
        })?;
    dd_entity::Entity::delete_many()
        .filter(dd_entity::Column::IdEntity.is_in(delete_entity_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dd_entity delete_many failed");
            TcdtServiceError::build_internal_msg_error("dd_entity delete_many failed", err)
        })?;

    let mut result_po = save_po.clone();
    result_po.node_uis = save_po
        .node_uis
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.enum_associates = save_po
        .enum_associates
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.entity_associates = save_po
        .entity_associates
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.enums = save_po
        .enums
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.enums.iter_mut().for_each(|enum_po| {
        enum_po.attributes = enum_po
            .attributes
            .clone()
            .into_iter()
            .filter(|po| po.action != DO_DELETE)
            .collect();
    });
    result_po.entities = save_po
        .entities
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.entities.iter_mut().for_each(|entity_po| {
        entity_po.attributes = entity_po
            .attributes
            .clone()
            .into_iter()
            .filter(|po| po.action != DO_DELETE)
            .collect();
    });
    if save_po.action == DO_DELETE {
        entity_associate::Entity::delete_many()
            .filter(
                entity_associate::Column::IdEntityCollection
                    .eq(save_po.id_entity_collection.clone()),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("entity_associate delete_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "entity_associate delete_many failed",
                    err,
                )
            })?;
        enum_associate::Entity::delete_many()
            .filter(
                enum_associate::Column::IdEntityCollection.eq(save_po.id_entity_collection.clone()),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("enum_associate delete_many failed");
                TcdtServiceError::build_internal_msg_error("enum_associate delete_many failed", err)
            })?;
        node_ui::Entity::delete_many()
            .filter(node_ui::Column::IdEntityCollection.eq(save_po.id_entity_collection.clone()))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("node_ui delete_many failed");
                TcdtServiceError::build_internal_msg_error("node_ui delete_many failed", err)
            })?;
        EntityCollectionEntity::delete_by_id(save_po.id_entity_collection)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection delete_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "EntityCollection delete_many failed",
                    err,
                )
            })?;

        return Ok(None);
    }
    Ok(Some(result_po))
}

async fn insert_or_update_by_action<C: ConnectionTrait>(
    db: &C,
    save_po: &mut SavePO,
) -> Result<EntityCollectionModel, TcdtServiceError> {
    if save_po.action == DO_NEW {
        save_po.id_entity_collection = generate_id();
        let active_entity = make_collection_active_model(save_po);

        let _ = EntityCollectionEntity::insert(active_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection insert failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection insert failed", err)
            })?;
    } else if save_po.action == DO_UPDATE {
        let active_entity = make_collection_active_model(save_po);

        let _ = EntityCollectionEntity::update(active_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection update failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection update failed", err)
            })?;
    }
    make_relation(save_po);

    do_insert(save_po, db).await?;

    do_update(save_po, db).await?;

    let entity_collection_save =
        EntityCollectionEntity::find_by_id(save_po.id_entity_collection.clone())
            .one(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection find_by_id failed");
                TcdtServiceError::build_internal_msg_error(
                    "EntityCollection find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg("Cannot find entity"))?;
    Ok(entity_collection_save)
}

fn make_collection_active_model(save_po: &mut SavePO) -> EntityCollectionActiveModel {
    EntityCollectionActiveModel {
        id_entity_collection: Set(save_po.id_entity_collection.clone()),
        package_name: Set(save_po.package_name.clone()),
        display_name: Set(save_po.display_name.clone()),
        id_sub_project: Set(save_po.id_sub_project.clone()),
    }
}

async fn do_insert<C: ConnectionTrait>(
    save_po: &mut SavePO,
    db: &C,
) -> Result<(), TcdtServiceError> {
    let new_entity_list: Vec<dd_entity::ActiveModel> = save_po
        .entities
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_entity_active_model(po))
        .collect();
    if new_entity_list.len() > 0 {
        dd_entity::Entity::insert_many(new_entity_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dd_entity insert_many failed");
                TcdtServiceError::build_internal_msg_error("dd_entity insert_many failed", err)
            })?;
    }
    let new_enum_list: Vec<dd_enum::ActiveModel> = save_po
        .enums
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enum_active_model(po))
        .collect();
    if new_enum_list.len() > 0 {
        dd_enum::Entity::insert_many(new_enum_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dd_enum insert_many failed");
                TcdtServiceError::build_internal_msg_error("dd_enum insert_many failed", err)
            })?;
    }
    let new_entity_attr_list: Vec<entity_attribute::ActiveModel> = save_po
        .entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<entity_attribute::ActiveModel> = po
                .attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_entity_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_entity_attr_list.len() > 0 {
        entity_attribute::Entity::insert_many(new_entity_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("entity_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "entity_attribute insert_many failed",
                    err,
                )
            })?;
    }
    let new_enum_attr_list: Vec<enum_attribute::ActiveModel> = save_po
        .enums
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<enum_attribute::ActiveModel> = po
                .attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_enum_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_enum_attr_list.len() > 0 {
        enum_attribute::Entity::insert_many(new_enum_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("enum_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error("enum_attribute insert_many failed", err)
            })?;
    }
    let new_entity_associate_list: Vec<entity_associate::ActiveModel> = save_po
        .entity_associates
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enti_asso_active_model(po))
        .collect();
    if new_entity_associate_list.len() > 0 {
        entity_associate::Entity::insert_many(new_entity_associate_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("entity_associate insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "entity_associate insert_many failed",
                    err,
                )
            })?;
    }
    let new_enum_associate_list: Vec<enum_associate::ActiveModel> = save_po
        .enum_associates
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enum_asso_active_model(po))
        .collect();
    if new_enum_associate_list.len() > 0 {
        enum_associate::Entity::insert_many(new_enum_associate_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("enum_associate insert_many failed");
                TcdtServiceError::build_internal_msg_error("enum_associate insert_many failed", err)
            })?;
    }
    let new_node_uis: Vec<node_ui::ActiveModel> = save_po
        .node_uis
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_node_ui_active_model(po))
        .collect();
    Ok(if new_node_uis.len() > 0 {
        node_ui::Entity::insert_many(new_node_uis)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("node_ui insert_many failed");
                TcdtServiceError::build_internal_msg_error("node_ui insert_many failed", err)
            })?;
    })
}

async fn do_update<C: ConnectionTrait>(
    save_po: &mut SavePO,
    db: &C,
) -> Result<(), TcdtServiceError> {
    let update_entity_list: Vec<dd_entity::ActiveModel> = save_po
        .entities
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_entity_active_model(po))
        .collect();
    for update_entity in update_entity_list {
        dd_entity::Entity::update(update_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dd_entity update failed");
                TcdtServiceError::build_internal_msg_error("dd_entity update failed", err)
            })?;
    }
    let update_enum_list: Vec<dd_enum::ActiveModel> = save_po
        .enums
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enum_active_model(po))
        .collect();
    for update_enum in update_enum_list {
        dd_enum::Entity::update(update_enum)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dd_enum update failed");
                TcdtServiceError::build_internal_msg_error("dd_enum update failed", err)
            })?;
    }
    let update_entity_attr_list: Vec<entity_attribute::ActiveModel> = save_po
        .entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<entity_attribute::ActiveModel> = po
                .attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_entity_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_entity_attr in update_entity_attr_list {
        entity_attribute::Entity::update(update_entity_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("entity_attribute update failed");
                TcdtServiceError::build_internal_msg_error("entity_attribute update failed", err)
            })?;
    }
    let update_enum_attr_list: Vec<enum_attribute::ActiveModel> = save_po
        .enums
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<enum_attribute::ActiveModel> = po
                .attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_enum_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_enum_attr in update_enum_attr_list {
        enum_attribute::Entity::update(update_enum_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("enum_attribute update failed");
                TcdtServiceError::build_internal_msg_error("enum_attribute update failed", err)
            })?;
    }
    let update_entity_associate_list: Vec<entity_associate::ActiveModel> = save_po
        .entity_associates
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enti_asso_active_model(po))
        .collect();
    for update_entity_associate in update_entity_associate_list {
        entity_associate::Entity::update(update_entity_associate)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("entity_associate update failed");
                TcdtServiceError::build_internal_msg_error("entity_associate update failed", err)
            })?;
    }
    let update_enum_associate_list: Vec<enum_associate::ActiveModel> = save_po
        .enum_associates
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enum_asso_active_model(po))
        .collect();
    for update_enum_associate in update_enum_associate_list {
        enum_associate::Entity::update(update_enum_associate)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("enum_associate update failed");
                TcdtServiceError::build_internal_msg_error("enum_associate update failed", err)
            })?;
    }
    let new_node_uis: Vec<node_ui::ActiveModel> = save_po
        .node_uis
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_node_ui_active_model(po))
        .collect();
    Ok(for update_node_ui in new_node_uis {
        node_ui::Entity::update(update_node_ui)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("node_ui update failed");
                TcdtServiceError::build_internal_msg_error("node_ui update failed", err)
            })?;
    })
}

fn make_relation(save_po: &mut SavePO) {
    save_po.node_uis.iter_mut().for_each(|po| {
        po.id_entity_collection = Some(save_po.id_entity_collection.clone());
        if po.action == DO_NEW {
            po.id_node_ui = generate_id();
        }
    });

    save_po.entity_associates.iter_mut().for_each(|po| {
        po.id_entity_collection = Some(save_po.id_entity_collection.clone());
        if po.action == DO_NEW {
            po.id_entity_associate = generate_id();
        }
    });

    save_po.enum_associates.iter_mut().for_each(|po| {
        po.id_entity_collection = Some(save_po.id_entity_collection.clone());
        if po.action == DO_NEW {
            po.id_enum_associate = generate_id();
        }
    });

    save_po.enums.iter_mut().for_each(|po| {
        po.id_entity_collection = Some(save_po.id_entity_collection.clone());
        if po.action == DO_NEW {
            let old_id_enum = po.id_enum.clone();
            po.id_enum = generate_id();
            save_po.enum_associates.iter_mut().for_each(|asso| {
                if asso.id_enum == Some(old_id_enum.clone()) {
                    asso.id_enum = Some(po.id_enum.clone());
                }
            });
            save_po.node_uis.iter_mut().for_each(|node_ui| {
                if node_ui.id_element == Some(old_id_enum.clone()) {
                    node_ui.id_element = Some(po.id_enum.clone());
                }
            });
        }
        po.attributes.iter_mut().for_each(|attr| {
            attr.id_enum = Some(po.id_enum.clone());
            if po.action == DO_NEW {
                attr.id_enum_attribute = generate_id();
            }
        });
    });

    save_po.entities.iter_mut().for_each(|po| {
        po.id_entity_collection = Some(save_po.id_entity_collection.clone());
        if po.action == DO_NEW {
            let old_id_entity = po.id_entity.clone();
            po.id_entity = generate_id();
            save_po.enum_associates.iter_mut().for_each(|asso| {
                if asso.id_entity == Some(old_id_entity.clone()) {
                    asso.id_entity = Some(po.id_entity.clone());
                }
            });
            save_po.entity_associates.iter_mut().for_each(|asso| {
                if asso.id_up == Some(old_id_entity.clone()) {
                    asso.id_up = Some(po.id_entity.clone());
                }
                if asso.id_down == Some(old_id_entity.clone()) {
                    asso.id_down = Some(po.id_entity.clone());
                }
            });
            save_po.node_uis.iter_mut().for_each(|node_ui| {
                if node_ui.id_element == Some(old_id_entity.clone()) {
                    node_ui.id_element = Some(po.id_entity.clone());
                }
            });
        }
        po.attributes.iter_mut().for_each(|attr| {
            attr.id_entity = Some(po.id_entity.clone());
            if po.action == DO_NEW {
                let old_id_attribute = attr.id_attribute.clone();
                attr.id_attribute = generate_id();
                save_po.enum_associates.iter_mut().for_each(|asso| {
                    if asso.id_attribute == Some(old_id_attribute.clone()) {
                        asso.id_attribute = Some(attr.id_attribute.clone());
                    }
                });
            }
        });
    });
}

fn make_enum_asso_active_model(
    po: &crate::dto::po::ext::entity_collection::collection_po::EnumAssociatePO,
) -> enum_associate::ActiveModel {
    enum_associate::ActiveModel {
        id_enum_associate: Set(po.id_enum_associate.clone()),
        group_order: Set(po.group_order.clone()),
        id_enum: Set(po.id_enum.clone()),
        id_entity_collection: Set(po.id_entity_collection.clone()),
        id_entity: Set(po.id_entity.clone()),
        id_attribute: Set(po.id_attribute.clone()),
    }
}

fn make_enti_asso_active_model(
    po: &crate::dto::po::ext::entity_collection::collection_po::EntityAssociatePO,
) -> entity_associate::ActiveModel {
    entity_associate::ActiveModel {
        id_entity_associate: Set(po.id_entity_associate.clone()),
        group_order: Set(po.group_order.clone()),
        up_associate_type: Set(po.up_associate_type.clone()),
        down_associate_type: Set(po.down_associate_type.clone()),
        down_attribute_name: Set(po.down_attribute_name.clone()),
        down_attribute_display_name: Set(po.down_attribute_display_name.clone()),
        ref_attribute_name: Set(po.ref_attribute_name.clone()),
        ref_attribute_display_name: Set(po.ref_attribute_display_name.clone()),
        fk_column_name: Set(po.fk_column_name.clone()),
        fk_attribute_name: Set(po.fk_attribute_name.clone()),
        fk_attribute_display_name: Set(po.fk_attribute_display_name.clone()),
        fg_foreign_key: Set(po.fg_foreign_key.clone()),
        down_order_str: Set(po.down_order_str.clone()),
        down_batch_size: Set(po.down_batch_size.clone()),
        id_up: Set(po.id_up.clone()),
        id_entity_collection: Set(po.id_entity_collection.clone()),
        id_down: Set(po.id_down.clone()),
    }
}

fn make_enum_active_model(
    po: &crate::dto::po::ext::entity_collection::collection_po::DdEnumPO,
) -> dd_enum::ActiveModel {
    dd_enum::ActiveModel {
        id_enum: Set(po.id_enum.clone()),
        class_name: Set(po.class_name.clone()),
        display_name: Set(po.display_name.clone()),
        enum_value_type: Set(po.enum_value_type.clone()),
        id_entity_collection: Set(po.id_entity_collection.clone()),
    }
}

fn make_enum_attribute_active_model(
    attr: &crate::dto::po::ext::entity_collection::collection_po::EnumAttributePO,
) -> enum_attribute::ActiveModel {
    enum_attribute::ActiveModel {
        id_enum_attribute: Set(attr.id_enum_attribute.clone()),
        display_name: Set(attr.display_name.clone()),
        code: Set(attr.code.clone()),
        enum_value: Set(attr.enum_value.clone()),
        sn: Set(attr.sn.clone()),
        id_enum: Set(attr.id_enum.clone()),
    }
}

fn make_node_ui_active_model(
    po: &crate::dto::po::ext::entity_collection::collection_po::NodeUiPO,
) -> node_ui::ActiveModel {
    node_ui::ActiveModel {
        id_node_ui: Set(po.id_node_ui.clone()),
        x: Set(po.x),
        y: Set(po.y),
        width: Set(po.width),
        height: Set(po.height),
        id_element: Set(po.id_element.clone()),
        id_entity_collection: Set(po.id_entity_collection.clone()),
    }
}

fn make_entity_active_model(
    po: &crate::dto::po::ext::entity_collection::collection_po::DdEntityPO,
) -> dd_entity::ActiveModel {
    dd_entity::ActiveModel {
        id_entity: Set(po.id_entity.clone()),
        display_name: Set(po.display_name.clone()),
        class_name: Set(po.class_name.clone()),
        table_name: Set(po.table_name.clone()),
        pk_attribute_code: Set(po.pk_attribute_code.clone()),
        pk_attribute_name: Set(po.pk_attribute_name.clone()),
        pk_attribute_type_name: Set(po.pk_attribute_type_name.clone()),
        id_entity_collection: Set(po.id_entity_collection.clone()),
    }
}

fn make_entity_attribute_active_model(
    po: &crate::dto::po::ext::entity_collection::collection_po::EntityAttributePO,
) -> entity_attribute::ActiveModel {
    entity_attribute::ActiveModel {
        id_attribute: Set(po.id_attribute.clone()),
        attribute_name: Set(po.attribute_name.clone()),
        display_name: Set(po.display_name.clone()),
        column_name: Set(po.column_name.clone()),
        fg_primary_key: Set(po.fg_primary_key.clone()),
        fg_mandatory: Set(po.fg_mandatory.clone()),
        default_value: Set(po.default_value.clone()),
        len: Set(po.len.clone()),
        pcs: Set(po.pcs.clone()),
        sn: Set(po.sn.clone()),
        note: Set(po.note.clone()),
        category: Set(po.category.clone()),
        id_attribute_type: Set(po.id_attribute_type.clone()),
        id_entity: Set(po.id_entity.clone()),
    }
}
