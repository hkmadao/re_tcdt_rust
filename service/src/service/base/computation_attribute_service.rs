use crate::{
    common::{
        aq::*,
    },
    dto::po::base::computation_attribute_po::ComputationAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::computation_attribute;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ComputationAttributeMutation;

impl ComputationAttributeMutation {
    pub async fn create(
        db: &DbConn,
        computation_attribute_model: computation_attribute::Model,
    ) -> Result<computation_attribute::Model, TcdtServiceError> {
        let mut computation_attribute_active_model = computation_attribute::convert_model_to_active_model(computation_attribute_model);
        let id = generate_id();
        computation_attribute_active_model.id_computation_attribute = Set(id.clone());
        let _ = computation_attribute::Entity::insert(computation_attribute_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComputationAttribute insert failed",
                err,
            )
        })?;

        let computation_attribute_save = computation_attribute::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComputationAttribute insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("ComputationAttribute insert after cannot find entity"))?;
        Ok(computation_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        computation_attribute_model: computation_attribute::Model,
    ) -> Result<computation_attribute::Model, TcdtServiceError> {
        let id = computation_attribute_model.id_computation_attribute.clone();

        let computation_attribute_persist_model: computation_attribute::ActiveModel = computation_attribute::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "ComputationAttribute update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("ComputationAttribute update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut computation_attribute_active_model = computation_attribute::convert_model_to_active_model(computation_attribute_model);

        let computation_attribute_save = computation_attribute_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " ComputationAttribute update failed",
                err,
            )
        })?;

        Ok(computation_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        computation_attribute_model: computation_attribute::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = computation_attribute::Entity::delete(computation_attribute_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        computation_attribute_model_list: Vec<computation_attribute::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = computation_attribute_model_list.iter().map(|computation_attribute_model| {
            computation_attribute_model.id_computation_attribute.clone()
        }).collect::<Vec<String>>();
        let delete_result = computation_attribute::Entity::delete_many()
            .filter(Expr::col(computation_attribute::Column::IdComputationAttribute).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute batch_delete failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = computation_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComputationAttributeQuery;

impl ComputationAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<computation_attribute::Model, TcdtServiceError> {
        let computation_attribute_entity =
            computation_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("ComputationAttribute find_by_id failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("ComputationAttribute cant not find data"))?;
        Ok(computation_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<computation_attribute::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idComputationAttribute", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            computation_attribute::Entity::default(),
            aq_condition,
            "dd_computation_attribute",
            "ComputationAttribute",
        )?;

        let computation_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute find_by_ids failed", err)
            })?;

        Ok(computation_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<computation_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            computation_attribute::Entity::default(),
            aq_condition,
            "dd_computation_attribute",
            "ComputationAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<computation_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute num_items failed", err)
            })?;
        let computation_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute fetch_page failed", err)
            })?;
        Ok((computation_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<computation_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            computation_attribute::Entity::default(),
            aq_condition,
            "dd_computation_attribute",
            "ComputationAttribute",
        )?;

        let computation_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute find_collection_by_condition failed", err)
            })?;

        Ok(computation_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<computation_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            computation_attribute::Entity::default(),
            aq_condition,
            "dd_computation_attribute",
            "ComputationAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            computation_attribute::Entity::default(),
            aq_condition,
            "dd_computation_attribute",
            "ComputationAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            computation_attribute::Entity::default(),
            aq_condition,
            "dd_computation_attribute",
            "ComputationAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComputationAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComputationAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
