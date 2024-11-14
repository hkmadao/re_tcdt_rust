use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::component_entity_po::ComponentEntityPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::component_entity;
use sea_orm::*;

pub struct ComponentEntityMutation;

impl ComponentEntityMutation {
    pub async fn create(
        db: &DbConn,
        component_entity_po: ComponentEntityPO,
    ) -> Result<component_entity::Model, TcdtServiceError> {
        let component_entity_save = ComponentEntityPO::insert(component_entity_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity insert failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity insert failed", err)
            })?;
        Ok(component_entity_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_entity_po: ComponentEntityPO,
    ) -> Result<component_entity::Model, TcdtServiceError> {
        let component_entity_save = ComponentEntityPO::update(component_entity_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity update failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity update failed", err)
            })?;
        Ok(component_entity_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_entity_po: ComponentEntityPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ComponentEntityPO::delete(component_entity_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_entity::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentEntityQuery;

impl ComponentEntityQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_entity::Model, TcdtServiceError> {
        let component_entity_entity =
            component_entity::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("ComponentEntity find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("ComponentEntity find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentEntity cant not find data"))?;
        Ok(component_entity_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_entity::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idComponentEntity".to_string(),
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
            component_entity::Entity::default(),
            aq_conditoin,
            "dd_component_entity",
            "ComponentEntity",
        )?;

        let component_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity find_by_ids failed", err)
            })?;

        Ok(component_entitys)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_entity::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_entity::Entity::default(),
            aq_conditoin,
            "dd_component_entity",
            "ComponentEntity",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_entity::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentEntity num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity num_items failed", err)
            })?;
        let component_entitys = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity fetch_page failed", err)
            })?;
        Ok((component_entitys, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<component_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity::Entity::default(),
            aq_conditoin,
            "dd_component_entity",
            "ComponentEntity",
        )?;

        let component_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity find_collection_by_condition failed", err)
            })?;

        Ok(component_entitys)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<component_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity::Entity::default(),
            aq_conditoin,
            "dd_component_entity",
            "ComponentEntity",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity::Entity::default(),
            aq_conditoin,
            "dd_component_entity",
            "ComponentEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity::Entity::default(),
            aq_conditoin,
            "dd_component_entity",
            "ComponentEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntity exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntity exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
