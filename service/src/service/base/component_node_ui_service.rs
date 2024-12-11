use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::component_node_ui_po::ComponentNodeUiPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::component_node_ui;
use sea_orm::*;

pub struct ComponentNodeUiMutation;

impl ComponentNodeUiMutation {
    pub async fn create(
        db: &DbConn,
        component_node_ui_po: ComponentNodeUiPO,
    ) -> Result<component_node_ui::Model, TcdtServiceError> {
        let component_node_ui_save = ComponentNodeUiPO::insert(component_node_ui_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi insert failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi insert failed", err)
            })?;
        Ok(component_node_ui_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_node_ui_po: ComponentNodeUiPO,
    ) -> Result<component_node_ui::Model, TcdtServiceError> {
        let component_node_ui_save = ComponentNodeUiPO::update(component_node_ui_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi update failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi update failed", err)
            })?;
        Ok(component_node_ui_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_node_ui_po: ComponentNodeUiPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ComponentNodeUiPO::delete(component_node_ui_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_node_ui::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentNodeUiQuery;

impl ComponentNodeUiQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_node_ui::Model, TcdtServiceError> {
        let component_node_ui_entity =
            component_node_ui::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("ComponentNodeUi find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("ComponentNodeUi find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentNodeUi cant not find data"))?;
        Ok(component_node_ui_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_node_ui::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idComponentNodeUi".to_string(),
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
            component_node_ui::Entity::default(),
            aq_condition,
            "dd_component_node_ui",
            "ComponentNodeUi",
        )?;

        let component_node_uis = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi find_by_ids failed", err)
            })?;

        Ok(component_node_uis)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_node_ui::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_node_ui::Entity::default(),
            aq_condition,
            "dd_component_node_ui",
            "ComponentNodeUi",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_node_ui::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi num_items failed", err)
            })?;
        let component_node_uis = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi fetch_page failed", err)
            })?;
        Ok((component_node_uis, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<component_node_ui::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_node_ui::Entity::default(),
            aq_condition,
            "dd_component_node_ui",
            "ComponentNodeUi",
        )?;

        let component_node_uis = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi find_collection_by_condition failed", err)
            })?;

        Ok(component_node_uis)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<component_node_ui::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_node_ui::Entity::default(),
            aq_condition,
            "dd_component_node_ui",
            "ComponentNodeUi",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_node_ui::Entity::default(),
            aq_condition,
            "dd_component_node_ui",
            "ComponentNodeUi",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_node_ui::Entity::default(),
            aq_condition,
            "dd_component_node_ui",
            "ComponentNodeUi",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentNodeUi exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentNodeUi exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
