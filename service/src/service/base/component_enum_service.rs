use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::component_enum_po::ComponentEnumPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::component_enum;
use sea_orm::*;

pub struct ComponentEnumMutation;

impl ComponentEnumMutation {
    pub async fn create(
        db: &DbConn,
        component_enum_po: ComponentEnumPO,
    ) -> Result<component_enum::Model, TcdtServiceError> {
        let component_enum_save = ComponentEnumPO::insert(component_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum insert failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum insert failed", err)
            })?;
        Ok(component_enum_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_enum_po: ComponentEnumPO,
    ) -> Result<component_enum::Model, TcdtServiceError> {
        let component_enum_save = ComponentEnumPO::update(component_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum update failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum update failed", err)
            })?;
        Ok(component_enum_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_enum_po: ComponentEnumPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ComponentEnumPO::delete(component_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_enum::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentEnumQuery;

impl ComponentEnumQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_enum::Model, TcdtServiceError> {
        let component_enum_entity =
            component_enum::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("ComponentEnum find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("ComponentEnum find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentEnum cant not find data"))?;
        Ok(component_enum_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_enum::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idComponentEnum".to_string(),
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
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let component_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_by_ids failed", err)
            })?;

        Ok(component_enums)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_enum::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_enum::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentEnum num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum num_items failed", err)
            })?;
        let component_enums = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum fetch_page failed", err)
            })?;
        Ok((component_enums, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<component_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let component_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_collection_by_condition failed", err)
            })?;

        Ok(component_enums)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<component_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
