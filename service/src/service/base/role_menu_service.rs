use crate::{
    common::{
        aq::*,
    },
    dto::po::base::role_menu_po::RoleMenuPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::role_menu;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct RoleMenuMutation;

impl RoleMenuMutation {
    pub async fn create(
        db: &DbConn,
        role_menu_model: role_menu::Model,
    ) -> Result<role_menu::Model, TcdtServiceError> {
        let mut role_menu_active_model = role_menu::convert_model_to_active_model(role_menu_model);
        let id = generate_id();
        role_menu_active_model.id_role_menu = Set(id.clone());
        let _ = role_menu::Entity::insert(role_menu_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "RoleMenu insert failed",
                err,
            )
        })?;

        let role_menu_save = role_menu::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "RoleMenu insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("RoleMenu insert after cannot find entity"))?;
        Ok(role_menu_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        role_menu_model: role_menu::Model,
    ) -> Result<role_menu::Model, TcdtServiceError> {
        let id = role_menu_model.id_role_menu.clone();

        let role_menu_persist_model: role_menu::ActiveModel = role_menu::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "RoleMenu update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("RoleMenu update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut role_menu_active_model = role_menu::convert_model_to_active_model(role_menu_model);

        let role_menu_save = role_menu_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " RoleMenu update failed",
                err,
            )
        })?;

        Ok(role_menu_save)
    }

    pub async fn delete(
        db: &DbConn,
        role_menu_model: role_menu::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = role_menu::Entity::delete(role_menu_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu delete failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        role_menu_model_list: Vec<role_menu::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = role_menu_model_list.iter().map(|role_menu_model| {
            role_menu_model.id_role_menu.clone()
        }).collect::<Vec<String>>();
        let delete_result = role_menu::Entity::delete_many()
            .filter(Expr::col(role_menu::Column::IdRoleMenu).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu batch_delete failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = role_menu::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu delete_all failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct RoleMenuQuery;

impl RoleMenuQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<role_menu::Model, TcdtServiceError> {
        let role_menu_entity =
            role_menu::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("RoleMenu find_by_id failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("RoleMenu cant not find data"))?;
        Ok(role_menu_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<role_menu::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idRoleMenu", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_condition,
            "sys_role_menu",
            "RoleMenu",
        )?;

        let role_menus = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu find_by_ids failed", err)
            })?;

        Ok(role_menus)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<role_menu::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_condition,
            "sys_role_menu",
            "RoleMenu",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<role_menu::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("RoleMenu num_items failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu num_items failed", err)
            })?;
        let role_menus = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("RoleMenu fetch_page failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu fetch_page failed", err)
            })?;
        Ok((role_menus, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<role_menu::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_condition,
            "sys_role_menu",
            "RoleMenu",
        )?;

        let role_menus = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu find_collection_by_condition failed", err)
            })?;

        Ok(role_menus)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<role_menu::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_condition,
            "sys_role_menu",
            "RoleMenu",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_condition,
            "sys_role_menu",
            "RoleMenu",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_condition,
            "sys_role_menu",
            "RoleMenu",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("RoleMenu exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
