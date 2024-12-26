use crate::{
    common::{
        aq::*,
    },
    dto::po::base::query_po::QueryPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::query;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct QueryMutation;

impl QueryMutation {
    pub async fn create(
        db: &DbConn,
        query_model: query::Model,
    ) -> Result<query::Model, TcdtServiceError> {
        let mut query_active_model = query::convert_model_to_active_model(query_model);
        let id = generate_id();
        query_active_model.id_query = Set(id.clone());
        let _ = query::Entity::insert(query_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Query insert failed",
                err,
            )
        })?;

        let query_save = query::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Query insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Query insert after cannot find entity"))?;
        Ok(query_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        query_model: query::Model,
    ) -> Result<query::Model, TcdtServiceError> {
        let id = query_model.id_query.clone();

        let query_persist_model: query::ActiveModel = query::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Query update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Query update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut query_active_model = query::convert_model_to_active_model(query_model);

        let query_save = query_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Query update failed",
                err,
            )
        })?;

        Ok(query_save)
    }

    pub async fn delete(
        db: &DbConn,
        query_model: query::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = query::Entity::delete(query_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Query delete failed");
                TcdtServiceError::build_internal_msg_error("Query delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        query_model_list: Vec<query::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = query_model_list.iter().map(|query_model| {
            query_model.id_query.clone()
        }).collect::<Vec<String>>();
        let delete_result = query::Entity::delete_many()
            .filter(Expr::col(query::Column::IdQuery).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Query batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Query batch_delete failed", err)
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
        let aq_condition = AqCondition::build_in_condition("idQuery", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

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
