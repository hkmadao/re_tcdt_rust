use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dto_entity_po::DtoEntityPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dto_entity;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DtoEntityMutation;

impl DtoEntityMutation {
    pub async fn create(
        db: &DbConn,
        dto_entity_model: dto_entity::Model,
    ) -> Result<dto_entity::Model, TcdtServiceError> {
        let mut dto_entity_active_model = dto_entity::convert_model_to_active_model(dto_entity_model);
        let id = generate_id();
        dto_entity_active_model.id_dto_entity = Set(id.clone());
        let _ = dto_entity::Entity::insert(dto_entity_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEntity insert failed",
                err,
            )
        })?;

        let dto_entity_save = dto_entity::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEntity insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DtoEntity insert after cannot find entity"))?;
        Ok(dto_entity_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_entity_model: dto_entity::Model,
    ) -> Result<dto_entity::Model, TcdtServiceError> {
        let id = dto_entity_model.id_dto_entity.clone();

        let dto_entity_persist_model: dto_entity::ActiveModel = dto_entity::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DtoEntity update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DtoEntity update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dto_entity_active_model = dto_entity::convert_model_to_active_model(dto_entity_model);

        let dto_entity_save = dto_entity_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DtoEntity update failed",
                err,
            )
        })?;

        Ok(dto_entity_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_entity_model: dto_entity::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dto_entity::Entity::delete(dto_entity_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dto_entity_model_list: Vec<dto_entity::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dto_entity_model_list.iter().map(|dto_entity_model| {
            dto_entity_model.id_dto_entity.clone()
        }).collect::<Vec<String>>();
        let delete_result = dto_entity::Entity::delete_many()
            .filter(Expr::col(dto_entity::Column::IdDtoEntity).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_entity::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEntityQuery;

impl DtoEntityQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_entity::Model, TcdtServiceError> {
        let dto_entity_entity =
            dto_entity::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DtoEntity find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEntity cant not find data"))?;
        Ok(dto_entity_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_entity::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idDtoEntity", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dto_entity::Entity::default(),
            aq_condition,
            "dto_entity",
            "DtoEntity",
        )?;

        let dto_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity find_by_ids failed", err)
            })?;

        Ok(dto_entitys)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_entity::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_entity::Entity::default(),
            aq_condition,
            "dto_entity",
            "DtoEntity",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_entity::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEntity num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity num_items failed", err)
            })?;
        let dto_entitys = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEntity fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity fetch_page failed", err)
            })?;
        Ok((dto_entitys, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity::Entity::default(),
            aq_condition,
            "dto_entity",
            "DtoEntity",
        )?;

        let dto_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity find_collection_by_condition failed", err)
            })?;

        Ok(dto_entitys)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity::Entity::default(),
            aq_condition,
            "dto_entity",
            "DtoEntity",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity::Entity::default(),
            aq_condition,
            "dto_entity",
            "DtoEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity::Entity::default(),
            aq_condition,
            "dto_entity",
            "DtoEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntity exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntity exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
