use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dto_entity_associate_po::DtoEntityAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dto_entity_associate;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DtoEntityAssociateMutation;

impl DtoEntityAssociateMutation {
    pub async fn create(
        db: &DbConn,
        dto_entity_associate_model: dto_entity_associate::Model,
    ) -> Result<dto_entity_associate::Model, TcdtServiceError> {
        let mut dto_entity_associate_active_model = dto_entity_associate::convert_model_to_active_model(dto_entity_associate_model);
        let id = generate_id();
        dto_entity_associate_active_model.id_dto_entity_associate = Set(id.clone());
        let _ = dto_entity_associate::Entity::insert(dto_entity_associate_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEntityAssociate insert failed",
                err,
            )
        })?;

        let dto_entity_associate_save = dto_entity_associate::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoEntityAssociate insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DtoEntityAssociate insert after cannot find entity"))?;
        Ok(dto_entity_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_entity_associate_model: dto_entity_associate::Model,
    ) -> Result<dto_entity_associate::Model, TcdtServiceError> {
        let id = dto_entity_associate_model.id_dto_entity_associate.clone();

        let dto_entity_associate_persist_model: dto_entity_associate::ActiveModel = dto_entity_associate::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DtoEntityAssociate update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DtoEntityAssociate update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dto_entity_associate_active_model = dto_entity_associate::convert_model_to_active_model(dto_entity_associate_model);

        let dto_entity_associate_save = dto_entity_associate_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DtoEntityAssociate update failed",
                err,
            )
        })?;

        Ok(dto_entity_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_entity_associate_model: dto_entity_associate::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dto_entity_associate::Entity::delete(dto_entity_associate_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dto_entity_associate_model_list: Vec<dto_entity_associate::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dto_entity_associate_model_list.iter().map(|dto_entity_associate_model| {
            dto_entity_associate_model.id_dto_entity_associate.clone()
        }).collect::<Vec<String>>();
        let delete_result = dto_entity_associate::Entity::delete_many()
            .filter(Expr::col(dto_entity_associate::Column::IdDtoEntityAssociate).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_entity_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoEntityAssociateQuery;

impl DtoEntityAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_entity_associate::Model, TcdtServiceError> {
        let dto_entity_associate_entity =
            dto_entity_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DtoEntityAssociate find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoEntityAssociate cant not find data"))?;
        Ok(dto_entity_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_entity_associate::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idDtoEntityAssociate", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let dto_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_by_ids failed", err)
            })?;

        Ok(dto_entity_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_entity_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_entity_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate num_items failed", err)
            })?;
        let dto_entity_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate fetch_page failed", err)
            })?;
        Ok((dto_entity_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let dto_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_collection_by_condition failed", err)
            })?;

        Ok(dto_entity_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_entity_associate::Entity::default(),
            aq_condition,
            "dto_entity_associate",
            "DtoEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoEntityAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoEntityAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
