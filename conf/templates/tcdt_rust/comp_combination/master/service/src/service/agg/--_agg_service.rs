use crate::common::aq::*;
use crate::common::aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN};
use crate::dto::po::agg::{{ rootInfo.snakeCaseName }}_agg_po::{{ rootInfo.pascalCaseName }}AggPO;
use crate::util::dyn_query::make_select_by_condition;
use ::entity::entity::{{ rootInfo.snakeCaseName }};
use sea_orm::*;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtSaveParamObjectTrait;

pub struct {{ rootInfo.pascalCaseName }}AggMutation;

impl {{ rootInfo.pascalCaseName }}AggMutation {
    pub async fn save(
        db: &DbConn,
        {{ rootInfo.snakeCaseName }}_agg_po: {{ rootInfo.pascalCaseName }}AggPO,
    ) -> Result<{{ rootInfo.snakeCaseName }}::Model, TcdtServiceError> {
        let save_after = {{ rootInfo.pascalCaseName }}AggPO::save({{ rootInfo.snakeCaseName }}_agg_po, db, None)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} save failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} save failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("cant not find data"))?;
        Ok(save_after)
    }

    pub async fn delete(db: &DbConn, id: String) -> Result<DeleteResult, TcdtServiceError> {
        let {{ rootInfo.snakeCaseName }}: {{ rootInfo.snakeCaseName }}::ActiveModel = {{ rootInfo.snakeCaseName }}::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} find_by_id failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_by_id failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("cant not find data"))?
            .into_active_model();
        let delete_result = {{ rootInfo.snakeCaseName }}.delete(db).await.map_err(|err| {
            log::error!("{{ rootInfo.pascalCaseName }} delete failed");
            TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} delete failed", err)
        })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = {{ rootInfo.snakeCaseName }}::Entity::delete_many().exec(db).await.map_err(|err| {
            log::error!("{{ rootInfo.pascalCaseName }} delete_many failed");
            TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} delete_many failed", err)
        })?;
        Ok(delete_result)
    }
}

pub struct {{ rootInfo.pascalCaseName }}AggQuery;

impl {{ rootInfo.pascalCaseName }}AggQuery {
    pub async fn find_by_id(db: &DbConn, id: String) -> Result<{{ rootInfo.snakeCaseName }}::Model, TcdtServiceError> {
        let {{ rootInfo.snakeCaseName }}_entity = {{ rootInfo.snakeCaseName }}::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} find_by_id failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_by_id failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg("cant not find data"))?;
        Ok({{ rootInfo.snakeCaseName }}_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<{{ rootInfo.snakeCaseName }}::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
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
            make_select_by_condition({{ rootInfo.snakeCaseName }}::Entity::default(), aq_condition, "sys_{{ rootInfo.snakeCaseName }}", "{{ rootInfo.pascalCaseName }}")?;

        let bill_forms = sql_build.all(db).await.map_err(|err| {
            log::error!("{{ rootInfo.pascalCaseName }} find_by_ids failed");
            TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_by_ids failed", err)
        })?;

        Ok(bill_forms)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<{{ rootInfo.snakeCaseName }}::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build =
            make_select_by_condition({{ rootInfo.snakeCaseName }}::Entity::default(), aq_condition, "sys_{{ rootInfo.snakeCaseName }}", "{{ rootInfo.pascalCaseName }}")?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<{{ rootInfo.snakeCaseName }}::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items().await.map_err(|err| {
            log::error!("{{ rootInfo.pascalCaseName }} num_items failed");
            TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} num_items failed", err)
        })?;
        let bill_forms = paginator.fetch_page(page_index - 1).await.map_err(|err| {
            log::error!("{{ rootInfo.pascalCaseName }} fetch_page failed");
            TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} fetch_page failed", err)
        })?;
        Ok((bill_forms, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<{{ rootInfo.snakeCaseName }}::Model>, TcdtServiceError> {
        let sql_build =
            make_select_by_condition({{ rootInfo.snakeCaseName }}::Entity::default(), aq_condition, "sys_{{ rootInfo.snakeCaseName }}", "{{ rootInfo.pascalCaseName }}")?;

        let bill_forms = sql_build.all(db).await.map_err(|err| {
            log::error!("{{ rootInfo.pascalCaseName }} find_collection_by_condition failed");
            TcdtServiceError::build_internal_msg_error(
                "{{ rootInfo.pascalCaseName }} find_collection_by_condition failed",
                err,
            )
        })?;

        Ok(bill_forms)
    }
}
