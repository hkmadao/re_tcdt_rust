use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dto_enum_associate_po::DtoEnumAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dto_enum_associate;
use sea_orm::*;

pub struct DtoEnumAssociateMutation;

impl DtoEnumAssociateMutation {
    pub async fn create(
        db: &DbConn,
        dto_enum_associate_po: DtoEnumAssociatePO,
    ) -> Result<dto_enum_associate::Model, TcdtServiceError> {
        let dto_enum_associate_save = DtoEnumAssociatePO::insert(dto_enum_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate insert failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate insert failed", err)
            })?;
        Ok(dto_enum_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_enum_associate_po: DtoEnumAssociatePO,
    ) -> Result<dto_enum_associate::Model, TcdtServiceError> {
        let dto_enum_associate_save = DtoEnumAssociatePO::update(dto_enum_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate update failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate update failed", err)
            })?;
        Ok(dto_enum_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_enum_associate_po: DtoEnumAssociatePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DtoEnumAssociatePO::delete(dto_enum_associate_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_enum_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEnumAssociateQuery;

impl DtoEnumAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_enum_associate::Model, TcdtServiceError> {
        let dto_enum_associate_entity =
            dto_enum_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DtoEnumAssociate find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DtoEnumAssociate find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEnumAssociate cant not find data"))?;
        Ok(dto_enum_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_enum_associate::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idDtoEnumAssociate".to_string(),
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
            dto_enum_associate::Entity::default(),
            aq_condition,
            "dto_enum_associate",
            "DtoEnumAssociate",
        )?;

        let dto_enum_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate find_by_ids failed", err)
            })?;

        Ok(dto_enum_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_enum_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_enum_associate::Entity::default(),
            aq_condition,
            "dto_enum_associate",
            "DtoEnumAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_enum_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate num_items failed", err)
            })?;
        let dto_enum_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate fetch_page failed", err)
            })?;
        Ok((dto_enum_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_enum_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_associate::Entity::default(),
            aq_condition,
            "dto_enum_associate",
            "DtoEnumAssociate",
        )?;

        let dto_enum_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate find_collection_by_condition failed", err)
            })?;

        Ok(dto_enum_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_enum_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_associate::Entity::default(),
            aq_condition,
            "dto_enum_associate",
            "DtoEnumAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_associate::Entity::default(),
            aq_condition,
            "dto_enum_associate",
            "DtoEnumAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_associate::Entity::default(),
            aq_condition,
            "dto_enum_associate",
            "DtoEnumAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
