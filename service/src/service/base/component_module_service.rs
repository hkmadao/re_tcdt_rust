use crate::{
    common::{
        aq::*,
    },
    dto::po::base::component_module_po::ComponentModulePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::component_module;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ComponentModuleMutation;

impl ComponentModuleMutation {
    pub async fn create(
        db: &DbConn,
        component_module_model: component_module::Model,
    ) -> Result<component_module::Model, TcdtServiceError> {
        let mut component_module_active_model = component_module::convert_model_to_active_model(component_module_model);
        let id = generate_id();
        component_module_active_model.id_component_module = Set(id.clone());
        let _ = component_module::Entity::insert(component_module_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComponentModule insert failed",
                err,
            )
        })?;

        let component_module_save = component_module::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComponentModule insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("ComponentModule insert after cannot find entity"))?;
        Ok(component_module_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_module_model: component_module::Model,
    ) -> Result<component_module::Model, TcdtServiceError> {
        let id = component_module_model.id_component_module.clone();

        let component_module_persist_model: component_module::ActiveModel = component_module::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "ComponentModule update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("ComponentModule update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut component_module_active_model = component_module::convert_model_to_active_model(component_module_model);

        let component_module_save = component_module_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " ComponentModule update failed",
                err,
            )
        })?;

        Ok(component_module_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_module_model: component_module::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = component_module::Entity::delete(component_module_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        component_module_model_list: Vec<component_module::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = component_module_model_list.iter().map(|component_module_model| {
            component_module_model.id_component_module.clone()
        }).collect::<Vec<String>>();
        let delete_result = component_module::Entity::delete_many()
            .filter(Expr::col(component_module::Column::IdComponentModule).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule batch_delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_module::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentModuleQuery;

impl ComponentModuleQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_module::Model, TcdtServiceError> {
        let component_module_entity =
            component_module::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("ComponentModule find_by_id failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentModule cant not find data"))?;
        Ok(component_module_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_module::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idComponentModule", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            component_module::Entity::default(),
            aq_condition,
            "dd_component_module",
            "ComponentModule",
        )?;

        let component_modules = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule find_by_ids failed", err)
            })?;

        Ok(component_modules)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_module::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_module::Entity::default(),
            aq_condition,
            "dd_component_module",
            "ComponentModule",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_module::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentModule num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule num_items failed", err)
            })?;
        let component_modules = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentModule fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule fetch_page failed", err)
            })?;
        Ok((component_modules, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<component_module::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_module::Entity::default(),
            aq_condition,
            "dd_component_module",
            "ComponentModule",
        )?;

        let component_modules = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule find_collection_by_condition failed", err)
            })?;

        Ok(component_modules)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<component_module::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_module::Entity::default(),
            aq_condition,
            "dd_component_module",
            "ComponentModule",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_module::Entity::default(),
            aq_condition,
            "dd_component_module",
            "ComponentModule",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_module::Entity::default(),
            aq_condition,
            "dd_component_module",
            "ComponentModule",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentModule exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentModule exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
