use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::ext_attribute_po::ExtAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::ext_attribute;
use sea_orm::*;

pub struct ExtAttributeMutation;

impl ExtAttributeMutation {
    pub async fn create(
        db: &DbConn,
        ext_attribute_po: ExtAttributePO,
    ) -> Result<ext_attribute::Model, TcdtServiceError> {
        let ext_attribute_save = ExtAttributePO::insert(ext_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute insert failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute insert failed", err)
            })?;
        Ok(ext_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        ext_attribute_po: ExtAttributePO,
    ) -> Result<ext_attribute::Model, TcdtServiceError> {
        let ext_attribute_save = ExtAttributePO::update(ext_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute update failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute update failed", err)
            })?;
        Ok(ext_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        ext_attribute_po: ExtAttributePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ExtAttributePO::delete(ext_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = ext_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ExtAttributeQuery;

impl ExtAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<ext_attribute::Model, TcdtServiceError> {
        let ext_attribute_entity =
            ext_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("ExtAttribute find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("ExtAttribute find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("ExtAttribute cant not find data"))?;
        Ok(ext_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<ext_attribute::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idExtAttribute".to_string(),
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
            ext_attribute::Entity::default(),
            aq_conditoin,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let ext_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_by_ids failed", err)
            })?;

        Ok(ext_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<ext_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_conditoin,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<ext_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ExtAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute num_items failed", err)
            })?;
        let ext_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute fetch_page failed", err)
            })?;
        Ok((ext_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<ext_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_conditoin,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let ext_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_collection_by_condition failed", err)
            })?;

        Ok(ext_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<ext_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_conditoin,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_conditoin,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_conditoin,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
