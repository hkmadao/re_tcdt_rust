use crate::{
    common::{
        aq::*,
    },
    dto::po::base::role_po::RolePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::role;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct RoleMutation;

impl RoleMutation {
    pub async fn create(
        db: &DbConn,
        role_model: role::Model,
    ) -> Result<role::Model, TcdtServiceError> {
        let mut role_active_model = role::convert_model_to_active_model(role_model);
        let id = generate_id();
        role_active_model.id_role = Set(id.clone());
        let _ = role::Entity::insert(role_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Role insert failed",
                err,
            )
        })?;

        let role_save = role::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Role insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Role insert after cannot find entity"))?;
        Ok(role_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        role_model: role::Model,
    ) -> Result<role::Model, TcdtServiceError> {
        let id = role_model.id_role.clone();

        let role_persist_model: role::ActiveModel = role::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Role update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Role update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut role_active_model = role::convert_model_to_active_model(role_model);

        let role_save = role_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Role update failed",
                err,
            )
        })?;

        Ok(role_save)
    }

    pub async fn delete(
        db: &DbConn,
        role_model: role::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = role::Entity::delete(role_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Role delete failed");
                TcdtServiceError::build_internal_msg_error("Role delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        role_model_list: Vec<role::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = role_model_list.iter().map(|role_model| {
            role_model.id_role.clone()
        }).collect::<Vec<String>>();
        let delete_result = role::Entity::delete_many()
            .filter(Expr::col(role::Column::IdRole).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Role batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Role batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = role::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Role delete_all failed");
                TcdtServiceError::build_internal_msg_error("Role delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct RoleQuery;

impl RoleQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<role::Model, TcdtServiceError> {
        let role_entity =
            role::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("Role find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Role find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("Role cant not find data"))?;
        Ok(role_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<role::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idRole", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            role::Entity::default(),
            aq_condition,
            "sys_role",
            "Role",
        )?;

        let roles = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Role find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Role find_by_ids failed", err)
            })?;

        Ok(roles)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<role::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            role::Entity::default(),
            aq_condition,
            "sys_role",
            "Role",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<role::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Role num_items failed");
                TcdtServiceError::build_internal_msg_error("Role num_items failed", err)
            })?;
        let roles = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Role fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Role fetch_page failed", err)
            })?;
        Ok((roles, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<role::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role::Entity::default(),
            aq_condition,
            "sys_role",
            "Role",
        )?;

        let roles = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Role find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Role find_collection_by_condition failed", err)
            })?;

        Ok(roles)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<role::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role::Entity::default(),
            aq_condition,
            "sys_role",
            "Role",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Role find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Role find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role::Entity::default(),
            aq_condition,
            "sys_role",
            "Role",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Role count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Role count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role::Entity::default(),
            aq_condition,
            "sys_role",
            "Role",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Role exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Role exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
