use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::common_attribute_po::CommonAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::common_attribute;
use sea_orm::*;

pub struct CommonAttributeMutation;

impl CommonAttributeMutation {
    pub async fn create(
        db: &DbConn,
        common_attribute_po: CommonAttributePO,
    ) -> Result<common_attribute::Model, TcdtServiceError> {
        let common_attribute_save = CommonAttributePO::insert(common_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute insert failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute insert failed", err)
            })?;
        Ok(common_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        common_attribute_po: CommonAttributePO,
    ) -> Result<common_attribute::Model, TcdtServiceError> {
        let common_attribute_save = CommonAttributePO::update(common_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute update failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute update failed", err)
            })?;
        Ok(common_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        common_attribute_po: CommonAttributePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = CommonAttributePO::delete(common_attribute_po, db, None)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        data_type_po_list: Vec<CommonAttributePO>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = data_type_po_list
            .iter()
            .map(|po| po.id_data_type.clone())
            .collect::<Vec<_>>();
        let delete_result = common_attribute::Entity::delete_many()
            .filter(common_attribute::Column::IdCommonAttribute.is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute batch_delete failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = common_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct CommonAttributeQuery;

impl CommonAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<common_attribute::Model, TcdtServiceError> {
        let common_attribute_entity =
            common_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("CommonAttribute find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("CommonAttribute find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("CommonAttribute cant not find data"))?;
        Ok(common_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<common_attribute::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idCommonAttribute".to_string(),
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
            common_attribute::Entity::default(),
            aq_conditoin,
            "dd_common_attribute",
            "CommonAttribute",
        )?;

        let common_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute find_by_ids failed", err)
            })?;

        Ok(common_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<common_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            common_attribute::Entity::default(),
            aq_conditoin,
            "dd_common_attribute",
            "CommonAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<common_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("CommonAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute num_items failed", err)
            })?;
        let common_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute fetch_page failed", err)
            })?;
        Ok((common_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<common_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            common_attribute::Entity::default(),
            aq_conditoin,
            "dd_common_attribute",
            "CommonAttribute",
        )?;

        let common_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute find_collection_by_condition failed", err)
            })?;

        Ok(common_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<common_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            common_attribute::Entity::default(),
            aq_conditoin,
            "dd_common_attribute",
            "CommonAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            common_attribute::Entity::default(),
            aq_conditoin,
            "dd_common_attribute",
            "CommonAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            common_attribute::Entity::default(),
            aq_conditoin,
            "dd_common_attribute",
            "CommonAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("CommonAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("CommonAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
