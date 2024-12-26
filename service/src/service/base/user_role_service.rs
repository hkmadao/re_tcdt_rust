use crate::{
    common::{
        aq::*,
    },
    dto::po::base::user_role_po::UserRolePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::user_role;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct UserRoleMutation;

impl UserRoleMutation {
    pub async fn create(
        db: &DbConn,
        user_role_model: user_role::Model,
    ) -> Result<user_role::Model, TcdtServiceError> {
        let mut user_role_active_model = user_role::convert_model_to_active_model(user_role_model);
        let id = generate_id();
        user_role_active_model.id_sys_user_role = Set(id.clone());
        let _ = user_role::Entity::insert(user_role_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "UserRole insert failed",
                err,
            )
        })?;

        let user_role_save = user_role::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "UserRole insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("UserRole insert after cannot find entity"))?;
        Ok(user_role_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        user_role_model: user_role::Model,
    ) -> Result<user_role::Model, TcdtServiceError> {
        let id = user_role_model.id_sys_user_role.clone();

        let user_role_persist_model: user_role::ActiveModel = user_role::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "UserRole update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("UserRole update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut user_role_active_model = user_role::convert_model_to_active_model(user_role_model);

        let user_role_save = user_role_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " UserRole update failed",
                err,
            )
        })?;

        Ok(user_role_save)
    }

    pub async fn delete(
        db: &DbConn,
        user_role_model: user_role::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = user_role::Entity::delete(user_role_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("UserRole delete failed");
                TcdtServiceError::build_internal_msg_error("UserRole delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        user_role_model_list: Vec<user_role::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = user_role_model_list.iter().map(|user_role_model| {
            user_role_model.id_sys_user_role.clone()
        }).collect::<Vec<String>>();
        let delete_result = user_role::Entity::delete_many()
            .filter(Expr::col(user_role::Column::IdSysUserRole).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("UserRole batch_delete failed");
                TcdtServiceError::build_internal_msg_error("UserRole batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = user_role::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("UserRole delete_all failed");
                TcdtServiceError::build_internal_msg_error("UserRole delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct UserRoleQuery;

impl UserRoleQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<user_role::Model, TcdtServiceError> {
        let user_role_entity =
            user_role::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("UserRole find_by_id failed");
                TcdtServiceError::build_internal_msg_error("UserRole find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("UserRole cant not find data"))?;
        Ok(user_role_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<user_role::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idSysUserRole", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_condition,
            "sys_user_role",
            "UserRole",
        )?;

        let user_roles = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("UserRole find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("UserRole find_by_ids failed", err)
            })?;

        Ok(user_roles)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<user_role::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_condition,
            "sys_user_role",
            "UserRole",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<user_role::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("UserRole num_items failed");
                TcdtServiceError::build_internal_msg_error("UserRole num_items failed", err)
            })?;
        let user_roles = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("UserRole fetch_page failed");
                TcdtServiceError::build_internal_msg_error("UserRole fetch_page failed", err)
            })?;
        Ok((user_roles, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<user_role::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_condition,
            "sys_user_role",
            "UserRole",
        )?;

        let user_roles = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("UserRole find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("UserRole find_collection_by_condition failed", err)
            })?;

        Ok(user_roles)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<user_role::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_condition,
            "sys_user_role",
            "UserRole",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("UserRole find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("UserRole find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_condition,
            "sys_user_role",
            "UserRole",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("UserRole count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("UserRole count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_condition,
            "sys_user_role",
            "UserRole",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("UserRole exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("UserRole exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
