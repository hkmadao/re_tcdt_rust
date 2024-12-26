use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dto_computation_attribute_po::DtoComputationAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dto_computation_attribute;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DtoComputationAttributeMutation;

impl DtoComputationAttributeMutation {
    pub async fn create(
        db: &DbConn,
        dto_computation_attribute_model: dto_computation_attribute::Model,
    ) -> Result<dto_computation_attribute::Model, TcdtServiceError> {
        let mut dto_computation_attribute_active_model = dto_computation_attribute::convert_model_to_active_model(dto_computation_attribute_model);
        let id = generate_id();
        dto_computation_attribute_active_model.id_dto_computation_attribute = Set(id.clone());
        let _ = dto_computation_attribute::Entity::insert(dto_computation_attribute_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoComputationAttribute insert failed",
                err,
            )
        })?;

        let dto_computation_attribute_save = dto_computation_attribute::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoComputationAttribute insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DtoComputationAttribute insert after cannot find entity"))?;
        Ok(dto_computation_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_computation_attribute_model: dto_computation_attribute::Model,
    ) -> Result<dto_computation_attribute::Model, TcdtServiceError> {
        let id = dto_computation_attribute_model.id_dto_computation_attribute.clone();

        let dto_computation_attribute_persist_model: dto_computation_attribute::ActiveModel = dto_computation_attribute::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DtoComputationAttribute update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DtoComputationAttribute update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dto_computation_attribute_active_model = dto_computation_attribute::convert_model_to_active_model(dto_computation_attribute_model);

        let dto_computation_attribute_save = dto_computation_attribute_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DtoComputationAttribute update failed",
                err,
            )
        })?;

        Ok(dto_computation_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_computation_attribute_model: dto_computation_attribute::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dto_computation_attribute::Entity::delete(dto_computation_attribute_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dto_computation_attribute_model_list: Vec<dto_computation_attribute::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dto_computation_attribute_model_list.iter().map(|dto_computation_attribute_model| {
            dto_computation_attribute_model.id_dto_computation_attribute.clone()
        }).collect::<Vec<String>>();
        let delete_result = dto_computation_attribute::Entity::delete_many()
            .filter(Expr::col(dto_computation_attribute::Column::IdDtoComputationAttribute).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_computation_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoComputationAttributeQuery;

impl DtoComputationAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_computation_attribute::Model, TcdtServiceError> {
        let dto_computation_attribute_entity =
            dto_computation_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DtoComputationAttribute find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoComputationAttribute cant not find data"))?;
        Ok(dto_computation_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_computation_attribute::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idDtoComputationAttribute", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dto_computation_attribute::Entity::default(),
            aq_condition,
            "dto_computation_attribute",
            "DtoComputationAttribute",
        )?;

        let dto_computation_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute find_by_ids failed", err)
            })?;

        Ok(dto_computation_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_computation_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_computation_attribute::Entity::default(),
            aq_condition,
            "dto_computation_attribute",
            "DtoComputationAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_computation_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute num_items failed", err)
            })?;
        let dto_computation_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute fetch_page failed", err)
            })?;
        Ok((dto_computation_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_computation_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_computation_attribute::Entity::default(),
            aq_condition,
            "dto_computation_attribute",
            "DtoComputationAttribute",
        )?;

        let dto_computation_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute find_collection_by_condition failed", err)
            })?;

        Ok(dto_computation_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_computation_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_computation_attribute::Entity::default(),
            aq_condition,
            "dto_computation_attribute",
            "DtoComputationAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_computation_attribute::Entity::default(),
            aq_condition,
            "dto_computation_attribute",
            "DtoComputationAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_computation_attribute::Entity::default(),
            aq_condition,
            "dto_computation_attribute",
            "DtoComputationAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoComputationAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoComputationAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
