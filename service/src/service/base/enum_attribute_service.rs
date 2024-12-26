use crate::{
    common::{
        aq::*,
    },
    dto::po::base::enum_attribute_po::EnumAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::enum_attribute;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct EnumAttributeMutation;

impl EnumAttributeMutation {
    pub async fn create(
        db: &DbConn,
        enum_attribute_model: enum_attribute::Model,
    ) -> Result<enum_attribute::Model, TcdtServiceError> {
        let mut enum_attribute_active_model = enum_attribute::convert_model_to_active_model(enum_attribute_model);
        let id = generate_id();
        enum_attribute_active_model.id_enum_attribute = Set(id.clone());
        let _ = enum_attribute::Entity::insert(enum_attribute_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "EnumAttribute insert failed",
                err,
            )
        })?;

        let enum_attribute_save = enum_attribute::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "EnumAttribute insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("EnumAttribute insert after cannot find entity"))?;
        Ok(enum_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        enum_attribute_model: enum_attribute::Model,
    ) -> Result<enum_attribute::Model, TcdtServiceError> {
        let id = enum_attribute_model.id_enum_attribute.clone();

        let enum_attribute_persist_model: enum_attribute::ActiveModel = enum_attribute::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "EnumAttribute update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("EnumAttribute update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut enum_attribute_active_model = enum_attribute::convert_model_to_active_model(enum_attribute_model);

        let enum_attribute_save = enum_attribute_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " EnumAttribute update failed",
                err,
            )
        })?;

        Ok(enum_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        enum_attribute_model: enum_attribute::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = enum_attribute::Entity::delete(enum_attribute_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        enum_attribute_model_list: Vec<enum_attribute::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = enum_attribute_model_list.iter().map(|enum_attribute_model| {
            enum_attribute_model.id_enum_attribute.clone()
        }).collect::<Vec<String>>();
        let delete_result = enum_attribute::Entity::delete_many()
            .filter(Expr::col(enum_attribute::Column::IdEnumAttribute).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute batch_delete failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = enum_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct EnumAttributeQuery;

impl EnumAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<enum_attribute::Model, TcdtServiceError> {
        let enum_attribute_entity =
            enum_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("EnumAttribute find_by_id failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("EnumAttribute cant not find data"))?;
        Ok(enum_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<enum_attribute::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idEnumAttribute", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            enum_attribute::Entity::default(),
            aq_condition,
            "dd_enum_attribute",
            "EnumAttribute",
        )?;

        let enum_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute find_by_ids failed", err)
            })?;

        Ok(enum_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<enum_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            enum_attribute::Entity::default(),
            aq_condition,
            "dd_enum_attribute",
            "EnumAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<enum_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("EnumAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute num_items failed", err)
            })?;
        let enum_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute fetch_page failed", err)
            })?;
        Ok((enum_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<enum_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_attribute::Entity::default(),
            aq_condition,
            "dd_enum_attribute",
            "EnumAttribute",
        )?;

        let enum_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute find_collection_by_condition failed", err)
            })?;

        Ok(enum_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<enum_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_attribute::Entity::default(),
            aq_condition,
            "dd_enum_attribute",
            "EnumAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_attribute::Entity::default(),
            aq_condition,
            "dd_enum_attribute",
            "EnumAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_attribute::Entity::default(),
            aq_condition,
            "dd_enum_attribute",
            "EnumAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EnumAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
