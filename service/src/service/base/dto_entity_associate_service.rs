use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dto_entity_associate_po::DtoEntityAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dto_entity_associate;
use sea_orm::*;

pub struct DtoEntityAssociateMutation;

impl DtoEntityAssociateMutation {
    pub async fn create(
        db: &DbConn,
        dto_entity_associate_po: DtoEntityAssociatePO,
    ) -> Result<dto_entity_associate::Model, TcdtServiceError> {
        let dto_entity_associate_save = DtoEntityAssociatePO::insert(dto_entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate insert failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate insert failed", err)
            })?;
        Ok(dto_entity_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_entity_associate_po: DtoEntityAssociatePO,
    ) -> Result<dto_entity_associate::Model, TcdtServiceError> {
        let dto_entity_associate_save = DtoEntityAssociatePO::update(dto_entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate update failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate update failed", err)
            })?;
        Ok(dto_entity_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_entity_associate_po: DtoEntityAssociatePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DtoEntityAssociatePO::delete(dto_entity_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_entity_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEntityAssociateQuery;

impl DtoEntityAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_entity_associate::Model, TcdtServiceError> {
        let dto_entity_associate_entity =
            dto_entity_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DtoEntityAssociate find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEntityAssociate cant not find data"))?;
        Ok(dto_entity_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_entity_associate::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idDtoEntityAssociate".to_string(),
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
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let dto_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_by_ids failed", err)
            })?;

        Ok(dto_entity_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_entity_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_entity_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate num_items failed", err)
            })?;
        let dto_entity_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate fetch_page failed", err)
            })?;
        Ok((dto_entity_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let dto_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_collection_by_condition failed", err)
            })?;

        Ok(dto_entity_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
