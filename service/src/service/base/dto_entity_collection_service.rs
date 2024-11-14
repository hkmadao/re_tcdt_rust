use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::dto_entity_collection_po::DtoEntityCollectionPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::dto_entity_collection;
use sea_orm::*;

pub struct DtoEntityCollectionMutation;

impl DtoEntityCollectionMutation {
    pub async fn create(
        db: &DbConn,
        dto_entity_collection_po: DtoEntityCollectionPO,
    ) -> Result<dto_entity_collection::Model, TcdtServiceError> {
        let dto_entity_collection_save = DtoEntityCollectionPO::insert(dto_entity_collection_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection insert failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection insert failed", err)
            })?;
        Ok(dto_entity_collection_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_entity_collection_po: DtoEntityCollectionPO,
    ) -> Result<dto_entity_collection::Model, TcdtServiceError> {
        let dto_entity_collection_save = DtoEntityCollectionPO::update(dto_entity_collection_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection update failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection update failed", err)
            })?;
        Ok(dto_entity_collection_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_entity_collection_po: DtoEntityCollectionPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = DtoEntityCollectionPO::delete(dto_entity_collection_po, db, None)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_entity_collection::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEntityCollectionQuery;

impl DtoEntityCollectionQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_entity_collection::Model, TcdtServiceError> {
        let dto_entity_collection_entity =
            dto_entity_collection::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("DtoEntityCollection find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("DtoEntityCollection find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEntityCollection cant not find data"))?;
        Ok(dto_entity_collection_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_entity_collection::Model>, TcdtServiceError> {
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idDtoEntityCollection".to_string(),
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
            dto_entity_collection::Entity::default(),
            aq_conditoin,
            "dto_entity_collection",
            "DtoEntityCollection",
        )?;

        let dto_entity_collections = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection find_by_ids failed", err)
            })?;

        Ok(dto_entity_collections)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_entity_collection::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_entity_collection::Entity::default(),
            aq_conditoin,
            "dto_entity_collection",
            "DtoEntityCollection",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_entity_collection::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection num_items failed", err)
            })?;
        let dto_entity_collections = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection fetch_page failed", err)
            })?;
        Ok((dto_entity_collections, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Vec<dto_entity_collection::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_collection::Entity::default(),
            aq_conditoin,
            "dto_entity_collection",
            "DtoEntityCollection",
        )?;

        let dto_entity_collections = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection find_collection_by_condition failed", err)
            })?;

        Ok(dto_entity_collections)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<Option<dto_entity_collection::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_collection::Entity::default(),
            aq_conditoin,
            "dto_entity_collection",
            "DtoEntityCollection",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_collection::Entity::default(),
            aq_conditoin,
            "dto_entity_collection",
            "DtoEntityCollection",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_collection::Entity::default(),
            aq_conditoin,
            "dto_entity_collection",
            "DtoEntityCollection",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityCollection exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityCollection exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
