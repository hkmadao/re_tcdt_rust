use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::entity_collection_po::EntityCollectionPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::entity_collection;
use sea_orm::*;

pub struct EntityCollectionMutation;

impl EntityCollectionMutation {
    pub async fn create(
        db: &DbConn,
        entity_collection_po: EntityCollectionPO,
    ) -> Result<entity_collection::Model, TcdtServiceError> {
        let entity_collection_save = EntityCollectionPO::insert(entity_collection_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityCollection insert failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection insert failed", err)
            })?;
        Ok(entity_collection_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        entity_collection_po: EntityCollectionPO,
    ) -> Result<entity_collection::Model, TcdtServiceError> {
        let entity_collection_save = EntityCollectionPO::update(entity_collection_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityCollection update failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection update failed", err)
            })?;
        Ok(entity_collection_save)
    }

    pub async fn delete(
        db: &DbConn,
        entity_collection_po: EntityCollectionPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = EntityCollectionPO::delete(entity_collection_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityCollection delete failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = entity_collection::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection delete_all failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct EntityCollectionQuery;

impl EntityCollectionQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<entity_collection::Model, TcdtServiceError> {
        let entity_collection_entity =
            entity_collection::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("EntityCollection find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("EntityCollection find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("EntityCollection cant not find data"))?;
        Ok(entity_collection_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<entity_collection::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idEntityCollection".to_string(),
                    operator_code: OPERATOR_CODE_IN.to_owned(),
                    filter_params: ids
                        .iter()
                        .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
                        .collect(),
                }],
            })),
            orders: vec![],
        };
        let sql_build = make_select_by_condition(
            entity_collection::Entity::default(),
            aq_conditoin,
            "dd_entity_collection",
            "EntityCollection",
        )?;

        let entity_collections = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection find_by_ids failed", err)
            })?;

        Ok(entity_collections)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<entity_collection::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            entity_collection::Entity::default(),
            aq_conditoin,
            "dd_entity_collection",
            "EntityCollection",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<entity_collection::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("EntityCollection num_items failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection num_items failed", err)
            })?;
        let entity_collections = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("EntityCollection fetch_page failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection fetch_page failed", err)
            })?;
        Ok((entity_collections, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<entity_collection::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_collection::Entity::default(),
            aq_conditoin,
            "dd_entity_collection",
            "EntityCollection",
        )?;

        let entity_collections = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection find_collection_by_condition failed", err)
            })?;

        Ok(entity_collections)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<entity_collection::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_collection::Entity::default(),
            aq_conditoin,
            "dd_entity_collection",
            "EntityCollection",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_collection::Entity::default(),
            aq_conditoin,
            "dd_entity_collection",
            "EntityCollection",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_collection::Entity::default(),
            aq_conditoin,
            "dd_entity_collection",
            "EntityCollection",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EntityCollection exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityCollection exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
