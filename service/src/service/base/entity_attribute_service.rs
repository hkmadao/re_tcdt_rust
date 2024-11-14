use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::entity_attribute_po::EntityAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::entity_attribute;
use sea_orm::*;

pub struct EntityAttributeMutation;

impl EntityAttributeMutation {
    pub async fn create(
        db: &DbConn,
        entity_attribute_po: EntityAttributePO,
    ) -> Result<entity_attribute::Model, TcdtServiceError> {
        let entity_attribute_save = EntityAttributePO::insert(entity_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute insert failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute insert failed", err)
            })?;
        Ok(entity_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        entity_attribute_po: EntityAttributePO,
    ) -> Result<entity_attribute::Model, TcdtServiceError> {
        let entity_attribute_save = EntityAttributePO::update(entity_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute update failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute update failed", err)
            })?;
        Ok(entity_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        entity_attribute_po: EntityAttributePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = EntityAttributePO::delete(entity_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = entity_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct EntityAttributeQuery;

impl EntityAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<entity_attribute::Model, TcdtServiceError> {
        let entity_attribute_entity =
            entity_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("EntityAttribute find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("EntityAttribute find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("EntityAttribute cant not find data"))?;
        Ok(entity_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<entity_attribute::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idAttribute".to_string(),
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
            entity_attribute::Entity::default(),
            aq_conditoin,
            "dd_entity_attribute",
            "EntityAttribute",
        )?;

        let entity_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute find_by_ids failed", err)
            })?;

        Ok(entity_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<entity_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            entity_attribute::Entity::default(),
            aq_conditoin,
            "dd_entity_attribute",
            "EntityAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<entity_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("EntityAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute num_items failed", err)
            })?;
        let entity_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute fetch_page failed", err)
            })?;
        Ok((entity_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<entity_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_attribute::Entity::default(),
            aq_conditoin,
            "dd_entity_attribute",
            "EntityAttribute",
        )?;

        let entity_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute find_collection_by_condition failed", err)
            })?;

        Ok(entity_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<entity_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_attribute::Entity::default(),
            aq_conditoin,
            "dd_entity_attribute",
            "EntityAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_attribute::Entity::default(),
            aq_conditoin,
            "dd_entity_attribute",
            "EntityAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            entity_attribute::Entity::default(),
            aq_conditoin,
            "dd_entity_attribute",
            "EntityAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EntityAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EntityAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
