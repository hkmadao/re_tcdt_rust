use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dd_entity_po::DdEntityPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dd_entity;
use sea_orm::*;

pub struct DdEntityMutation;

impl DdEntityMutation {
    pub async fn create(
        db: &DbConn,
        dd_entity_po: DdEntityPO,
    ) -> Result<dd_entity::Model, TcdtServiceError> {
        let dd_entity_save = DdEntityPO::insert(dd_entity_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DdEntity insert failed");
                TcdtServiceError::build_internal_msg_error("DdEntity insert failed", err)
            })?;
        Ok(dd_entity_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dd_entity_po: DdEntityPO,
    ) -> Result<dd_entity::Model, TcdtServiceError> {
        let dd_entity_save = DdEntityPO::update(dd_entity_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DdEntity update failed");
                TcdtServiceError::build_internal_msg_error("DdEntity update failed", err)
            })?;
        Ok(dd_entity_save)
    }

    pub async fn delete(
        db: &DbConn,
        dd_entity_po: DdEntityPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DdEntityPO::delete(dd_entity_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DdEntity delete failed");
                TcdtServiceError::build_internal_msg_error("DdEntity delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dd_entity::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity delete_all failed");
                TcdtServiceError::build_internal_msg_error("DdEntity delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DdEntityQuery;

impl DdEntityQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dd_entity::Model, TcdtServiceError> {
        let dd_entity_entity =
            dd_entity::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DdEntity find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DdEntity find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DdEntity cant not find data"))?;
        Ok(dd_entity_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dd_entity::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idEntity".to_string(),
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
            dd_entity::Entity::default(),
            aq_conditoin,
            "dd_entity",
            "DdEntity",
        )?;

        let dd_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_by_ids failed", err)
            })?;

        Ok(dd_entitys)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dd_entity::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_conditoin,
            "dd_entity",
            "DdEntity",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dd_entity::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DdEntity num_items failed");
                TcdtServiceError::build_internal_msg_error("DdEntity num_items failed", err)
            })?;
        let dd_entitys = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DdEntity fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DdEntity fetch_page failed", err)
            })?;
        Ok((dd_entitys, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<dd_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_conditoin,
            "dd_entity",
            "DdEntity",
        )?;

        let dd_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_collection_by_condition failed", err)
            })?;

        Ok(dd_entitys)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<dd_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_conditoin,
            "dd_entity",
            "DdEntity",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_conditoin,
            "dd_entity",
            "DdEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_conditoin,
            "dd_entity",
            "DdEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
