use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dto_module_po::DtoModulePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dto_module;
use sea_orm::*;

pub struct DtoModuleMutation;

impl DtoModuleMutation {
    pub async fn create(
        db: &DbConn,
        dto_module_po: DtoModulePO,
    ) -> Result<dto_module::Model, TcdtServiceError> {
        let dto_module_save = DtoModulePO::insert(dto_module_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoModule insert failed");
                TcdtServiceError::build_internal_msg_error("DtoModule insert failed", err)
            })?;
        Ok(dto_module_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_module_po: DtoModulePO,
    ) -> Result<dto_module::Model, TcdtServiceError> {
        let dto_module_save = DtoModulePO::update(dto_module_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoModule update failed");
                TcdtServiceError::build_internal_msg_error("DtoModule update failed", err)
            })?;
        Ok(dto_module_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_module_po: DtoModulePO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DtoModulePO::delete(dto_module_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoModule delete failed");
                TcdtServiceError::build_internal_msg_error("DtoModule delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_module::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoModule delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoModule delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoModuleQuery;

impl DtoModuleQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_module::Model, TcdtServiceError> {
        let dto_module_entity =
            dto_module::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DtoModule find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DtoModule find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoModule cant not find data"))?;
        Ok(dto_module_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_module::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idDtoModule".to_string(),
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
            dto_module::Entity::default(),
            aq_conditoin,
            "dto_module",
            "DtoModule",
        )?;

        let dto_modules = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoModule find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoModule find_by_ids failed", err)
            })?;

        Ok(dto_modules)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_module::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_module::Entity::default(),
            aq_conditoin,
            "dto_module",
            "DtoModule",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_module::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoModule num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoModule num_items failed", err)
            })?;
        let dto_modules = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoModule fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoModule fetch_page failed", err)
            })?;
        Ok((dto_modules, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<dto_module::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_module::Entity::default(),
            aq_conditoin,
            "dto_module",
            "DtoModule",
        )?;

        let dto_modules = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoModule find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoModule find_collection_by_condition failed", err)
            })?;

        Ok(dto_modules)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<dto_module::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_module::Entity::default(),
            aq_conditoin,
            "dto_module",
            "DtoModule",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoModule find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoModule find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_module::Entity::default(),
            aq_conditoin,
            "dto_module",
            "DtoModule",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoModule count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoModule count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_module::Entity::default(),
            aq_conditoin,
            "dto_module",
            "DtoModule",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoModule exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoModule exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
