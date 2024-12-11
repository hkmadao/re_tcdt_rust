use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::role_po::RolePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::role;
use sea_orm::*;

pub struct RoleMutation;

impl RoleMutation {
    pub async fn create(
        db: &DbConn,
        role_po: RolePO,
    ) -> Result<role::Model, TcdtServiceError> {
        let role_save = RolePO::insert(role_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Role insert failed");
                TcdtServiceError::build_internal_msg_error("Role insert failed", err)
            })?;
        Ok(role_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        role_po: RolePO,
    ) -> Result<role::Model, TcdtServiceError> {
        let role_save = RolePO::update(role_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Role update failed");
                TcdtServiceError::build_internal_msg_error("Role update failed", err)
            })?;
        Ok(role_save)
    }

    pub async fn delete(
        db: &DbConn,
        role_po: RolePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = RolePO::delete(role_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Role delete failed");
                TcdtServiceError::build_internal_msg_error("Role delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        role_po_list: Vec<RolePO>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = role_po_list
            .iter()
            .map(|po| po.id_role.clone())
            .collect::<Vec<_>>();
        let delete_result = role::Entity::delete_many()
            .filter(role::Column::IdRole.is_in(id_list))
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
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idRole".to_string(),
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
