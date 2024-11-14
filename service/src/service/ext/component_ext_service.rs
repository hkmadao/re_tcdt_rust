use crate::common::aq_const::DO_DELETE;
use crate::common::result::{DeleteRefErrorMessageVO, SaveResult};
use crate::dto::vo::ext::component::description_info::DescriptionInfo;
use crate::{
    common::aq_const::{DO_NEW, DO_UPDATE},
    dto::po::ext::component::collection::ComponentPO as SavePO,
    service::{
        base::{component_service::ComponentQuery, entity_associate_service::EntityAssociateQuery},
        ext::component::description_util::DescriptionUtil,
    },
};
use ::entity::entity::{
    component,
    component::{
        ActiveModel as ComponentActiveModel, Entity as ComponentEntity, Model as ComponentModel,
    },
    component_entity, component_entity_associate, component_enum, component_node_ui,
    computation_attribute, ext_attribute,
};
use sea_orm::*;
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub struct ComponentExtMutation;

impl ComponentExtMutation {
    pub async fn get_description(
        db: &DbConn,
        id: String,
    ) -> Result<DescriptionInfo, TcdtServiceError> {
        let comp_entity = ComponentQuery::find_by_id(db, id).await?;
        let component_entity_list = comp_entity
            .find_linked(component::ComponentEntitiesLinked)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find component_entity failed");
                TcdtServiceError::build_internal_msg_error("find component_entity failed", err)
            })?;

        let main_component_entity = component_entity_list
            .iter()
            .find(|component_entity_entity| {
                Some(component_entity_entity.id_component_entity.clone())
                    == comp_entity.id_main_component_entity
            })
            .ok_or(TcdtServiceError::build_internal_msg("main entity not set"))?;
        let description_util =
            DescriptionUtil::load_data(db, main_component_entity.id_component_entity.clone())
                .await?;
        let mut main_description = description_util.build_description_info(None).await?;
        main_description.fg_partner = Some(true);
        let component_entity_associate_list = comp_entity
            .find_linked(component::ComponentEntityAssociatesLinked)
            .filter(component_entity_associate::Column::FgAggAsso.eq(true))
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find component_entity_associate failed");
                TcdtServiceError::build_internal_msg_error(
                    "find component_entity_associate failed",
                    err,
                )
            })?;
        if component_entity_associate_list.len() > 0 {
            let associate_id_list = component_entity_associate_list
                .iter()
                .map(|cp_associate| cp_associate.id_entity_associate.clone().unwrap())
                .collect::<Vec<_>>();
            let entity_associate_list =
                EntityAssociateQuery::find_by_ids(db, associate_id_list).await?;
            for cp_associate in component_entity_associate_list {
                let entity_associate_entity = entity_associate_list
                    .iter()
                    .find(|associate| {
                        Some(associate.id_entity_associate.clone())
                            == cp_associate.id_entity_associate
                    })
                    .unwrap();
                let child_component_entity = component_entity_list
                    .iter()
                    .find(|component_entity_entity| {
                        entity_associate_entity.id_down == component_entity_entity.id_entity
                    })
                    .unwrap();
                let description_util = DescriptionUtil::load_data(
                    db,
                    child_component_entity.id_component_entity.clone(),
                )
                .await?;
                let child_description = description_util
                    .build_description_info(entity_associate_entity.down_attribute_name.clone())
                    .await?;
                main_description.children = main_description
                    .children
                    .iter()
                    .map(|desc| {
                        let mut desc = desc.clone();
                        if desc.attribute_name == entity_associate_entity.down_attribute_name {
                            desc.fg_partner = Some(true);
                            desc.children = child_description.children.clone();
                            desc.entity_info = child_description.entity_info.clone();
                        }
                        desc.clone()
                    })
                    .collect::<Vec<_>>();
            }
        }

        Ok(main_description)
    }
    pub async fn save(
        db: &DbConn,
        save_po: SavePO,
    ) -> Result<SaveResult<ComponentModel>, TcdtServiceError> {
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
}

async fn delete_check_by_action<C: ConnectionTrait>(
    db: &C,
    save_po: &SavePO,
) -> Result<Vec<DeleteRefErrorMessageVO>, TcdtServiceError> {
    let mut messge_list: Vec<DeleteRefErrorMessageVO> = vec![];
    if save_po.action == DO_DELETE {
        let entity_count = component_entity::Entity::find()
            .filter(component_entity::Column::IdComponent.eq(save_po.id_component.clone()))
            .count(db)
            .await
            .map_err(|err| {
                log::error!("component_entity count failed");
                TcdtServiceError::build_internal_msg_error("component_entity count failed", err)
            })?;
        if entity_count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: save_po.id_component.clone(),
                message: format!("component exist entity, can not delete",),
                source_class_name: String::from(""),
                ref_class_name: String::from(""),
            };
            messge_list.push(msg);
        }
        let enum_count = component_enum::Entity::find()
            .filter(component_enum::Column::IdComponent.eq(save_po.id_component.clone()))
            .count(db)
            .await
            .map_err(|err| {
                log::error!("enum_count failed");
                TcdtServiceError::build_internal_msg_error("enum_count failed", err)
            })?;
        if enum_count > 0 {
            let msg = DeleteRefErrorMessageVO {
                id_data: save_po.id_component.clone(),
                message: format!("component exist enum, can not delete",),
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
        .component_node_uis
        .clone()
        .iter()
        .filter(|component_node_ui| component_node_ui.action == DO_DELETE)
        .map(|component_node_ui| component_node_ui.id_component_node_ui.to_owned())
        .collect();
    component_node_ui::Entity::delete_many()
        .filter(component_node_ui::Column::IdComponentNodeUi.is_in(delete_node_ui_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("component_node_ui delete_many failed");
            TcdtServiceError::build_internal_msg_error("component_node_ui delete_many failed", err)
        })?;
    //实体连线
    let delete_entity_associate_ids: Vec<String> = save_po
        .component_entity_associates
        .clone()
        .iter()
        .filter(|component_entity_associate| component_entity_associate.action == DO_DELETE)
        .map(|component_entity_associate| {
            component_entity_associate
                .id_component_entity_associate
                .to_owned()
        })
        .collect();
    component_entity_associate::Entity::delete_many()
        .filter(
            component_entity_associate::Column::IdComponentEntityAssociate
                .is_in(delete_entity_associate_ids),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("component_entity_associate delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "component_entity_associate delete_many failed",
                err,
            )
        })?;
    //枚举
    let delete_enum_ids: Vec<String> = save_po
        .component_enums
        .clone()
        .iter()
        .filter(|enum_po| enum_po.action == DO_DELETE)
        .map(|enum_po| enum_po.id_component_enum.to_owned())
        .collect();
    component_enum::Entity::delete_many()
        .filter(component_enum::Column::IdComponentEnum.is_in(delete_enum_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("component_enum delete_many failed");
            TcdtServiceError::build_internal_msg_error("component_enum delete_many failed", err)
        })?;
    //实体属性
    let delete_entity_attr_ids: Vec<String> = save_po
        .component_entities
        .clone()
        .iter()
        .flat_map(|entity_po| entity_po.ext_attributes.clone())
        .filter(|entity_attr| entity_attr.action == DO_DELETE)
        .map(|entity_attr| entity_attr.id_ext_attribute.to_owned())
        .collect();
    ext_attribute::Entity::delete_many()
        .filter(ext_attribute::Column::IdExtAttribute.is_in(delete_entity_attr_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("ext_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error("ext_attribute delete_many failed", err)
        })?;
    //实体计算属性
    let delete_computation_attr_ids: Vec<String> = save_po
        .component_entities
        .clone()
        .iter()
        .flat_map(|entity_po| entity_po.computation_attributes.clone())
        .filter(|entity_attr| entity_attr.action == DO_DELETE)
        .map(|entity_attr| entity_attr.id_computation_attribute.to_owned())
        .collect();
    computation_attribute::Entity::delete_many()
        .filter(
            computation_attribute::Column::IdComputationAttribute
                .is_in(delete_computation_attr_ids),
        )
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("computation_attribute delete_many failed");
            TcdtServiceError::build_internal_msg_error(
                "computation_attribute delete_many failed",
                err,
            )
        })?;
    //实体
    let delete_entity_ids: Vec<String> = save_po
        .component_entities
        .clone()
        .iter()
        .filter(|entity_po| entity_po.action == DO_DELETE)
        .map(|entity_po| entity_po.id_component_entity.to_owned())
        .collect();
    component_entity::Entity::delete_many()
        .filter(component_entity::Column::IdComponentEntity.is_in(delete_entity_ids))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("component_entity delete_many failed");
            TcdtServiceError::build_internal_msg_error("component_entity delete_many failed", err)
        })?;

    let mut result_po = save_po.clone();
    result_po.component_node_uis = save_po
        .component_node_uis
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.component_entity_associates = save_po
        .component_entity_associates
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.component_enums = save_po
        .component_enums
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po.component_entities = save_po
        .component_entities
        .into_iter()
        .filter(|po| po.action != DO_DELETE)
        .collect();
    result_po
        .component_entities
        .iter_mut()
        .for_each(|entity_po| {
            entity_po.ext_attributes = entity_po
                .ext_attributes
                .clone()
                .into_iter()
                .filter(|po| po.action != DO_DELETE)
                .collect();
        });
    if save_po.action == DO_DELETE {
        component_entity_associate::Entity::delete_many()
            .filter(
                component_entity_associate::Column::IdComponent.eq(save_po.id_component.clone()),
            )
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_entity_associate delete_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_entity_associate delete_many failed",
                    err,
                )
            })?;
        component_node_ui::Entity::delete_many()
            .filter(component_node_ui::Column::IdComponent.eq(save_po.id_component.clone()))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_node_ui delete_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_node_ui delete_many failed",
                    err,
                )
            })?;
        ComponentEntity::delete_by_id(save_po.id_component)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Component delete_by_id failed");
                TcdtServiceError::build_internal_msg_error("Component delete_by_id failed", err)
            })?;

        return Ok(None);
    }
    Ok(Some(result_po))
}

async fn insert_or_update_by_action<C: ConnectionTrait>(
    db: &C,
    save_po: &mut SavePO,
) -> Result<ComponentModel, TcdtServiceError> {
    if save_po.action == DO_NEW {
        save_po.id_component = nanoid::nanoid!();
    }

    make_relation(save_po);

    if save_po.action == DO_NEW {
        let active_entity = make_component_active_model(save_po);

        let _ = ComponentEntity::insert(active_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Component insert failed");
                TcdtServiceError::build_internal_msg_error("Component insert failed", err)
            })?;
    } else if save_po.action == DO_UPDATE {
        let active_entity = make_component_active_model(save_po);

        let _ = ComponentEntity::update(active_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Component update failed");
                TcdtServiceError::build_internal_msg_error("Component update failed", err)
            })?;
    }

    do_insert(save_po, db).await?;

    do_update(save_po, db).await?;

    // update id_main_component_entity attribute
    let active_entity = make_component_active_model(save_po);
    let _ = ComponentEntity::update(active_entity)
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("Component insert failed");
            TcdtServiceError::build_internal_msg_error("Component insert failed", err)
        })?;

    let entity_collection_save = ComponentEntity::find_by_id(save_po.id_component.clone())
        .one(db)
        .await
        .map_err(|err| {
            log::error!("Component find_by_id failed");
            TcdtServiceError::build_internal_msg_error("Component find_by_id failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg("Cannot find entity"))?;
    Ok(entity_collection_save)
}

async fn do_insert<C: ConnectionTrait>(
    save_po: &mut SavePO,
    db: &C,
) -> Result<(), TcdtServiceError> {
    let new_entity_list: Vec<component_entity::ActiveModel> = save_po
        .component_entities
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_entity_active_model(po))
        .collect();
    if new_entity_list.len() > 0 {
        component_entity::Entity::insert_many(new_entity_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_entity insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_entity insert_many failed",
                    err,
                )
            })?;
    }
    let new_enum_list: Vec<component_enum::ActiveModel> = save_po
        .component_enums
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enum_active_model(po))
        .collect();
    if new_enum_list.len() > 0 {
        component_enum::Entity::insert_many(new_enum_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_enum insert_many failed");
                TcdtServiceError::build_internal_msg_error("component_enum insert_many failed", err)
            })?;
    }
    let new_entity_attr_list: Vec<ext_attribute::ActiveModel> = save_po
        .component_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<ext_attribute::ActiveModel> = po
                .ext_attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_entity_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_entity_attr_list.len() > 0 {
        ext_attribute::Entity::insert_many(new_entity_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ext_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error("ext_attribute insert_many failed", err)
            })?;
    }
    let new_computation_attr_list: Vec<computation_attribute::ActiveModel> = save_po
        .component_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<computation_attribute::ActiveModel> = po
                .computation_attributes
                .iter()
                .filter(|attr| attr.action == DO_NEW)
                .map(|attr| make_computation_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    if new_computation_attr_list.len() > 0 {
        computation_attribute::Entity::insert_many(new_computation_attr_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("computation_attribute insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "computation_attribute insert_many failed",
                    err,
                )
            })?;
    }
    let new_entity_associate_list: Vec<component_entity_associate::ActiveModel> = save_po
        .component_entity_associates
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_enti_asso_active_model(po))
        .collect();
    if new_entity_associate_list.len() > 0 {
        component_entity_associate::Entity::insert_many(new_entity_associate_list)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_entity_associate insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_entity_associate insert_many failed",
                    err,
                )
            })?;
    }
    let new_node_uis: Vec<component_node_ui::ActiveModel> = save_po
        .component_node_uis
        .iter()
        .filter(|po| po.action == DO_NEW)
        .map(|po| make_node_ui_active_model(po))
        .collect();
    Ok(if new_node_uis.len() > 0 {
        component_node_ui::Entity::insert_many(new_node_uis)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_node_ui insert_many failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_node_ui insert_many failed",
                    err,
                )
            })?;
    })
}

async fn do_update<C: ConnectionTrait>(
    save_po: &mut SavePO,
    db: &C,
) -> Result<(), TcdtServiceError> {
    let update_entity_list: Vec<component_entity::ActiveModel> = save_po
        .component_entities
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_entity_active_model(po))
        .collect();
    for update_entity in update_entity_list {
        component_entity::Entity::update(update_entity)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_entity update failed");
                TcdtServiceError::build_internal_msg_error("component_entity update failed", err)
            })?;
    }
    let update_enum_list: Vec<component_enum::ActiveModel> = save_po
        .component_enums
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enum_active_model(po))
        .collect();
    for update_enum in update_enum_list {
        component_enum::Entity::update(update_enum)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_enum update failed");
                TcdtServiceError::build_internal_msg_error("component_enum update failed", err)
            })?;
    }
    let update_entity_attr_list: Vec<ext_attribute::ActiveModel> = save_po
        .component_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<ext_attribute::ActiveModel> = po
                .ext_attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_entity_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_entity_attr in update_entity_attr_list {
        ext_attribute::Entity::update(update_entity_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ext_attribute update failed");
                TcdtServiceError::build_internal_msg_error("ext_attribute update failed", err)
            })?;
    }
    let update_computation_attr_list: Vec<computation_attribute::ActiveModel> = save_po
        .component_entities
        .iter()
        .flat_map(|po| {
            let attr_list: Vec<computation_attribute::ActiveModel> = po
                .computation_attributes
                .iter()
                .filter(|attr| attr.action == DO_UPDATE)
                .map(|attr| make_computation_attribute_active_model(attr))
                .collect();
            return attr_list;
        })
        .collect();
    for update_computation_attr in update_computation_attr_list {
        computation_attribute::Entity::update(update_computation_attr)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("computation_attribute update failed");
                TcdtServiceError::build_internal_msg_error(
                    "computation_attribute update failed",
                    err,
                )
            })?;
    }
    let update_entity_associate_list: Vec<component_entity_associate::ActiveModel> = save_po
        .component_entity_associates
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_enti_asso_active_model(po))
        .collect();
    for update_entity_associate in update_entity_associate_list {
        component_entity_associate::Entity::update(update_entity_associate)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_entity_associate update failed");
                TcdtServiceError::build_internal_msg_error(
                    "component_entity_associate update failed",
                    err,
                )
            })?;
    }
    let new_node_uis: Vec<component_node_ui::ActiveModel> = save_po
        .component_node_uis
        .iter()
        .filter(|po| po.action == DO_UPDATE)
        .map(|po| make_node_ui_active_model(po))
        .collect();
    Ok(for update_node_ui in new_node_uis {
        component_node_ui::Entity::update(update_node_ui)
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("component_node_ui update failed");
                TcdtServiceError::build_internal_msg_error("component_node_ui update failed", err)
            })?;
    })
}

fn make_relation(save_po: &mut SavePO) {
    save_po.component_node_uis.iter_mut().for_each(|po| {
        po.id_component = Some(save_po.id_component.clone());
        if po.action == DO_NEW {
            po.id_component_node_ui = nanoid::nanoid!();
        }
    });

    save_po
        .component_entity_associates
        .iter_mut()
        .for_each(|po| {
            po.id_component = Some(save_po.id_component.clone());
            if po.action == DO_NEW {
                po.id_component_entity_associate = nanoid::nanoid!();
            }
        });

    save_po.component_enums.iter_mut().for_each(|po| {
        po.id_component = Some(save_po.id_component.clone());
        if po.action == DO_NEW {
            let old_id_enum = po.id_component_enum.clone();
            po.id_component_enum = nanoid::nanoid!();
            save_po
                .component_node_uis
                .iter_mut()
                .for_each(|component_node_ui| {
                    if component_node_ui.id_element == Some(old_id_enum.clone()) {
                        component_node_ui.id_element = Some(po.id_component_enum.clone());
                    }
                });
        }
    });

    save_po.component_entities.iter_mut().for_each(|po| {
        po.id_component = Some(save_po.id_component.clone());
        if po.action == DO_NEW {
            let old_id_entity = po.id_component_entity.clone();
            po.id_component_entity = nanoid::nanoid!();
            if save_po.id_main_component_entity == Some(old_id_entity.clone()) {
                save_po.id_main_component_entity = Some(po.id_component_entity.clone());
            }
            save_po
                .component_node_uis
                .iter_mut()
                .for_each(|component_node_ui| {
                    if component_node_ui.id_element == Some(old_id_entity.clone()) {
                        component_node_ui.id_element = Some(po.id_component_entity.clone());
                    }
                });
            save_po
                .component_entity_associates
                .iter_mut()
                .for_each(|asso| {
                    if asso.id_up_cp_entity == Some(old_id_entity.clone()) {
                        asso.id_up_cp_entity = Some(po.id_component_entity.clone());
                    }
                    if asso.id_down_cp_entity == Some(old_id_entity.clone()) {
                        asso.id_down_cp_entity = Some(po.id_component_entity.clone());
                    }
                });
        }
        po.ext_attributes.iter_mut().for_each(|attr| {
            attr.id_component_entity = Some(po.id_component_entity.clone());
            if po.action == DO_NEW {
                let _old_id_attribute = attr.id_ext_attribute.clone();
                attr.id_ext_attribute = nanoid::nanoid!();
            }
        });
        po.computation_attributes.iter_mut().for_each(|attr| {
            attr.id_component_entity = Some(po.id_component_entity.clone());
            if po.action == DO_NEW {
                let _old_id_computation_attribute = attr.id_computation_attribute.clone();
                attr.id_computation_attribute = nanoid::nanoid!();
            }
        });
    });
}

fn make_component_active_model(save_po: &mut SavePO) -> ComponentActiveModel {
    ComponentActiveModel {
        id_component: Set(save_po.id_component.clone()),
        id_main_component_entity: Set(save_po.id_main_component_entity.clone()),
        display_name: Set(save_po.display_name.clone()),
        package_name: Set(save_po.package_name.clone()),
        component_type: Set(save_po.component_type.clone()),
        id_component_module: Set(save_po.id_component_module.clone()),
    }
}

fn make_enti_asso_active_model(
    po: &crate::dto::po::ext::component::collection::ComponentEntityAssociatePO,
) -> component_entity_associate::ActiveModel {
    component_entity_associate::ActiveModel {
        id_component_entity_associate: Set(po.id_component_entity_associate.clone()),
        down_package_name: Set(po.down_package_name.clone()),
        up_package_name: Set(po.up_package_name.clone()),
        fg_agg_asso: Set(po.fg_agg_asso.clone()),
        id_component: Set(po.id_component.clone()),
        id_entity_associate: Set(po.id_entity_associate.clone()),
    }
}

fn make_enum_active_model(
    po: &crate::dto::po::ext::component::collection::ComponentEnumPO,
) -> component_enum::ActiveModel {
    component_enum::ActiveModel {
        id_component_enum: Set(po.id_component_enum.clone()),
        id_component: Set(po.id_component.clone()),
        id_enum: Set(po.id_enum.clone()),
    }
}

fn make_node_ui_active_model(
    po: &crate::dto::po::ext::component::collection::ComponentNodeUiPO,
) -> component_node_ui::ActiveModel {
    component_node_ui::ActiveModel {
        id_component_node_ui: Set(po.id_component_node_ui.clone()),
        x: Set(po.x.clone()),
        y: Set(po.y.clone()),
        width: Set(po.width.clone()),
        height: Set(po.height.clone()),
        id_element: Set(po.id_element.clone()),
        id_component: Set(po.id_component.clone()),
    }
}

fn make_entity_active_model(
    po: &crate::dto::po::ext::component::collection::ComponentEntityPO,
) -> component_entity::ActiveModel {
    component_entity::ActiveModel {
        id_component_entity: Set(po.id_component_entity.clone()),
        fg_virtual: Set(po.fg_virtual.clone()),
        id_component: Set(po.id_component.clone()),
        id_entity: Set(po.id_entity.clone()),
    }
}

fn make_computation_attribute_active_model(
    po: &crate::dto::po::ext::component::collection::ComputationAttributePO,
) -> computation_attribute::ActiveModel {
    computation_attribute::ActiveModel {
        id_computation_attribute: Set(po.id_computation_attribute.clone()),
        attribute_name: Set(po.attribute_name.clone()),
        display_name: Set(po.display_name.clone()),
        len: Set(po.len.clone()),
        fg_mandatory: Set(po.fg_mandatory.clone()),
        default_value: Set(po.default_value.clone()),
        pcs: Set(po.pcs.clone()),
        sn: Set(po.sn.clone()),
        id_attribute_type: Set(po.id_attribute_type.clone()),
        id_component_entity: Set(po.id_component_entity.clone()),
    }
}

fn make_entity_attribute_active_model(
    po: &crate::dto::po::ext::component::collection::ExtAttributePO,
) -> ext_attribute::ActiveModel {
    ext_attribute::ActiveModel {
        id_ext_attribute: Set(po.id_ext_attribute.clone()),
        ext1: Set(po.ext1.clone()),
        sn: Set(po.sn.clone()),
        id_component_entity: Set(po.id_component_entity.clone()),
        id_attribute: Set(po.id_attribute.clone()),
    }
}
