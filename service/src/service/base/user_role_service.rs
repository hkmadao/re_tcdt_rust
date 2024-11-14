use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::user_role_po::UserRolePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::user_role;
use sea_orm::*;

pub struct UserRoleMutation;

impl UserRoleMutation {
    pub async fn create(
        db: &DbConn,
        user_role_po: UserRolePO,
    ) -> Result<user_role::Model, TcdtServiceError> {
        let user_role_save = UserRolePO::insert(user_role_po, db, None)
            .await
            .map_err(|err| {
                log::error!("UserRole insert failed");
                TcdtServiceError::build_internal_msg_error("UserRole insert failed", err)
            })?;
        Ok(user_role_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        user_role_po: UserRolePO,
    ) -> Result<user_role::Model, TcdtServiceError> {
        let user_role_save = UserRolePO::update(user_role_po, db, None)
            .await
            .map_err(|err| {
                log::error!("UserRole update failed");
                TcdtServiceError::build_internal_msg_error("UserRole update failed", err)
            })?;
        Ok(user_role_save)
    }

    pub async fn delete(
        db: &DbConn,
        user_role_po: UserRolePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = UserRolePO::delete(user_role_po, db, None)
            .await
            .map_err(|err| {
                log::error!("UserRole delete failed");
                TcdtServiceError::build_internal_msg_error("UserRole delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        user_role_po_list: Vec<UserRolePO>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = user_role_po_list
            .iter()
            .map(|po| po.id_sys_user_role.clone())
            .collect::<Vec<_>>();
        let delete_result = user_role::Entity::delete_many()
            .filter(user_role::Column::IdSysUserRole.is_in(id_list))
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
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idSysUserRole".to_string(),
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
            user_role::Entity::default(),
            aq_conditoin,
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
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<Vec<user_role::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<Option<user_role::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user_role::Entity::default(),
            aq_conditoin,
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
