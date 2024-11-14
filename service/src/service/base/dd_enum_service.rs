use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dd_enum_po::DdEnumPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dd_enum;
use sea_orm::*;

pub struct DdEnumMutation;

impl DdEnumMutation {
    pub async fn create(
        db: &DbConn,
        dd_enum_po: DdEnumPO,
    ) -> Result<dd_enum::Model, TcdtServiceError> {
        let dd_enum_save = DdEnumPO::insert(dd_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DdEnum insert failed");
                TcdtServiceError::build_internal_msg_error("DdEnum insert failed", err)
            })?;
        Ok(dd_enum_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dd_enum_po: DdEnumPO,
    ) -> Result<dd_enum::Model, TcdtServiceError> {
        let dd_enum_save = DdEnumPO::update(dd_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DdEnum update failed");
                TcdtServiceError::build_internal_msg_error("DdEnum update failed", err)
            })?;
        Ok(dd_enum_save)
    }

    pub async fn delete(
        db: &DbConn,
        dd_enum_po: DdEnumPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DdEnumPO::delete(dd_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DdEnum delete failed");
                TcdtServiceError::build_internal_msg_error("DdEnum delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dd_enum::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum delete_all failed");
                TcdtServiceError::build_internal_msg_error("DdEnum delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DdEnumQuery;

impl DdEnumQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dd_enum::Model, TcdtServiceError> {
        let dd_enum_entity =
            dd_enum::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DdEnum find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DdEnum find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DdEnum cant not find data"))?;
        Ok(dd_enum_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dd_enum::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idEnum".to_string(),
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
            dd_enum::Entity::default(),
            aq_conditoin,
            "dd_enum",
            "DdEnum",
        )?;

        let dd_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_by_ids failed", err)
            })?;

        Ok(dd_enums)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dd_enum::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_conditoin,
            "dd_enum",
            "DdEnum",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dd_enum::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DdEnum num_items failed");
                TcdtServiceError::build_internal_msg_error("DdEnum num_items failed", err)
            })?;
        let dd_enums = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DdEnum fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DdEnum fetch_page failed", err)
            })?;
        Ok((dd_enums, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<dd_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_conditoin,
            "dd_enum",
            "DdEnum",
        )?;

        let dd_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_collection_by_condition failed", err)
            })?;

        Ok(dd_enums)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<dd_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_conditoin,
            "dd_enum",
            "DdEnum",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_conditoin,
            "dd_enum",
            "DdEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_conditoin,
            "dd_enum",
            "DdEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}