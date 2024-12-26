use crate::{
    common::{
        aq::*,
    },
    dto::po::base::factory_po::FactoryPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::factory;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct FactoryMutation;

impl FactoryMutation {
    pub async fn create(
        db: &DbConn,
        factory_model: factory::Model,
    ) -> Result<factory::Model, TcdtServiceError> {
        let mut factory_active_model = factory::convert_model_to_active_model(factory_model);
        let id = generate_id();
        factory_active_model.id_factory = Set(id.clone());
        let _ = factory::Entity::insert(factory_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Factory insert failed",
                err,
            )
        })?;

        let factory_save = factory::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Factory insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Factory insert after cannot find entity"))?;
        Ok(factory_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        factory_model: factory::Model,
    ) -> Result<factory::Model, TcdtServiceError> {
        let id = factory_model.id_factory.clone();

        let factory_persist_model: factory::ActiveModel = factory::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Factory update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Factory update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut factory_active_model = factory::convert_model_to_active_model(factory_model);

        let factory_save = factory_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Factory update failed",
                err,
            )
        })?;

        Ok(factory_save)
    }

    pub async fn delete(
        db: &DbConn,
        factory_model: factory::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = factory::Entity::delete(factory_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Factory delete failed");
                TcdtServiceError::build_internal_msg_error("Factory delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        factory_model_list: Vec<factory::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = factory_model_list.iter().map(|factory_model| {
            factory_model.id_factory.clone()
        }).collect::<Vec<String>>();
        let delete_result = factory::Entity::delete_many()
            .filter(Expr::col(factory::Column::IdFactory).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Factory batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Factory batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = factory::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Factory delete_all failed");
                TcdtServiceError::build_internal_msg_error("Factory delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct FactoryQuery;

impl FactoryQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<factory::Model, TcdtServiceError> {
        let factory_entity =
            factory::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("Factory find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Factory find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("Factory cant not find data"))?;
        Ok(factory_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<factory::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idFactory", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            factory::Entity::default(),
            aq_condition,
            "ui_factory",
            "Factory",
        )?;

        let factorys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Factory find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Factory find_by_ids failed", err)
            })?;

        Ok(factorys)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<factory::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            factory::Entity::default(),
            aq_condition,
            "ui_factory",
            "Factory",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<factory::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Factory num_items failed");
                TcdtServiceError::build_internal_msg_error("Factory num_items failed", err)
            })?;
        let factorys = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Factory fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Factory fetch_page failed", err)
            })?;
        Ok((factorys, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<factory::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            factory::Entity::default(),
            aq_condition,
            "ui_factory",
            "Factory",
        )?;

        let factorys = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Factory find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Factory find_collection_by_condition failed", err)
            })?;

        Ok(factorys)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<factory::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            factory::Entity::default(),
            aq_condition,
            "ui_factory",
            "Factory",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Factory find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Factory find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            factory::Entity::default(),
            aq_condition,
            "ui_factory",
            "Factory",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Factory count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Factory count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            factory::Entity::default(),
            aq_condition,
            "ui_factory",
            "Factory",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Factory exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Factory exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
