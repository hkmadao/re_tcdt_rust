use crate::{
    common::{
        aq::*,
    },
    dto::po::base::button_action_po::ButtonActionPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::button_action;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ButtonActionMutation;

impl ButtonActionMutation {
    pub async fn create(
        db: &DbConn,
        button_action_model: button_action::Model,
    ) -> Result<button_action::Model, TcdtServiceError> {
        let mut button_action_active_model = button_action::convert_model_to_active_model(button_action_model);
        let id = generate_id();
        button_action_active_model.id_button_action = Set(id.clone());
        let _ = button_action::Entity::insert(button_action_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ButtonAction insert failed",
                err,
            )
        })?;

        let button_action_save = button_action::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ButtonAction insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("ButtonAction insert after cannot find entity"))?;
        Ok(button_action_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        button_action_model: button_action::Model,
    ) -> Result<button_action::Model, TcdtServiceError> {
        let id = button_action_model.id_button_action.clone();

        let button_action_persist_model: button_action::ActiveModel = button_action::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "ButtonAction update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("ButtonAction update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut button_action_active_model = button_action::convert_model_to_active_model(button_action_model);

        let button_action_save = button_action_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " ButtonAction update failed",
                err,
            )
        })?;

        Ok(button_action_save)
    }

    pub async fn delete(
        db: &DbConn,
        button_action_model: button_action::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = button_action::Entity::delete(button_action_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction delete failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        button_action_model_list: Vec<button_action::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = button_action_model_list.iter().map(|button_action_model| {
            button_action_model.id_button_action.clone()
        }).collect::<Vec<String>>();
        let delete_result = button_action::Entity::delete_many()
            .filter(Expr::col(button_action::Column::IdButtonAction).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ButtonAction batch_delete failed");
                TcdtServiceError::build_internal_msg_error("ButtonAction batch_delete failed", err)
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
        let aq_condition = AqCondition::build_in_condition("idButtonAction", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

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
