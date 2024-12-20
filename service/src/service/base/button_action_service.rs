use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::button_action_po::ButtonActionPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::button_action;
use sea_orm::*;

pub struct ButtonActionMutation;

impl ButtonActionMutation {
    pub async fn create(
        db: &DbConn,
        button_action_po: ButtonActionPO,
    ) -> Result<button_action::Model, TcdtServiceError> {
        let button_action_save = ButtonActionPO::insert(button_action_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ButtonAction insert failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction insert failed", err)
            })?;
        Ok(button_action_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        button_action_po: ButtonActionPO,
    ) -> Result<button_action::Model, TcdtServiceError> {
        let button_action_save = ButtonActionPO::update(button_action_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ButtonAction update failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction update failed", err)
            })?;
        Ok(button_action_save)
    }

    pub async fn delete(
        db: &DbConn,
        button_action_po: ButtonActionPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ButtonActionPO::delete(button_action_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ButtonAction delete failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = button_action::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction delete_all failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ButtonActionQuery;

impl ButtonActionQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<button_action::Model, TcdtServiceError> {
        let button_action_entity =
            button_action::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("ButtonAction find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("ButtonAction find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("ButtonAction cant not find data"))?;
        Ok(button_action_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<button_action::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idButtonAction".to_string(),
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
            button_action::Entity::default(),
            aq_condition,
            "ui_button_action",
            "ButtonAction",
        )?;

        let button_actions = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction find_by_ids failed", err)
            })?;

        Ok(button_actions)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<button_action::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            button_action::Entity::default(),
            aq_condition,
            "ui_button_action",
            "ButtonAction",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<button_action::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ButtonAction num_items failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction num_items failed", err)
            })?;
        let button_actions = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ButtonAction fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction fetch_page failed", err)
            })?;
        Ok((button_actions, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<button_action::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            button_action::Entity::default(),
            aq_condition,
            "ui_button_action",
            "ButtonAction",
        )?;

        let button_actions = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction find_collection_by_condition failed", err)
            })?;

        Ok(button_actions)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<button_action::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            button_action::Entity::default(),
            aq_condition,
            "ui_button_action",
            "ButtonAction",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            button_action::Entity::default(),
            aq_condition,
            "ui_button_action",
            "ButtonAction",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            button_action::Entity::default(),
            aq_condition,
            "ui_button_action",
            "ButtonAction",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
