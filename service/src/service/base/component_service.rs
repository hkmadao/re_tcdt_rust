use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::component_po::ComponentPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::component;
use sea_orm::*;

pub struct ComponentMutation;

impl ComponentMutation {
    pub async fn create(
        db: &DbConn,
        component_po: ComponentPO,
    ) -> Result<component::Model, TcdtServiceError> {
        let component_save = ComponentPO::insert(component_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Component insert failed");
                TcdtServiceError::build_internal_msg_error("Component insert failed", err)
            })?;
        Ok(component_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_po: ComponentPO,
    ) -> Result<component::Model, TcdtServiceError> {
        let component_save = ComponentPO::update(component_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Component update failed");
                TcdtServiceError::build_internal_msg_error("Component update failed", err)
            })?;
        Ok(component_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_po: ComponentPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ComponentPO::delete(component_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Component delete failed");
                TcdtServiceError::build_internal_msg_error("Component delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Component delete_all failed");
                TcdtServiceError::build_internal_msg_error("Component delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentQuery;

impl ComponentQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component::Model, TcdtServiceError> {
        let component_entity =
            component::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("Component find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("Component find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("Component cant not find data"))?;
        Ok(component_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idComponent".to_string(),
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
            component::Entity::default(),
            aq_conditoin,
            "dd_component",
            "Component",
        )?;

        let components = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Component find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Component find_by_ids failed", err)
            })?;

        Ok(components)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component::Entity::default(),
            aq_conditoin,
            "dd_component",
            "Component",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Component num_items failed");
                TcdtServiceError::build_internal_msg_error("Component num_items failed", err)
            })?;
        let components = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Component fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Component fetch_page failed", err)
            })?;
        Ok((components, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<component::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component::Entity::default(),
            aq_conditoin,
            "dd_component",
            "Component",
        )?;

        let components = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Component find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Component find_collection_by_condition failed", err)
            })?;

        Ok(components)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<component::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component::Entity::default(),
            aq_conditoin,
            "dd_component",
            "Component",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Component find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Component find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component::Entity::default(),
            aq_conditoin,
            "dd_component",
            "Component",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Component count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Component count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component::Entity::default(),
            aq_conditoin,
            "dd_component",
            "Component",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Component exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Component exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
