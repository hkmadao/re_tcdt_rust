use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dto_enum_po::DtoEnumPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dto_enum;
use sea_orm::*;

pub struct DtoEnumMutation;

impl DtoEnumMutation {
    pub async fn create(
        db: &DbConn,
        dto_enum_po: DtoEnumPO,
    ) -> Result<dto_enum::Model, TcdtServiceError> {
        let dto_enum_save = DtoEnumPO::insert(dto_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEnum insert failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum insert failed", err)
            })?;
        Ok(dto_enum_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_enum_po: DtoEnumPO,
    ) -> Result<dto_enum::Model, TcdtServiceError> {
        let dto_enum_save = DtoEnumPO::update(dto_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEnum update failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum update failed", err)
            })?;
        Ok(dto_enum_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_enum_po: DtoEnumPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DtoEnumPO::delete(dto_enum_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEnum delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_enum::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEnumQuery;

impl DtoEnumQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_enum::Model, TcdtServiceError> {
        let dto_enum_entity =
            dto_enum::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DtoEnum find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DtoEnum find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEnum cant not find data"))?;
        Ok(dto_enum_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_enum::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idDtoEnum".to_string(),
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
            dto_enum::Entity::default(),
            aq_condition,
            "dto_enum",
            "DtoEnum",
        )?;

        let dto_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum find_by_ids failed", err)
            })?;

        Ok(dto_enums)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_enum::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_enum::Entity::default(),
            aq_condition,
            "dto_enum",
            "DtoEnum",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_enum::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEnum num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum num_items failed", err)
            })?;
        let dto_enums = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEnum fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum fetch_page failed", err)
            })?;
        Ok((dto_enums, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum::Entity::default(),
            aq_condition,
            "dto_enum",
            "DtoEnum",
        )?;

        let dto_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum find_collection_by_condition failed", err)
            })?;

        Ok(dto_enums)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum::Entity::default(),
            aq_condition,
            "dto_enum",
            "DtoEnum",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum::Entity::default(),
            aq_condition,
            "dto_enum",
            "DtoEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum::Entity::default(),
            aq_condition,
            "dto_enum",
            "DtoEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
