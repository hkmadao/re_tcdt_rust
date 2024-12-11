use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::enum_associate_po::EnumAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::enum_associate;
use sea_orm::*;

pub struct EnumAssociateMutation;

impl EnumAssociateMutation {
    pub async fn create(
        db: &DbConn,
        enum_associate_po: EnumAssociatePO,
    ) -> Result<enum_associate::Model, TcdtServiceError> {
        let enum_associate_save = EnumAssociatePO::insert(enum_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate insert failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate insert failed", err)
            })?;
        Ok(enum_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        enum_associate_po: EnumAssociatePO,
    ) -> Result<enum_associate::Model, TcdtServiceError> {
        let enum_associate_save = EnumAssociatePO::update(enum_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate update failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate update failed", err)
            })?;
        Ok(enum_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        enum_associate_po: EnumAssociatePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = EnumAssociatePO::delete(enum_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = enum_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct EnumAssociateQuery;

impl EnumAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<enum_associate::Model, TcdtServiceError> {
        let enum_associate_entity =
            enum_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("EnumAssociate find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("EnumAssociate find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("EnumAssociate cant not find data"))?;
        Ok(enum_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<enum_associate::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idEnumAssociate".to_string(),
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
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let enum_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_by_ids failed", err)
            })?;

        Ok(enum_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<enum_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<enum_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("EnumAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate num_items failed", err)
            })?;
        let enum_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate fetch_page failed", err)
            })?;
        Ok((enum_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<enum_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let enum_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_collection_by_condition failed", err)
            })?;

        Ok(enum_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<enum_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
