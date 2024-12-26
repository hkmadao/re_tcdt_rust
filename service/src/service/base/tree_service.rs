use crate::{
    common::{
        aq::*,
    },
    dto::po::base::tree_po::TreePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::tree;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct TreeMutation;

impl TreeMutation {
    pub async fn create(
        db: &DbConn,
        tree_model: tree::Model,
    ) -> Result<tree::Model, TcdtServiceError> {
        let mut tree_active_model = tree::convert_model_to_active_model(tree_model);
        let id = generate_id();
        tree_active_model.id_tree = Set(id.clone());
        let _ = tree::Entity::insert(tree_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Tree insert failed",
                err,
            )
        })?;

        let tree_save = tree::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Tree insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Tree insert after cannot find entity"))?;
        Ok(tree_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        tree_model: tree::Model,
    ) -> Result<tree::Model, TcdtServiceError> {
        let id = tree_model.id_tree.clone();

        let tree_persist_model: tree::ActiveModel = tree::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Tree update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Tree update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut tree_active_model = tree::convert_model_to_active_model(tree_model);

        let tree_save = tree_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Tree update failed",
                err,
            )
        })?;

        Ok(tree_save)
    }

    pub async fn delete(
        db: &DbConn,
        tree_model: tree::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = tree::Entity::delete(tree_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Tree delete failed");
                TcdtServiceError::build_internal_msg_error("Tree delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        tree_model_list: Vec<tree::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = tree_model_list.iter().map(|tree_model| {
            tree_model.id_tree.clone()
        }).collect::<Vec<String>>();
        let delete_result = tree::Entity::delete_many()
            .filter(Expr::col(tree::Column::IdTree).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Tree batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Tree batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = tree::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Tree delete_all failed");
                TcdtServiceError::build_internal_msg_error("Tree delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct TreeQuery;

impl TreeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<tree::Model, TcdtServiceError> {
        let tree_entity =
            tree::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("Tree find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Tree find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("Tree cant not find data"))?;
        Ok(tree_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<tree::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idTree", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_condition,
            "ui_tree",
            "Tree",
        )?;

        let trees = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Tree find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Tree find_by_ids failed", err)
            })?;

        Ok(trees)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<tree::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_condition,
            "ui_tree",
            "Tree",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<tree::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Tree num_items failed");
                TcdtServiceError::build_internal_msg_error("Tree num_items failed", err)
            })?;
        let trees = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Tree fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Tree fetch_page failed", err)
            })?;
        Ok((trees, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<tree::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_condition,
            "ui_tree",
            "Tree",
        )?;

        let trees = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Tree find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Tree find_collection_by_condition failed", err)
            })?;

        Ok(trees)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<tree::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_condition,
            "ui_tree",
            "Tree",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Tree find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Tree find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_condition,
            "ui_tree",
            "Tree",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Tree count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Tree count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_condition,
            "ui_tree",
            "Tree",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Tree exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Tree exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
