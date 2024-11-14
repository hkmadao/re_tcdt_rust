use crate::common::aq::*;
use crate::common::aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN};
use crate::dto::po::agg::role_agg_po::RoleAggPO;
use crate::util::dyn_query::make_select_by_condition;
use ::entity::entity::role;
use sea_orm::*;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtSaveParamObjectTrait;

pub struct RoleAggMutation;

impl RoleAggMutation {
    pub async fn save(
        db: &DbConn,
        role_agg_po: RoleAggPO,
    ) -> Result<role::Model, TcdtServiceError> {
        let save_after = RoleAggPO::save(role_agg_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Role save failed");
                TcdtServiceError::build_internal_msg_error("Role save failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("cant not find data"))?;
        Ok(save_after)
    }

    pub async fn delete(db: &DbConn, id: String) -> Result<DeleteResult, TcdtServiceError> {
        let role: role::ActiveModel = role::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("Role find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Role find_by_id failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("cant not find data"))?
            .into_active_model();
        let delete_result = role.delete(db).await.map_err(|err| {
            log::error!("Role delete failed");
            TcdtServiceError::build_internal_msg_error("Role delete failed", err)
        })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = role::Entity::delete_many().exec(db).await.map_err(|err| {
            log::error!("Role delete_many failed");
            TcdtServiceError::build_internal_msg_error("Role delete_many failed", err)
        })?;
        Ok(delete_result)
    }
}

pub struct RoleAggQuery;

impl RoleAggQuery {
    pub async fn find_by_id(db: &DbConn, id: String) -> Result<role::Model, TcdtServiceError> {
        let role_entity = role::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("Role find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Role find_by_id failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("cant not find data"))?;
        Ok(role_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<role::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idBillForm".to_string(),
                    operator_code: OPERATOR_CODE_IN.to_owned(),
                    filter_params: ids
                        .iter()
                        .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
                        .collect(),
                }],
            })),
            orders: vec![],
        };
        let sql_build =
            make_select_by_condition(role::Entity::default(), aq_conditoin, "sys_role", "Role")?;

        let bill_forms = sql_build.all(db).await.map_err(|err| {
            log::error!("Role find_by_ids failed");
            TcdtServiceError::build_internal_msg_error("Role find_by_ids failed", err)
        })?;

        Ok(bill_forms)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<role::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build =
            make_select_by_condition(role::Entity::default(), aq_conditoin, "sys_role", "Role")?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<role::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items().await.map_err(|err| {
            log::error!("Role num_items failed");
            TcdtServiceError::build_internal_msg_error("Role num_items failed", err)
        })?;
        let bill_forms = paginator.fetch_page(page_index - 1).await.map_err(|err| {
            log::error!("Role fetch_page failed");
            TcdtServiceError::build_internal_msg_error("Role fetch_page failed", err)
        })?;
        Ok((bill_forms, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<role::Model>, TcdtServiceError> {
        let sql_build =
            make_select_by_condition(role::Entity::default(), aq_conditoin, "sys_role", "Role")?;

        let bill_forms = sql_build.all(db).await.map_err(|err| {
            log::error!("Role find_collection_by_condition failed");
            TcdtServiceError::build_internal_msg_error(
                "Role find_collection_by_condition failed",
                err,
            )
        })?;

        Ok(bill_forms)
    }
}
