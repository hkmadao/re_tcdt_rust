use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dto_enum_po::DtoEnumPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dto_enum;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DtoEnumMutation;

impl DtoEnumMutation {
    pub async fn create(
        db: &DbConn,
        dto_enum_model: dto_enum::Model,
    ) -> Result<dto_enum::Model, TcdtServiceError> {
        let mut dto_enum_active_model = dto_enum::convert_model_to_active_model(dto_enum_model);
        let id = generate_id();
        dto_enum_active_model.id_dto_enum = Set(id.clone());
        let _ = dto_enum::Entity::insert(dto_enum_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEnum insert failed",
                err,
            )
        })?;

        let dto_enum_save = dto_enum::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEnum insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DtoEnum insert after cannot find entity"))?;
        Ok(dto_enum_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_enum_model: dto_enum::Model,
    ) -> Result<dto_enum::Model, TcdtServiceError> {
        let id = dto_enum_model.id_dto_enum.clone();

        let dto_enum_persist_model: dto_enum::ActiveModel = dto_enum::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DtoEnum update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DtoEnum update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dto_enum_active_model = dto_enum::convert_model_to_active_model(dto_enum_model);

        let dto_enum_save = dto_enum_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DtoEnum update failed",
                err,
            )
        })?;

        Ok(dto_enum_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_enum_model: dto_enum::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dto_enum::Entity::delete(dto_enum_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dto_enum_model_list: Vec<dto_enum::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dto_enum_model_list.iter().map(|dto_enum_model| {
            dto_enum_model.id_dto_enum.clone()
        }).collect::<Vec<String>>();
        let delete_result = dto_enum::Entity::delete_many()
            .filter(Expr::col(dto_enum::Column::IdDtoEnum).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEnum batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEnum batch_delete failed", err)
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
        let aq_condition = AqCondition::build_in_condition("idDtoEnum", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

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
