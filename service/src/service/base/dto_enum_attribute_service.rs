use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dto_enum_attribute_po::DtoEnumAttributePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dto_enum_attribute;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DtoEnumAttributeMutation;

impl DtoEnumAttributeMutation {
    pub async fn create(
        db: &DbConn,
        dto_enum_attribute_model: dto_enum_attribute::Model,
    ) -> Result<dto_enum_attribute::Model, TcdtServiceError> {
        let mut dto_enum_attribute_active_model = dto_enum_attribute::convert_model_to_active_model(dto_enum_attribute_model);
        let id = generate_id();
        dto_enum_attribute_active_model.id_dto_enum_attribute = Set(id.clone());
        let _ = dto_enum_attribute::Entity::insert(dto_enum_attribute_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEnumAttribute insert failed",
                err,
            )
        })?;

        let dto_enum_attribute_save = dto_enum_attribute::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEnumAttribute insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DtoEnumAttribute insert after cannot find entity"))?;
        Ok(dto_enum_attribute_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_enum_attribute_model: dto_enum_attribute::Model,
    ) -> Result<dto_enum_attribute::Model, TcdtServiceError> {
        let id = dto_enum_attribute_model.id_dto_enum_attribute.clone();

        let dto_enum_attribute_persist_model: dto_enum_attribute::ActiveModel = dto_enum_attribute::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DtoEnumAttribute update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DtoEnumAttribute update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dto_enum_attribute_active_model = dto_enum_attribute::convert_model_to_active_model(dto_enum_attribute_model);

        let dto_enum_attribute_save = dto_enum_attribute_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DtoEnumAttribute update failed",
                err,
            )
        })?;

        Ok(dto_enum_attribute_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_enum_attribute_model: dto_enum_attribute::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dto_enum_attribute::Entity::delete(dto_enum_attribute_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dto_enum_attribute_model_list: Vec<dto_enum_attribute::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dto_enum_attribute_model_list.iter().map(|dto_enum_attribute_model| {
            dto_enum_attribute_model.id_dto_enum_attribute.clone()
        }).collect::<Vec<String>>();
        let delete_result = dto_enum_attribute::Entity::delete_many()
            .filter(Expr::col(dto_enum_attribute::Column::IdDtoEnumAttribute).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_enum_attribute::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEnumAttributeQuery;

impl DtoEnumAttributeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_enum_attribute::Model, TcdtServiceError> {
        let dto_enum_attribute_entity =
            dto_enum_attribute::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DtoEnumAttribute find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEnumAttribute cant not find data"))?;
        Ok(dto_enum_attribute_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_enum_attribute::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idDtoEnumAttribute", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dto_enum_attribute::Entity::default(),
            aq_condition,
            "dto_enum_attribute",
            "DtoEnumAttribute",
        )?;

        let dto_enum_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute find_by_ids failed", err)
            })?;

        Ok(dto_enum_attributes)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_enum_attribute::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_enum_attribute::Entity::default(),
            aq_condition,
            "dto_enum_attribute",
            "DtoEnumAttribute",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_enum_attribute::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute num_items failed", err)
            })?;
        let dto_enum_attributes = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute fetch_page failed", err)
            })?;
        Ok((dto_enum_attributes, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_enum_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_attribute::Entity::default(),
            aq_condition,
            "dto_enum_attribute",
            "DtoEnumAttribute",
        )?;

        let dto_enum_attributes = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute find_collection_by_condition failed", err)
            })?;

        Ok(dto_enum_attributes)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_enum_attribute::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_attribute::Entity::default(),
            aq_condition,
            "dto_enum_attribute",
            "DtoEnumAttribute",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_attribute::Entity::default(),
            aq_condition,
            "dto_enum_attribute",
            "DtoEnumAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_enum_attribute::Entity::default(),
            aq_condition,
            "dto_enum_attribute",
            "DtoEnumAttribute",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnumAttribute exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEnumAttribute exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
