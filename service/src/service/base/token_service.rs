use crate::{
    common::{
        aq::*,
    },
    dto::po::base::token_po::TokenPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::token;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct TokenMutation;

impl TokenMutation {
    pub async fn create(
        db: &DbConn,
        token_model: token::Model,
    ) -> Result<token::Model, TcdtServiceError> {
        let mut token_active_model = token::convert_model_to_active_model(token_model);
        let id = generate_id();
        token_active_model.id_sys_token = Set(id.clone());
        let _ = token::Entity::insert(token_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Token insert failed",
                err,
            )
        })?;

        let token_save = token::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Token insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Token insert after cannot find entity"))?;
        Ok(token_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        token_model: token::Model,
    ) -> Result<token::Model, TcdtServiceError> {
        let id = token_model.id_sys_token.clone();

        let token_persist_model: token::ActiveModel = token::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Token update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Token update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut token_active_model = token::convert_model_to_active_model(token_model);

        let token_save = token_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Token update failed",
                err,
            )
        })?;

        Ok(token_save)
    }

    pub async fn delete(
        db: &DbConn,
        token_model: token::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = token::Entity::delete(token_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Token delete failed");
                TcdtServiceError::build_internal_msg_error("Token delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        token_model_list: Vec<token::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = token_model_list.iter().map(|token_model| {
            token_model.id_sys_token.clone()
        }).collect::<Vec<String>>();
        let delete_result = token::Entity::delete_many()
            .filter(Expr::col(token::Column::IdSysToken).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Token batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Token batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = token::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Token delete_all failed");
                TcdtServiceError::build_internal_msg_error("Token delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct TokenQuery;

impl TokenQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<token::Model, TcdtServiceError> {
        let token_entity =
            token::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("Token find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Token find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("Token cant not find data"))?;
        Ok(token_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<token::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idSysToken", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            token::Entity::default(),
            aq_condition,
            "sys_token",
            "Token",
        )?;

        let tokens = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Token find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Token find_by_ids failed", err)
            })?;

        Ok(tokens)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<token::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            token::Entity::default(),
            aq_condition,
            "sys_token",
            "Token",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<token::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Token num_items failed");
                TcdtServiceError::build_internal_msg_error("Token num_items failed", err)
            })?;
        let tokens = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Token fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Token fetch_page failed", err)
            })?;
        Ok((tokens, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<token::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            token::Entity::default(),
            aq_condition,
            "sys_token",
            "Token",
        )?;

        let tokens = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Token find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Token find_collection_by_condition failed", err)
            })?;

        Ok(tokens)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<token::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            token::Entity::default(),
            aq_condition,
            "sys_token",
            "Token",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Token find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Token find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            token::Entity::default(),
            aq_condition,
            "sys_token",
            "Token",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Token count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Token count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            token::Entity::default(),
            aq_condition,
            "sys_token",
            "Token",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Token exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Token exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
