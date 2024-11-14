use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::entity_associate_po::EntityAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::entity_associate;
use sea_orm::*;

pub struct EntityAssociateMutation;

impl EntityAssociateMutation {
    pub async fn create(
        db: &DbConn,
        entity_associate_po: EntityAssociatePO,
    ) -> Result<entity_associate::Model, TcdtServiceError> {
        let entity_associate_save = EntityAssociatePO::insert(entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate insert failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate insert failed", err)
            })?;
        Ok(entity_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        entity_associate_po: EntityAssociatePO,
    ) -> Result<entity_associate::Model, TcdtServiceError> {
        let entity_associate_save = EntityAssociatePO::update(entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate update failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate update failed", err)
            })?;
        Ok(entity_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        entity_associate_po: EntityAssociatePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = EntityAssociatePO::delete(entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = entity_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct EntityAssociateQuery;

impl EntityAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<entity_associate::Model, TcdtServiceError> {
        let entity_associate_entity =
            entity_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("EntityAssociate find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("EntityAssociate find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("EntityAssociate cant not find data"))?;
        Ok(entity_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<entity_associate::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idEntityAssociate".to_string(),
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
            entity_associate::Entity::default(),
            aq_conditoin,
            "dd_entity_associate",
            "EntityAssociate",
        )?;

        let entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate find_by_ids failed", err)
            })?;

        Ok(entity_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<entity_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            entity_associate::Entity::default(),
            aq_conditoin,
            "dd_entity_associate",
            "EntityAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<entity_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("EntityAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate num_items failed", err)
            })?;
        let entity_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate fetch_page failed", err)
            })?;
        Ok((entity_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_associate::Entity::default(),
            aq_conditoin,
            "dd_entity_associate",
            "EntityAssociate",
        )?;

        let entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate find_collection_by_condition failed", err)
            })?;

        Ok(entity_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_associate::Entity::default(),
            aq_conditoin,
            "dd_entity_associate",
            "EntityAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_associate::Entity::default(),
            aq_conditoin,
            "dd_entity_associate",
            "EntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_associate::Entity::default(),
            aq_conditoin,
            "dd_entity_associate",
            "EntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EntityAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
