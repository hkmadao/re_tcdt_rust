use crate::{
    common::{
        aq::*,
    },
    dto::po::base::component_entity_associate_po::ComponentEntityAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::component_entity_associate;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ComponentEntityAssociateMutation;

impl ComponentEntityAssociateMutation {
    pub async fn create(
        db: &DbConn,
        component_entity_associate_model: component_entity_associate::Model,
    ) -> Result<component_entity_associate::Model, TcdtServiceError> {
        let mut component_entity_associate_active_model = component_entity_associate::convert_model_to_active_model(component_entity_associate_model);
        let id = generate_id();
        component_entity_associate_active_model.id_component_entity_associate = Set(id.clone());
        let _ = component_entity_associate::Entity::insert(component_entity_associate_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComponentEntityAssociate insert failed",
                err,
            )
        })?;

        let component_entity_associate_save = component_entity_associate::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "ComponentEntityAssociate insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("ComponentEntityAssociate insert after cannot find entity"))?;
        Ok(component_entity_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        component_entity_associate_model: component_entity_associate::Model,
    ) -> Result<component_entity_associate::Model, TcdtServiceError> {
        let id = component_entity_associate_model.id_component_entity_associate.clone();

        let component_entity_associate_persist_model: component_entity_associate::ActiveModel = component_entity_associate::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "ComponentEntityAssociate update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("ComponentEntityAssociate update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut component_entity_associate_active_model = component_entity_associate::convert_model_to_active_model(component_entity_associate_model);

        let component_entity_associate_save = component_entity_associate_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " ComponentEntityAssociate update failed",
                err,
            )
        })?;

        Ok(component_entity_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        component_entity_associate_model: component_entity_associate::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = component_entity_associate::Entity::delete(component_entity_associate_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        component_entity_associate_model_list: Vec<component_entity_associate::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = component_entity_associate_model_list.iter().map(|component_entity_associate_model| {
            component_entity_associate_model.id_component_entity_associate.clone()
        }).collect::<Vec<String>>();
        let delete_result = component_entity_associate::Entity::delete_many()
            .filter(Expr::col(component_entity_associate::Column::IdComponentEntityAssociate).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate batch_delete failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = component_entity_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ComponentEntityAssociateQuery;

impl ComponentEntityAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<component_entity_associate::Model, TcdtServiceError> {
        let component_entity_associate_entity =
            component_entity_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("ComponentEntityAssociate find_by_id failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("ComponentEntityAssociate cant not find data"))?;
        Ok(component_entity_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<component_entity_associate::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idComponentEntityAssociate", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_condition,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let component_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_by_ids failed", err)
            })?;

        Ok(component_entity_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<component_entity_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_condition,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<component_entity_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate num_items failed", err)
            })?;
        let component_entity_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate fetch_page failed", err)
            })?;
        Ok((component_entity_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<component_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_condition,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let component_entity_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_collection_by_condition failed", err)
            })?;

        Ok(component_entity_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<component_entity_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_condition,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_condition,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            component_entity_associate::Entity::default(),
            aq_condition,
            "dd_component_entity_associate",
            "ComponentEntityAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("ComponentEntityAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("ComponentEntityAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
