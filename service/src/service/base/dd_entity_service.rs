use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dd_entity_po::DdEntityPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dd_entity;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DdEntityMutation;

impl DdEntityMutation {
    pub async fn create(
        db: &DbConn,
        dd_entity_model: dd_entity::Model,
    ) -> Result<dd_entity::Model, TcdtServiceError> {
        let mut dd_entity_active_model = dd_entity::convert_model_to_active_model(dd_entity_model);
        let id = generate_id();
        dd_entity_active_model.id_entity = Set(id.clone());
        let _ = dd_entity::Entity::insert(dd_entity_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DdEntity insert failed",
                err,
            )
        })?;

        let dd_entity_save = dd_entity::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DdEntity insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DdEntity insert after cannot find entity"))?;
        Ok(dd_entity_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dd_entity_model: dd_entity::Model,
    ) -> Result<dd_entity::Model, TcdtServiceError> {
        let id = dd_entity_model.id_entity.clone();

        let dd_entity_persist_model: dd_entity::ActiveModel = dd_entity::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DdEntity update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DdEntity update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dd_entity_active_model = dd_entity::convert_model_to_active_model(dd_entity_model);

        let dd_entity_save = dd_entity_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DdEntity update failed",
                err,
            )
        })?;

        Ok(dd_entity_save)
    }

    pub async fn delete(
        db: &DbConn,
        dd_entity_model: dd_entity::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dd_entity::Entity::delete(dd_entity_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity delete failed");
                TcdtServiceError::build_internal_msg_error("DdEntity delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dd_entity_model_list: Vec<dd_entity::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dd_entity_model_list.iter().map(|dd_entity_model| {
            dd_entity_model.id_entity.clone()
        }).collect::<Vec<String>>();
        let delete_result = dd_entity::Entity::delete_many()
            .filter(Expr::col(dd_entity::Column::IdEntity).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DdEntity batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dd_entity::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity delete_all failed");
                TcdtServiceError::build_internal_msg_error("DdEntity delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DdEntityQuery;

impl DdEntityQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dd_entity::Model, TcdtServiceError> {
        let dd_entity_entity =
            dd_entity::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DdEntity find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DdEntity cant not find data"))?;
        Ok(dd_entity_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dd_entity::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idEntity", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_condition,
            "dd_entity",
            "DdEntity",
        )?;

        let dd_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_by_ids failed", err)
            })?;

        Ok(dd_entitys)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dd_entity::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_condition,
            "dd_entity",
            "DdEntity",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dd_entity::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DdEntity num_items failed");
                TcdtServiceError::build_internal_msg_error("DdEntity num_items failed", err)
            })?;
        let dd_entitys = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DdEntity fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DdEntity fetch_page failed", err)
            })?;
        Ok((dd_entitys, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dd_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_condition,
            "dd_entity",
            "DdEntity",
        )?;

        let dd_entitys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_collection_by_condition failed", err)
            })?;

        Ok(dd_entitys)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dd_entity::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_condition,
            "dd_entity",
            "DdEntity",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_condition,
            "dd_entity",
            "DdEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_entity::Entity::default(),
            aq_condition,
            "dd_entity",
            "DdEntity",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEntity exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEntity exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
