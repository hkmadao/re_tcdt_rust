use crate::common::aq_const::{DO_DELETE, DO_NEW, DO_UPDATE};
use crate::common::result::DeleteRefErrorMessageVO;
use crate::{
    common::result::SaveResult,
    dto::po::ext::dto_collection::collection::DtoEntityCollectionPO as SavePO,
};
use ::entity::entity::dto_computation_attribute;
use ::entity::entity::{
    dto_entity, dto_entity_associate, dto_entity_attribute,
    dto_entity_collection::{
        ActiveModel as DtoEntityCollectionActiveModel, Entity as DtoEntityCollectionEntity,
        Model as DtoEntityCollectionModel,
    },
    dto_enum, dto_enum_associate, dto_enum_attribute, dto_node_ui,
};
use sea_orm::*;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use crate::util::id_util::generate_id;

pub struct DtoEntityCollectionExtMutation;

impl DtoEntityCollectionExtMutation {
    pub async fn save(
        db: &DbConn,
        save_po: SavePO,
    ) -> Result<SaveResult<DtoEntityCollectionModel>, TcdtServiceError> {
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
    ) -> Result<SaveResult<DtoEntityCollectionModel>, TcdtServiceError> {
        let txn = db.begin().await.map_err(|err| {
            log::error!("tx begin failed");
            TcdtServiceError::build_internal_msg_error("tx begin failed", err)
        })?;
        let mut save_po = save_po.clone();
        let after_save = insert_or_update_by_action(db, &mut save_po)
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
}
async fn delete_check_by_action<C: ConnectionTrait>(
    db: &C,
    save_po: &SavePO,
) -> Result<Vec<DeleteRefErrorMessageVO>, TcdtServiceError> {
    let mut messge_list: Vec<DeleteRefErrorMessageVO> = vec![];
    if save_po.action == DO_DELETE {
        let entity_count = dto_entity::Entity::find()
            .filter(
                dto_entity::Column::IdDtoEntityCollection
                    .eq(save_po.id_dto_entity_collection.clone()),
            )
            .count(db)
            .await
            .map_err(|err| {
                log::error!("entity_count failed");
                TcdtServiceError::build_internal_msg_error("entity_count failed", err)
            })?;
        if entity_count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: save_po.id_dto_entity_collection.clone(),
                message: format!("dto entity collectin exist dto entity, can not delete",),
                source_class_name: String::from(""),
                ref_class_name: String::from(""),
            };
            messge_list.push(msg);
        }
        let enum_count = dto_enum::Entity::find()
            .filter(
                dto_enum::Column::IdDtoEntityCollection
                    .eq(save_po.id_dto_entity_collection.clone()),
            )
            .count(db)
            .await
            .map_err(|err| {
                log::error!("enum_count failed");
                TcdtServiceError::build_internal_msg_error("enum_count failed", err)
            })?;
        if enum_count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: save_po.id_dto_entity_collection.clone(),
                message: format!("entity collectin exist enum, can not delete",),
                source_class_name: String::from(""),
                ref_class_name: String::from(""),
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
        .dto_node_uis
        .clone()
        .iter()
        .filter(|dto_node_ui| dto_node_ui.action == DO_DELETE)
        .map(|dto_node_ui| dto_node_ui.id_dto_node_ui.to_owned())
        .collect();
    dto_node_ui::Entity::delete_many()
        .filter(dto_node_ui::Column::IdDtoNodeUi.is_in(delete_node_ui_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_node_ui delete_many failed");
            TcdtServiceError::build_internal_msg_error("dto_node_ui delete_many failed", err)
        })?;
    //实体连线
    let delete_entity_associate_ids: Vec<String> = save_po
        .de_associates
        .clone()
        .iter()
        .filter(|dto_entity_associate| dto_entity_associate.action == DO_DELETE)
        .map(|dto_entity_associate| dto_entity_associate.id_dto_entity_associate.to_owned())
        .collect();
    dto_entity_associate::Entity::delete_many()
        .filter(
            dto_entity_associate::Column::IdDtoEntityAssociate.is_in(delete_entity_associate_ids),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_entity_associate delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "dto_entity_associate delete_many failed",
                err,
            )
        })?;
    //枚举连线
    let delete_enum_associate_ids: Vec<String> = save_po
        .dto_enum_associates
        .clone()
        .iter()
        .filter(|dto_enum_associate| dto_enum_associate.action == DO_DELETE)
        .map(|dto_enum_associate| dto_enum_associate.id_dto_enum_associate.to_owned())
        .collect();
    dto_enum_associate::Entity::delete_many()
        .filter(dto_enum_associate::Column::IdDtoEnumAssociate.is_in(delete_enum_associate_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_enum_associate delete_many failed");
            TcdtServiceError::build_internal_msg_error("dto_enum_associate delete_many failed", err)
        })?;
    //枚举属性
    let delete_enum_attr_ids: Vec<String> = save_po
        .dto_enums
        .clone()
        .iter()
        .flat_map(|enum_po| enum_po.dto_enum_attributes.clone())
        .filter(|enum_attr| enum_attr.action == DO_DELETE)
        .map(|enum_attr| enum_attr.id_dto_enum_attribute.to_owned())
        .collect();
    dto_enum_attribute::Entity::delete_many()
        .filter(dto_enum_attribute::Column::IdDtoEnum.is_in(delete_enum_attr_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_enum_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error("dto_enum_attribute delete_many failed", err)
        })?;
    //枚举
    let delete_enum_ids: Vec<String> = save_po
        .dto_enums
        .clone()
        .iter()
        .filter(|enum_po| enum_po.action == DO_DELETE)
        .map(|enum_po| enum_po.id_dto_enum.to_owned())
        .collect();
    dto_enum::Entity::delete_many()
        .filter(dto_enum::Column::IdDtoEnum.is_in(delete_enum_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_enum delete_many failed");
            TcdtServiceError::build_internal_msg_error("dto_enum delete_many failed", err)
        })?;
    //实体属性
    let delete_entity_attr_ids: Vec<String> = save_po
        .dto_entities
        .clone()
        .iter()
        .flat_map(|entity_po| entity_po.de_attributes.clone())
        .filter(|entity_attr| entity_attr.action == DO_DELETE)
        .map(|entity_attr| entity_attr.id_dto_entity_attribute.to_owned())
        .collect();
    dto_entity_attribute::Entity::delete_many()
        .filter(dto_entity_attribute::Column::IdDtoEntityAttribute.is_in(delete_entity_attr_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_entity_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "dto_entity_attribute delete_many failed",
                err,
            )
        })?;
    //实体属性
    let delete_computation_attr_ids: Vec<String> = save_po
        .dto_entities
        .clone()
        .iter()
        .flat_map(|entity_po| entity_po.dc_attributes.clone())
        .filter(|entity_attr| entity_attr.action == DO_DELETE)
        .map(|entity_attr| entity_attr.id_dto_computation_attribute.to_owned())
        .collect();
    dto_computation_attribute::Entity::delete_many()
        .filter(
            dto_computation_attribute::Column::IdDtoComputationAttribute
                .is_in(delete_computation_attr_ids),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_computation_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "dto_computation_attribute delete_many failed",
                err,
            )
        })?;
    //实体
    let delete_entity_ids: Vec<String> = save_po
        .dto_entities
        .clone()
        .iter()
        .filter(|entity_po| entity_po.action == DO_DELETE)
        .map(|entity_po| entity_po.id_dto_entity.to_owned())
        .collect();
    //因为前端数据分开加载，需要查找属性并删除
    dto_entity_attribute::Entity::delete_many()
        .filter(dto_entity_attribute::Column::IdDtoEntity.is_in(delete_entity_ids.clone()))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_entity_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "dto_entity_attribute delete_many failed",
                err,
            )
        })?;
    dto_computation_attribute::Entity::delete_many()
        .filter(dto_computation_attribute::Column::IdDtoEntity.is_in(delete_entity_ids.clone()))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_computation_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "dto_computation_attribute delete_many failed",
                err,
            )
        })?;
    dto_entity::Entity::delete_many()
        .filter(dto_entity::Column::IdDtoEntity.is_in(delete_entity_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("dto_entity delete_many failed");
            TcdtServiceError::build_internal_msg_error("dto_entity delete_many failed", err)
        })?;

    let mut result_po = save_po.clone();
    result_po.dto_node_uis = save_po
        .dto_node_uis
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.dto_enum_associates = save_po
        .dto_enum_associates
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.de_associates = save_po
        .de_associates
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.dto_enums = save_po
        .dto_enums
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.dto_enums.iter_mut().for_each(|enum_po| {
        enum_po.dto_enum_attributes = enum_po
            .dto_enum_attributes
            .clone()
            .into_iter()
            .filter(|po| po.action != DO_DELETE)
            .collect();
    });
    result_po.dto_entities = save_po
        .dto_entities
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.dto_entities.iter_mut().for_each(|entity_po| {
        entity_po.de_attributes = entity_po
            .de_attributes
            .clone()
            .into_iter()
            .filter(|po| po.action != DO_DELETE)
            .collect();
    });
    if save_po.action == DO_DELETE {
        dto_entity_associate::Entity::delete_many()
            .filter(
                dto_entity_associate::Column::IdDtoEntityCollection
                    .eq(save_po.id_dto_entity_collection.clone()),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entity_associate delete_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_entity_associate delete_many failed",
                    err,
                )
            })?;
        dto_enum_associate::Entity::delete_many()
            .filter(
                dto_enum_associate::Column::IdDtoEntityCollection
                    .eq(save_po.id_dto_entity_collection.clone()),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum_associate delete_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_enum_associate delete_many failed",
                    err,
                )
            })?;
        dto_node_ui::Entity::delete_many()
            .filter(
                dto_node_ui::Column::IdDtoEntityCollection
                    .eq(save_po.id_dto_entity_collection.clone()),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_node_ui delete_many failed");
                TcdtServiceError::build_internal_msg_error("dto_node_ui delete_many failed", err)
            })?;
        DtoEntityCollectionEntity::delete_by_id(save_po.id_dto_entity_collection)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection delete_by_id failed");
                TcdtServiceError::build_internal_msg_error(
                    "DtoEntityCollection delete_by_id failed",
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
) -> Result<DtoEntityCollectionModel, TcdtServiceError> {
    if save_po.action == DO_NEW {
        save_po.id_dto_entity_collection = generate_id();
    }
    make_relation(save_po);

    if save_po.action == DO_NEW {
        let active_entity = make_collection_active_model(save_po);

        let _ = DtoEntityCollectionEntity::insert(active_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection insert failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection insert failed", err)
            })?;
    } else if save_po.action == DO_UPDATE {
        let active_entity = make_collection_active_model(save_po);

        let _ = DtoEntityCollectionEntity::update(active_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection update failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection update failed", err)
            })?;
    }

    do_insert(save_po, db).await?;

    do_update(save_po, db).await?;

    let entity_collection_save =
        DtoEntityCollectionEntity::find_by_id(save_po.id_dto_entity_collection.clone())
            .one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection find_by_id failed");
                TcdtServiceError::build_internal_msg_error(
                    "DtoEntityCollection find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg("Cannot find entity"))?;
    Ok(entity_collection_save)
}

async fn do_insert<C: ConnectionTrait>(
    save_po: &mut SavePO,
    db: &C,
) -> Result<(), TcdtServiceError> {
    let new_entity_list: Vec<dto_entity::ActiveModel> = save_po
        .dto_entities
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_entity_active_model(po))
        .collect();
    if new_entity_list.len() > 0 {
        dto_entity::Entity::insert_many(new_entity_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entities insert_many failed");
                TcdtServiceError::build_internal_msg_error("dto_entities insert_many failed", err)
            })?;
    }
    let new_enum_list: Vec<dto_enum::ActiveModel> = save_po
        .dto_enums
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enum_active_model(po))
        .collect();
    if new_enum_list.len() > 0 {
        dto_enum::Entity::insert_many(new_enum_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum insert_many failed");
                TcdtServiceError::build_internal_msg_error("dto_enum insert_many failed", err)
            })?;
    }
    let new_entity_attr_list: Vec<dto_entity_attribute::ActiveModel> = save_po
        .dto_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<dto_entity_attribute::ActiveModel> = po
                .de_attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_entity_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_entity_attr_list.len() > 0 {
        dto_entity_attribute::Entity::insert_many(new_entity_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entity_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_entity_attribute insert_many failed",
                    err,
                )
            })?;
    }
    let new_computation_attr_list: Vec<dto_computation_attribute::ActiveModel> = save_po
        .dto_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<dto_computation_attribute::ActiveModel> = po
                .dc_attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_computation_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_computation_attr_list.len() > 0 {
        dto_computation_attribute::Entity::insert_many(new_computation_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_computation_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_computation_attribute insert_many failed",
                    err,
                )
            })?;
    }
    let new_enum_attr_list: Vec<dto_enum_attribute::ActiveModel> = save_po
        .dto_enums
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<dto_enum_attribute::ActiveModel> = po
                .dto_enum_attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_enum_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_enum_attr_list.len() > 0 {
        dto_enum_attribute::Entity::insert_many(new_enum_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_enum_attribute insert_many failed",
                    err,
                )
            })?;
    }
    let new_entity_associate_list: Vec<dto_entity_associate::ActiveModel> = save_po
        .de_associates
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enti_asso_active_model(po))
        .collect();
    if new_entity_associate_list.len() > 0 {
        dto_entity_associate::Entity::insert_many(new_entity_associate_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entity_associate insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_entity_associate insert_many failed",
                    err,
                )
            })?;
    }
    let new_enum_associate_list: Vec<dto_enum_associate::ActiveModel> = save_po
        .dto_enum_associates
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enum_asso_active_model(po))
        .collect();
    if new_enum_associate_list.len() > 0 {
        dto_enum_associate::Entity::insert_many(new_enum_associate_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum_associate insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_enum_associate insert_many failed",
                    err,
                )
            })?;
    }
    let new_node_uis: Vec<dto_node_ui::ActiveModel> = save_po
        .dto_node_uis
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_node_ui_active_model(po))
        .collect();
    Ok(if new_node_uis.len() > 0 {
        dto_node_ui::Entity::insert_many(new_node_uis)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_node_ui insert_many failed");
                TcdtServiceError::build_internal_msg_error("dto_node_ui insert_many failed", err)
            })?;
    })
}

async fn do_update<C: ConnectionTrait>(
    save_po: &mut SavePO,
    db: &C,
) -> Result<(), TcdtServiceError> {
    let update_entity_list: Vec<dto_entity::ActiveModel> = save_po
        .dto_entities
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_entity_active_model(po))
        .collect();
    for update_entity in update_entity_list {
        dto_entity::Entity::update(update_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entity update failed");
                TcdtServiceError::build_internal_msg_error("dto_entity update failed", err)
            })?;
    }
    let update_enum_list: Vec<dto_enum::ActiveModel> = save_po
        .dto_enums
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enum_active_model(po))
        .collect();
    for update_enum in update_enum_list {
        dto_enum::Entity::update(update_enum)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum update failed");
                TcdtServiceError::build_internal_msg_error("dto_enum update failed", err)
            })?;
    }
    let update_entity_attr_list: Vec<dto_entity_attribute::ActiveModel> = save_po
        .dto_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<dto_entity_attribute::ActiveModel> = po
                .de_attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_entity_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_entity_attr in update_entity_attr_list {
        dto_entity_attribute::Entity::update(update_entity_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entity_attribute update failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_entity_attribute update failed",
                    err,
                )
            })?;
    }
    let update_computation_attr_list: Vec<dto_computation_attribute::ActiveModel> = save_po
        .dto_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<dto_computation_attribute::ActiveModel> = po
                .dc_attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_computation_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_computation_attr in update_computation_attr_list {
        dto_computation_attribute::Entity::update(update_computation_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_computation_attribute update failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_computation_attribute update failed",
                    err,
                )
            })?;
    }
    let update_enum_attr_list: Vec<dto_enum_attribute::ActiveModel> = save_po
        .dto_enums
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<dto_enum_attribute::ActiveModel> = po
                .dto_enum_attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_enum_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_enum_attr in update_enum_attr_list {
        dto_enum_attribute::Entity::update(update_enum_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum_attribute update failed");
                TcdtServiceError::build_internal_msg_error("dto_enum_attribute update failed", err)
            })?;
    }
    let update_entity_associate_list: Vec<dto_entity_associate::ActiveModel> = save_po
        .de_associates
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enti_asso_active_model(po))
        .collect();
    for update_entity_associate in update_entity_associate_list {
        dto_entity_associate::Entity::update(update_entity_associate)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_entity_associate update failed");
                TcdtServiceError::build_internal_msg_error(
                    "dto_entity_associate update failed",
                    err,
                )
            })?;
    }
    let update_enum_associate_list: Vec<dto_enum_associate::ActiveModel> = save_po
        .dto_enum_associates
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enum_asso_active_model(po))
        .collect();
    for update_enum_associate in update_enum_associate_list {
        dto_enum_associate::Entity::update(update_enum_associate)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_enum_associate update failed");
                TcdtServiceError::build_internal_msg_error("dto_enum_associate update failed", err)
            })?;
    }
    let new_node_uis: Vec<dto_node_ui::ActiveModel> = save_po
        .dto_node_uis
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_node_ui_active_model(po))
        .collect();
    Ok(for update_node_ui in new_node_uis {
        dto_node_ui::Entity::update(update_node_ui)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("dto_node_ui update failed");
                TcdtServiceError::build_internal_msg_error("dto_node_ui update failed", err)
            })?;
    })
}

fn make_relation(save_po: &mut SavePO) {
    save_po.dto_node_uis.iter_mut().for_each(|po| {
        po.id_dto_entity_collection = Some(save_po.id_dto_entity_collection.clone());
        if po.action == DO_NEW {
            po.id_dto_node_ui = generate_id();
        }
    });

    save_po.de_associates.iter_mut().for_each(|po| {
        po.id_dto_entity_collection = Some(save_po.id_dto_entity_collection.clone());
        if po.action == DO_NEW {
            po.id_dto_entity_associate = generate_id();
        }
    });

    save_po.dto_enum_associates.iter_mut().for_each(|po| {
        po.id_dto_entity_collection = Some(save_po.id_dto_entity_collection.clone());
        if po.action == DO_NEW {
            po.id_dto_enum_associate = generate_id();
        }
    });

    save_po.dto_enums.iter_mut().for_each(|po| {
        po.id_dto_entity_collection = Some(save_po.id_dto_entity_collection.clone());
        if po.action == DO_NEW {
            let old_id_enum = po.id_dto_enum.clone();
            po.id_dto_enum = generate_id();
            save_po.dto_enum_associates.iter_mut().for_each(|asso| {
                if asso.id_dto_enum == Some(old_id_enum.clone()) {
                    asso.id_dto_enum = Some(po.id_dto_enum.clone());
                }
            });
            save_po.dto_node_uis.iter_mut().for_each(|dto_node_ui| {
                if dto_node_ui.id_element == Some(old_id_enum.clone()) {
                    dto_node_ui.id_element = Some(po.id_dto_enum.clone());
                }
            });
        }
        po.dto_enum_attributes.iter_mut().for_each(|attr| {
            attr.id_dto_enum = Some(po.id_dto_enum.clone());
            if po.action == DO_NEW {
                attr.id_dto_enum_attribute = generate_id();
            }
        });
    });

    save_po.dto_entities.iter_mut().for_each(|po| {
        po.id_dto_entity_collection = Some(save_po.id_dto_entity_collection.clone());
        if po.action == DO_NEW {
            let old_id_entity = po.id_dto_entity.clone();
            po.id_dto_entity = generate_id();
            if save_po.id_main_dto_entity == Some(old_id_entity.clone()) {
                save_po.id_main_dto_entity = Some(po.id_dto_entity.clone());
            }
            save_po.dto_enum_associates.iter_mut().for_each(|asso| {
                if asso.id_dto_entity == Some(old_id_entity.clone()) {
                    asso.id_dto_entity = Some(po.id_dto_entity.clone());
                }
            });
            save_po.de_associates.iter_mut().for_each(|asso| {
                if asso.id_up == Some(old_id_entity.clone()) {
                    asso.id_up = Some(po.id_dto_entity.clone());
                }
                if asso.id_down == Some(old_id_entity.clone()) {
                    asso.id_down = Some(po.id_dto_entity.clone());
                }
            });
            save_po.dto_node_uis.iter_mut().for_each(|dto_node_ui| {
                if dto_node_ui.id_element == Some(old_id_entity.clone()) {
                    dto_node_ui.id_element = Some(po.id_dto_entity.clone());
                }
            });
        }
        po.de_attributes.iter_mut().for_each(|attr| {
            attr.id_dto_entity = Some(po.id_dto_entity.clone());
            if po.action == DO_NEW {
                let old_id_attribute = attr.id_dto_entity_attribute.clone();
                attr.id_dto_entity_attribute = generate_id();
                save_po.dto_enum_associates.iter_mut().for_each(|asso| {
                    if asso.id_dto_entity_attribute == Some(old_id_attribute.clone()) {
                        asso.id_dto_entity_attribute = Some(attr.id_dto_entity_attribute.clone());
                    }
                });
            }
        });
        po.dc_attributes.iter_mut().for_each(|attr| {
            attr.id_dto_entity = Some(po.id_dto_entity.clone());
            if po.action == DO_NEW {
                let _old_id_attribute = attr.id_dto_computation_attribute.clone();
                attr.id_dto_computation_attribute = generate_id();
            }
        });
    });
}

fn make_collection_active_model(save_po: &mut SavePO) -> DtoEntityCollectionActiveModel {
    DtoEntityCollectionActiveModel {
        id_dto_entity_collection: Set(save_po.id_dto_entity_collection.clone()),
        package_name: Set(save_po.package_name.clone()),
        display_name: Set(save_po.display_name.clone()),
        id_dto_module: Set(save_po.id_dto_module.clone()),
        id_main_dto_entity: Set(save_po.id_main_dto_entity.clone()),
    }
}

fn make_enum_asso_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoEnumAssociatePO,
) -> dto_enum_associate::ActiveModel {
    dto_enum_associate::ActiveModel {
        id_dto_enum_associate: Set(po.id_dto_enum_associate.clone()),
        group_order: Set(po.group_order.clone()),
        id_dto_enum: Set(po.id_dto_enum.clone()),
        id_dto_entity_collection: Set(po.id_dto_entity_collection.clone()),
        id_dto_entity: Set(po.id_dto_entity.clone()),
        id_dto_entity_attribute: Set(po.id_dto_entity_attribute.clone()),
    }
}

fn make_enti_asso_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoEntityAssociatePO,
) -> dto_entity_associate::ActiveModel {
    dto_entity_associate::ActiveModel {
        id_dto_entity_associate: Set(po.id_dto_entity_associate.clone()),
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
        id_up: Set(po.id_up.clone()),
        id_dto_entity_collection: Set(po.id_dto_entity_collection.clone()),
        id_down: Set(po.id_down.clone()),
        fg_sys_ref: Set(po.fg_sys_ref.clone()),
    }
}

fn make_enum_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoEnumPO,
) -> dto_enum::ActiveModel {
    dto_enum::ActiveModel {
        id_dto_enum: Set(po.id_dto_enum.clone()),
        class_name: Set(po.class_name.clone()),
        display_name: Set(po.display_name.clone()),
        enum_value_type: Set(po.enum_value_type.clone()),
        id_dto_entity_collection: Set(po.id_dto_entity_collection.clone()),
        id_ref: Set(po.id_ref.clone()),
    }
}

fn make_enum_attribute_active_model(
    attr: &crate::dto::po::ext::dto_collection::collection::DtoEnumAttributePO,
) -> dto_enum_attribute::ActiveModel {
    dto_enum_attribute::ActiveModel {
        id_dto_enum_attribute: Set(attr.id_dto_enum_attribute.clone()),
        display_name: Set(attr.display_name.clone()),
        code: Set(attr.code.clone()),
        enum_value: Set(attr.enum_value.clone()),
        sn: Set(attr.sn.clone()),
        id_dto_enum: Set(attr.id_dto_enum.clone()),
        id_ref: Set(attr.id_ref.clone()),
    }
}

fn make_node_ui_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoNodeUiPO,
) -> dto_node_ui::ActiveModel {
    dto_node_ui::ActiveModel {
        id_dto_node_ui: Set(po.id_dto_node_ui.clone()),
        x: Set(po.x),
        y: Set(po.y),
        width: Set(po.width),
        height: Set(po.height),
        id_element: Set(po.id_element.clone()),
        id_dto_entity_collection: Set(po.id_dto_entity_collection.clone()),
    }
}

fn make_entity_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoEntityPO,
) -> dto_entity::ActiveModel {
    dto_entity::ActiveModel {
        id_dto_entity: Set(po.id_dto_entity.clone()),
        display_name: Set(po.display_name.clone()),
        class_name: Set(po.class_name.clone()),
        table_name: Set(po.table_name.clone()),
        pk_attribute_code: Set(po.pk_attribute_code.clone()),
        pk_attribute_name: Set(po.pk_attribute_name.clone()),
        pk_attribute_type_name: Set(po.pk_attribute_type_name.clone()),
        id_dto_entity_collection: Set(po.id_dto_entity_collection.clone()),
        id_ref: Set(po.id_ref.clone()),
    }
}

fn make_computation_attribute_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoComputationAttributePO,
) -> dto_computation_attribute::ActiveModel {
    dto_computation_attribute::ActiveModel {
        id_dto_computation_attribute: Set(po.id_dto_computation_attribute.clone()),
        attribute_name: Set(po.attribute_name.clone()),
        display_name: Set(po.display_name.clone()),
        note: Set(po.note.clone()),
        len: Set(po.len.clone()),
        fg_mandatory: Set(po.fg_mandatory.clone()),
        default_value: Set(po.default_value.clone()),
        pcs: Set(po.pcs.clone()),
        sn: Set(po.sn.clone()),
        id_attribute_type: Set(po.id_attribute_type.clone()),
        id_dto_entity: Set(po.id_dto_entity.clone()),
    }
}

fn make_entity_attribute_active_model(
    po: &crate::dto::po::ext::dto_collection::collection::DtoEntityAttributePO,
) -> dto_entity_attribute::ActiveModel {
    dto_entity_attribute::ActiveModel {
        id_dto_entity_attribute: Set(po.id_dto_entity_attribute.clone()),
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
        id_dto_entity: Set(po.id_dto_entity.clone()),
        id_ref_attribute: Set(po.id_ref_attribute.clone()),
    }
}
