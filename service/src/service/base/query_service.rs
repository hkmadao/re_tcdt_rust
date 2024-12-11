use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::query_po::QueryPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::query;
use sea_orm::*;

pub struct QueryMutation;

impl QueryMutation {
    pub async fn create(
        db: &DbConn,
        query_po: QueryPO,
    ) -> Result<query::Model, TcdtServiceError> {
        let query_save = QueryPO::insert(query_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Query insert failed");
                TcdtServiceError::build_internal_msg_error("Query insert failed", err)
            })?;
        Ok(query_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        query_po: QueryPO,
    ) -> Result<query::Model, TcdtServiceError> {
        let query_save = QueryPO::update(query_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Query update failed");
                TcdtServiceError::build_internal_msg_error("Query update failed", err)
            })?;
        Ok(query_save)
    }

    pub async fn delete(
        db: &DbConn,
        query_po: QueryPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = QueryPO::delete(query_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Query delete failed");
                TcdtServiceError::build_internal_msg_error("Query delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = query::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Query delete_all failed");
                TcdtServiceError::build_internal_msg_error("Query delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct QueryQuery;

impl QueryQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<query::Model, TcdtServiceError> {
        let query_entity =
            query::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("Query find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("Query find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("Query cant not find data"))?;
        Ok(query_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<query::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idQuery".to_string(),
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
            query::Entity::default(),
            aq_condition,
            "ui_query",
            "Query",
        )?;

        let querys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Query find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Query find_by_ids failed", err)
            })?;

        Ok(querys)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<query::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            query::Entity::default(),
            aq_condition,
            "ui_query",
            "Query",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<query::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Query num_items failed");
                TcdtServiceError::build_internal_msg_error("Query num_items failed", err)
            })?;
        let querys = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Query fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Query fetch_page failed", err)
            })?;
        Ok((querys, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<query::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            query::Entity::default(),
            aq_condition,
            "ui_query",
            "Query",
        )?;

        let querys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Query find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Query find_collection_by_condition failed", err)
            })?;

        Ok(querys)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<query::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            query::Entity::default(),
            aq_condition,
            "ui_query",
            "Query",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Query find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Query find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            query::Entity::default(),
            aq_condition,
            "ui_query",
            "Query",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Query count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Query count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            query::Entity::default(),
            aq_condition,
            "ui_query",
            "Query",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Query exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Query exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
