use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::component_entity_associate_po::ComponentEntityAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::component_entity_associate;
use sea_orm::*;

pub struct ComponentEntityAssociateMutation;

impl ComponentEntityAssociateMutation {
    pub async fn create(
        db: &DbConn,
        component_entity_associate_po: ComponentEntityAssociatePO,
    ) -> Result<component_entity_associate::Model, TcdtServiceError> {
        let component_entity_associate_save = ComponentEntityAssociatePO::insert(component_entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate insert failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate insert failed", err)
            })?;
        Ok(component_entity_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_entity_associate_po: ComponentEntityAssociatePO,
    ) -> Result<component_entity_associate::Model, TcdtServiceError> {
        let component_entity_associate_save = ComponentEntityAssociatePO::update(component_entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate update failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate update failed", err)
            })?;
        Ok(component_entity_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_entity_associate_po: ComponentEntityAssociatePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ComponentEntityAssociatePO::delete(component_entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_entity_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentEntityAssociateQuery;

impl ComponentEntityAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_entity_associate::Model, TcdtServiceError> {
        let component_entity_associate_entity =
            component_entity_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("ComponentEntityAssociate find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentEntityAssociate cant not find data"))?;
        Ok(component_entity_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_entity_associate::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idComponentEntityAssociate".to_string(),
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
            component_entity_associate::Entity::default(),
            aq_conditoin,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let component_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_by_ids failed", err)
            })?;

        Ok(component_entity_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_entity_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_conditoin,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_entity_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate num_items failed", err)
            })?;
        let component_entity_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate fetch_page failed", err)
            })?;
        Ok((component_entity_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<component_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_conditoin,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let component_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_collection_by_condition failed", err)
            })?;

        Ok(component_entity_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<component_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_conditoin,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_conditoin,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_conditoin,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
