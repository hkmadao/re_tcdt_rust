use crate::dto::vo::base::data_type_vo::DataTypeVO;
use crate::service::{
    base::{
        dd_entity_service::DdEntityQuery, dd_enum_service::DdEnumQuery,
        entity_collection_service::EntityCollectionQuery,
    },
    ext::data_type_ext_service::DataTypeExtQuery,
};
use ::entity::entity::{
    dd_entity, dd_enum, entity_associate, entity_attribute, entity_collection, enum_associate,
    enum_attribute, project, sub_project,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tcdt_common::tcdt_service_error::TcdtServiceError;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CollCache {
    pub project_entity: project::Model,
    pub column_domain_type_map: HashMap<String, DataTypeVO>,
    pub coll_list: Vec<entity_collection::Model>,
    pub all_entity_list: Vec<dd_entity::Model>,
    pub inner_entity_list: Vec<dd_entity::Model>,
    pub out_entity_list: Vec<dd_entity::Model>,
    pub attr_list: Vec<entity_attribute::Model>,
    pub all_enum_list: Vec<dd_enum::Model>,
    pub inner_enum_list: Vec<dd_enum::Model>,
    pub out_enum_list: Vec<dd_enum::Model>,
    pub enum_attr_list: Vec<enum_attribute::Model>,
    pub entity_associate_list: Vec<entity_associate::Model>,
    pub enum_associater_list: Vec<enum_associate::Model>,
}

impl CollCache {
    pub async fn load_coll_data(
        db: &DbConn,
        id_coll: String,
    ) -> Result<CollCache, TcdtServiceError> {
        let coll_entity = EntityCollectionQuery::find_by_id(db, id_coll.clone()).await?;
        let project_entity = get_project(db, &coll_entity).await?;
        let column_domain_type_map = DataTypeExtQuery::find_and_make_map_by_project_id(
            db,
            project_entity.id_project.clone(),
        )
        .await?;

        let entity_associate_list = coll_entity
            .find_linked(entity_collection::EntityAssociatesLinked)
            .order_by(entity_associate::Column::IdEntityAssociate, Order::Asc)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find entity_associate failed");
                TcdtServiceError::build_internal_msg_error("find entity_associate failed", err)
            })?;

        let enum_associate_list = coll_entity
            .find_linked(entity_collection::EnumAssociatesLinked)
            .order_by(enum_associate::Column::IdEnumAssociate, Order::Asc)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find enum_associate failed");
                TcdtServiceError::build_internal_msg_error("find enum_associate failed", err)
            })?;

        let inner_entity_list = coll_entity
            .find_linked(entity_collection::EntitiesLinked)
            .order_by(dd_entity::Column::ClassName, Order::Asc)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find dd_entity failed");
                TcdtServiceError::build_internal_msg_error("find dd_entity failed", err)
            })?;

        let inner_entity_id_list = inner_entity_list
            .iter()
            .map(|enti| enti.id_entity.clone())
            .collect::<Vec<_>>();
        let out_entity_id_list = entity_associate_list
            .iter()
            .filter(|asso| {
                asso.id_up.is_some() && !inner_entity_id_list.contains(&asso.id_up.clone().unwrap())
            })
            .map(|asso| asso.id_up.clone().unwrap())
            .collect::<Vec<_>>();
        let mut all_entity_list = inner_entity_list.clone();
        let mut out_entity_list: Vec<dd_entity::Model> = vec![];
        if out_entity_id_list.len() > 0 {
            let mut out_entity_list_temp =
                DdEntityQuery::find_by_ids(db, out_entity_id_list).await?;
            out_entity_list = out_entity_list_temp.clone();
            all_entity_list.append(&mut out_entity_list_temp);
        }
        let entity_id_list = all_entity_list
            .iter()
            .map(|enti| enti.id_entity.clone())
            .collect::<Vec<_>>();
        let entity_attr_list = entity_attribute::Entity::find()
            .filter(entity_attribute::Column::IdEntity.is_in(entity_id_list))
            .order_by_asc(entity_attribute::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error("find entity attribute faild", err)
            })?;

        let inner_enum_list = coll_entity
            .find_linked(entity_collection::EnumsLinked)
            .order_by(dd_enum::Column::ClassName, Order::Asc)
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find dd_enum failed");
                TcdtServiceError::build_internal_msg_error("find dd_enum failed", err)
            })?;

        let inner_enum_id_list = inner_enum_list
            .iter()
            .map(|enti| enti.id_enum.clone())
            .collect::<Vec<_>>();
        let out_enum_id_list = enum_associate_list
            .iter()
            .filter(|asso| {
                asso.id_enum.is_some()
                    && !inner_enum_id_list.contains(&asso.id_enum.clone().unwrap())
            })
            .map(|asso| asso.id_enum.clone().unwrap())
            .collect::<Vec<_>>();
        let mut all_enum_list = inner_enum_list.clone();
        let mut out_enum_list: Vec<dd_enum::Model> = vec![];
        if out_enum_id_list.len() > 0 {
            let mut out_enum_list_temp = DdEnumQuery::find_by_ids(db, out_enum_id_list).await?;
            out_enum_list = out_enum_list_temp.clone();
            all_enum_list.append(&mut out_enum_list_temp);
        }
        let enum_id_list = all_enum_list
            .iter()
            .map(|enti| enti.id_enum.clone())
            .collect::<Vec<_>>();
        let enum_attr_list = enum_attribute::Entity::find()
            .filter(enum_attribute::Column::IdEnum.is_in(enum_id_list))
            .order_by_asc(enum_attribute::Column::Sn)
            .all(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error("find enum attribute faild", err)
            })?;

        let mut out_coll_id_list = out_entity_list
            .iter()
            .filter(|enti| enti.id_entity_collection.is_some())
            .map(|enti| enti.id_entity_collection.clone().unwrap())
            .collect::<Vec<_>>();

        let mut out_enum_coll_id_list = out_enum_list
            .iter()
            .filter(|enti| enti.id_entity_collection.is_some())
            .map(|enti| enti.id_entity_collection.clone().unwrap())
            .collect::<Vec<_>>();

        out_coll_id_list.append(&mut out_enum_coll_id_list);

        out_coll_id_list.push(id_coll.clone());

        let coll_list = EntityCollectionQuery::find_by_ids(db, out_coll_id_list).await?;

        Ok(CollCache {
            project_entity,
            column_domain_type_map,
            coll_list: coll_list,
            all_entity_list,
            inner_entity_list,
            out_entity_list,
            attr_list: entity_attr_list,
            all_enum_list,
            inner_enum_list,
            out_enum_list,
            enum_attr_list: enum_attr_list,
            entity_associate_list: entity_associate_list,
            enum_associater_list: enum_associate_list,
        })
    }
}

async fn get_project(
    db: &DbConn,
    coll_entity: &entity_collection::Model,
) -> Result<project::Model, TcdtServiceError> {
    let sub_project_entity = coll_entity
        .find_linked(entity_collection::SubProjectLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find sub project failed");
            TcdtServiceError::build_internal_msg_error("find sub project failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find sub project",
        ))?;
    let project_entity = sub_project_entity
        .find_linked(sub_project::ProjectLinked)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("find project failed");
            TcdtServiceError::build_internal_msg_error("find project failed", err)
        })?
        .ok_or(TcdtServiceError::build_internal_msg(
            "cant not find project",
        ))?;
    Ok(project_entity)
}
