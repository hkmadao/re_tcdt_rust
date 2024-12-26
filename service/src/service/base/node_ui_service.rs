use crate::{
    common::{
        aq::*,
    },
    dto::po::base::node_ui_po::NodeUiPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::node_ui;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct NodeUiMutation;

impl NodeUiMutation {
    pub async fn create(
        db: &DbConn,
        node_ui_model: node_ui::Model,
    ) -> Result<node_ui::Model, TcdtServiceError> {
        let mut node_ui_active_model = node_ui::convert_model_to_active_model(node_ui_model);
        let id = generate_id();
        node_ui_active_model.id_node_ui = Set(id.clone());
        let _ = node_ui::Entity::insert(node_ui_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "NodeUi insert failed",
                err,
            )
        })?;

        let node_ui_save = node_ui::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "NodeUi insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("NodeUi insert after cannot find entity"))?;
        Ok(node_ui_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        node_ui_model: node_ui::Model,
    ) -> Result<node_ui::Model, TcdtServiceError> {
        let id = node_ui_model.id_node_ui.clone();

        let node_ui_persist_model: node_ui::ActiveModel = node_ui::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "NodeUi update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("NodeUi update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut node_ui_active_model = node_ui::convert_model_to_active_model(node_ui_model);

        let node_ui_save = node_ui_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " NodeUi update failed",
                err,
            )
        })?;

        Ok(node_ui_save)
    }

    pub async fn delete(
        db: &DbConn,
        node_ui_model: node_ui::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = node_ui::Entity::delete(node_ui_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi delete failed");
                TcdtServiceError::build_internal_msg_error("NodeUi delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        node_ui_model_list: Vec<node_ui::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = node_ui_model_list.iter().map(|node_ui_model| {
            node_ui_model.id_node_ui.clone()
        }).collect::<Vec<String>>();
        let delete_result = node_ui::Entity::delete_many()
            .filter(Expr::col(node_ui::Column::IdNodeUi).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi batch_delete failed");
                TcdtServiceError::build_internal_msg_error("NodeUi batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = node_ui::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi delete_all failed");
                TcdtServiceError::build_internal_msg_error("NodeUi delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct NodeUiQuery;

impl NodeUiQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<node_ui::Model, TcdtServiceError> {
        let node_ui_entity =
            node_ui::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("NodeUi find_by_id failed");
                TcdtServiceError::build_internal_msg_error("NodeUi find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("NodeUi cant not find data"))?;
        Ok(node_ui_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<node_ui::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idNodeUi", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            node_ui::Entity::default(),
            aq_condition,
            "dd_node_ui",
            "NodeUi",
        )?;

        let node_uis = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("NodeUi find_by_ids failed", err)
            })?;

        Ok(node_uis)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<node_ui::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            node_ui::Entity::default(),
            aq_condition,
            "dd_node_ui",
            "NodeUi",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<node_ui::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("NodeUi num_items failed");
                TcdtServiceError::build_internal_msg_error("NodeUi num_items failed", err)
            })?;
        let node_uis = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("NodeUi fetch_page failed");
                TcdtServiceError::build_internal_msg_error("NodeUi fetch_page failed", err)
            })?;
        Ok((node_uis, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<node_ui::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            node_ui::Entity::default(),
            aq_condition,
            "dd_node_ui",
            "NodeUi",
        )?;

        let node_uis = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("NodeUi find_collection_by_condition failed", err)
            })?;

        Ok(node_uis)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<node_ui::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            node_ui::Entity::default(),
            aq_condition,
            "dd_node_ui",
            "NodeUi",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("NodeUi find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            node_ui::Entity::default(),
            aq_condition,
            "dd_node_ui",
            "NodeUi",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("NodeUi count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            node_ui::Entity::default(),
            aq_condition,
            "dd_node_ui",
            "NodeUi",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("NodeUi exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("NodeUi exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
