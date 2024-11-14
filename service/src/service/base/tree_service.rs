use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::tree_po::TreePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::tree;
use sea_orm::*;

pub struct TreeMutation;

impl TreeMutation {
    pub async fn create(
        db: &DbConn,
        tree_po: TreePO,
    ) -> Result<tree::Model, TcdtServiceError> {
        let tree_save = TreePO::insert(tree_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Tree insert failed");
                TcdtServiceError::build_internal_msg_error("Tree insert failed", err)
            })?;
        Ok(tree_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        tree_po: TreePO,
    ) -> Result<tree::Model, TcdtServiceError> {
        let tree_save = TreePO::update(tree_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Tree update failed");
                TcdtServiceError::build_internal_msg_error("Tree update failed", err)
            })?;
        Ok(tree_save)
    }

    pub async fn delete(
        db: &DbConn,
        tree_po: TreePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = TreePO::delete(tree_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Tree delete failed");
                TcdtServiceError::build_internal_msg_error("Tree delete failed", err)
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
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idTree".to_string(),
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
            tree::Entity::default(),
            aq_conditoin,
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
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<Vec<tree::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<Option<tree::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            tree::Entity::default(),
            aq_conditoin,
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
