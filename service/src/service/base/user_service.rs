use crate::{
    common::{
        aq::*,
    },
    dto::po::base::user_po::UserPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::user;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct UserMutation;

impl UserMutation {
    pub async fn create(
        db: &DbConn,
        user_model: user::Model,
    ) -> Result<user::Model, TcdtServiceError> {
        let mut user_active_model = user::convert_model_to_active_model(user_model);
        let id = generate_id();
        user_active_model.id_user = Set(id.clone());
        let _ = user::Entity::insert(user_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "User insert failed",
                err,
            )
        })?;

        let user_save = user::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "User insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("User insert after cannot find entity"))?;
        Ok(user_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        user_model: user::Model,
    ) -> Result<user::Model, TcdtServiceError> {
        let id = user_model.id_user.clone();

        let user_persist_model: user::ActiveModel = user::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "User update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("User update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut user_active_model = user::convert_model_to_active_model(user_model);

        let user_save = user_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " User update failed",
                err,
            )
        })?;

        Ok(user_save)
    }

    pub async fn delete(
        db: &DbConn,
        user_model: user::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = user::Entity::delete(user_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("User delete failed");
                TcdtServiceError::build_internal_msg_error("User delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        user_model_list: Vec<user::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = user_model_list.iter().map(|user_model| {
            user_model.id_user.clone()
        }).collect::<Vec<String>>();
        let delete_result = user::Entity::delete_many()
            .filter(Expr::col(user::Column::IdUser).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("User batch_delete failed");
                TcdtServiceError::build_internal_msg_error("User batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = user::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("User delete_all failed");
                TcdtServiceError::build_internal_msg_error("User delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct UserQuery;

impl UserQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<user::Model, TcdtServiceError> {
        let user_entity =
            user::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("User find_by_id failed");
                TcdtServiceError::build_internal_msg_error("User find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("User cant not find data"))?;
        Ok(user_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<user::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idUser", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let users = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("User find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("User find_by_ids failed", err)
            })?;

        Ok(users)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<user::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<user::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("User num_items failed");
                TcdtServiceError::build_internal_msg_error("User num_items failed", err)
            })?;
        let users = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("User fetch_page failed");
                TcdtServiceError::build_internal_msg_error("User fetch_page failed", err)
            })?;
        Ok((users, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<user::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let users = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("User find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User find_collection_by_condition failed", err)
            })?;

        Ok(users)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<user::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("User find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("User count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("User exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
