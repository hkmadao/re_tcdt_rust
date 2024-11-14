use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::bill_form_po::BillFormPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::bill_form;
use sea_orm::*;

pub struct BillFormMutation;

impl BillFormMutation {
    pub async fn create(
        db: &DbConn,
        bill_form_po: BillFormPO,
    ) -> Result<bill_form::Model, TcdtServiceError> {
        let bill_form_save = BillFormPO::insert(bill_form_po, db, None)
            .await
            .map_err(|err| {
                log::error!("BillForm insert failed");
                TcdtServiceError::build_internal_msg_error("BillForm insert failed", err)
            })?;
        Ok(bill_form_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        bill_form_po: BillFormPO,
    ) -> Result<bill_form::Model, TcdtServiceError> {
        let bill_form_save = BillFormPO::update(bill_form_po, db, None)
            .await
            .map_err(|err| {
                log::error!("BillForm update failed");
                TcdtServiceError::build_internal_msg_error("BillForm update failed", err)
            })?;
        Ok(bill_form_save)
    }

    pub async fn delete(
        db: &DbConn,
        bill_form_po: BillFormPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = BillFormPO::delete(bill_form_po, db, None)
            .await
            .map_err(|err| {
                log::error!("BillForm delete failed");
                TcdtServiceError::build_internal_msg_error("BillForm delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = bill_form::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("BillForm delete_all failed");
                TcdtServiceError::build_internal_msg_error("BillForm delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct BillFormQuery;

impl BillFormQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<bill_form::Model, TcdtServiceError> {
        let bill_form_entity =
            bill_form::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("BillForm find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("BillForm find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("BillForm cant not find data"))?;
        Ok(bill_form_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<bill_form::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idBillForm".to_string(),
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
            bill_form::Entity::default(),
            aq_conditoin,
            "ui_bill_form",
            "BillForm",
        )?;

        let bill_forms = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("BillForm find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("BillForm find_by_ids failed", err)
            })?;

        Ok(bill_forms)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<bill_form::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            bill_form::Entity::default(),
            aq_conditoin,
            "ui_bill_form",
            "BillForm",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<bill_form::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("BillForm num_items failed");
                TcdtServiceError::build_internal_msg_error("BillForm num_items failed", err)
            })?;
        let bill_forms = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("BillForm fetch_page failed");
                TcdtServiceError::build_internal_msg_error("BillForm fetch_page failed", err)
            })?;
        Ok((bill_forms, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<bill_form::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            bill_form::Entity::default(),
            aq_conditoin,
            "ui_bill_form",
            "BillForm",
        )?;

        let bill_forms = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("BillForm find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("BillForm find_collection_by_condition failed", err)
            })?;

        Ok(bill_forms)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<bill_form::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            bill_form::Entity::default(),
            aq_conditoin,
            "ui_bill_form",
            "BillForm",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("BillForm find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("BillForm find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            bill_form::Entity::default(),
            aq_conditoin,
            "ui_bill_form",
            "BillForm",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("BillForm count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("BillForm count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            bill_form::Entity::default(),
            aq_conditoin,
            "ui_bill_form",
            "BillForm",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("BillForm exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("BillForm exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}