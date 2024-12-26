use crate::{
    common::{
        aq::*,
    },
    dto::po::base::ext_attribute_po::ExtAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::ext_attribute;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ExtAttributeMutation;

impl ExtAttributeMutation {
    pub async fn create(
        db: &DbConn,
        ext_attribute_model: ext_attribute::Model,
    ) -> Result<ext_attribute::Model, TcdtServiceError> {
        let mut ext_attribute_active_model = ext_attribute::convert_model_to_active_model(ext_attribute_model);
        let id = generate_id();
        ext_attribute_active_model.id_ext_attribute = Set(id.clone());
        let _ = ext_attribute::Entity::insert(ext_attribute_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ExtAttribute insert failed",
                err,
            )
        })?;

        let ext_attribute_save = ext_attribute::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ExtAttribute insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("ExtAttribute insert after cannot find entity"))?;
        Ok(ext_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        ext_attribute_model: ext_attribute::Model,
    ) -> Result<ext_attribute::Model, TcdtServiceError> {
        let id = ext_attribute_model.id_ext_attribute.clone();

        let ext_attribute_persist_model: ext_attribute::ActiveModel = ext_attribute::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "ExtAttribute update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("ExtAttribute update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut ext_attribute_active_model = ext_attribute::convert_model_to_active_model(ext_attribute_model);

        let ext_attribute_save = ext_attribute_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " ExtAttribute update failed",
                err,
            )
        })?;

        Ok(ext_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        ext_attribute_model: ext_attribute::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = ext_attribute::Entity::delete(ext_attribute_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        ext_attribute_model_list: Vec<ext_attribute::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = ext_attribute_model_list.iter().map(|ext_attribute_model| {
            ext_attribute_model.id_ext_attribute.clone()
        }).collect::<Vec<String>>();
        let delete_result = ext_attribute::Entity::delete_many()
            .filter(Expr::col(ext_attribute::Column::IdExtAttribute).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute batch_delete failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = ext_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ExtAttributeQuery;

impl ExtAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<ext_attribute::Model, TcdtServiceError> {
        let ext_attribute_entity =
            ext_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("ExtAttribute find_by_id failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("ExtAttribute cant not find data"))?;
        Ok(ext_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<ext_attribute::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idExtAttribute", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_condition,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let ext_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_by_ids failed", err)
            })?;

        Ok(ext_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<ext_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_condition,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<ext_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ExtAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute num_items failed", err)
            })?;
        let ext_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute fetch_page failed", err)
            })?;
        Ok((ext_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<ext_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_condition,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let ext_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_collection_by_condition failed", err)
            })?;

        Ok(ext_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<ext_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_condition,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_condition,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            ext_attribute::Entity::default(),
            aq_condition,
            "dd_ext_attribute",
            "ExtAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ExtAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ExtAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
