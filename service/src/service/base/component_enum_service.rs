use crate::{
    common::{
        aq::*,
    },
    dto::po::base::component_enum_po::ComponentEnumPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::component_enum;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ComponentEnumMutation;

impl ComponentEnumMutation {
    pub async fn create(
        db: &DbConn,
        component_enum_model: component_enum::Model,
    ) -> Result<component_enum::Model, TcdtServiceError> {
        let mut component_enum_active_model = component_enum::convert_model_to_active_model(component_enum_model);
        let id = generate_id();
        component_enum_active_model.id_component_enum = Set(id.clone());
        let _ = component_enum::Entity::insert(component_enum_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComponentEnum insert failed",
                err,
            )
        })?;

        let component_enum_save = component_enum::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComponentEnum insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("ComponentEnum insert after cannot find entity"))?;
        Ok(component_enum_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_enum_model: component_enum::Model,
    ) -> Result<component_enum::Model, TcdtServiceError> {
        let id = component_enum_model.id_component_enum.clone();

        let component_enum_persist_model: component_enum::ActiveModel = component_enum::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "ComponentEnum update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("ComponentEnum update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut component_enum_active_model = component_enum::convert_model_to_active_model(component_enum_model);

        let component_enum_save = component_enum_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " ComponentEnum update failed",
                err,
            )
        })?;

        Ok(component_enum_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_enum_model: component_enum::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = component_enum::Entity::delete(component_enum_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        component_enum_model_list: Vec<component_enum::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = component_enum_model_list.iter().map(|component_enum_model| {
            component_enum_model.id_component_enum.clone()
        }).collect::<Vec<String>>();
        let delete_result = component_enum::Entity::delete_many()
            .filter(Expr::col(component_enum::Column::IdComponentEnum).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum batch_delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_enum::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentEnumQuery;

impl ComponentEnumQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_enum::Model, TcdtServiceError> {
        let component_enum_entity =
            component_enum::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("ComponentEnum find_by_id failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentEnum cant not find data"))?;
        Ok(component_enum_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_enum::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idComponentEnum", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let component_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_by_ids failed", err)
            })?;

        Ok(component_enums)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_enum::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_enum::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentEnum num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum num_items failed", err)
            })?;
        let component_enums = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum fetch_page failed", err)
            })?;
        Ok((component_enums, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<component_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let component_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_collection_by_condition failed", err)
            })?;

        Ok(component_enums)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<component_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_enum::Entity::default(),
            aq_condition,
            "dd_component_enum",
            "ComponentEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEnum exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEnum exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
