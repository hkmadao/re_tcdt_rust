use crate::{
    common::{
        aq::*,
    },
    dto::po::base::menu_po::MenuPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::menu;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct MenuMutation;

impl MenuMutation {
    pub async fn create(
        db: &DbConn,
        menu_model: menu::Model,
    ) -> Result<menu::Model, TcdtServiceError> {
        let mut menu_active_model = menu::convert_model_to_active_model(menu_model);
        let id = generate_id();
        menu_active_model.id_menu = Set(id.clone());
        let _ = menu::Entity::insert(menu_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Menu insert failed",
                err,
            )
        })?;

        let menu_save = menu::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Menu insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Menu insert after cannot find entity"))?;
        Ok(menu_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        menu_model: menu::Model,
    ) -> Result<menu::Model, TcdtServiceError> {
        let id = menu_model.id_menu.clone();

        let menu_persist_model: menu::ActiveModel = menu::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Menu update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Menu update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut menu_active_model = menu::convert_model_to_active_model(menu_model);

        let menu_save = menu_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Menu update failed",
                err,
            )
        })?;

        Ok(menu_save)
    }

    pub async fn delete(
        db: &DbConn,
        menu_model: menu::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = menu::Entity::delete(menu_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Menu delete failed");
                TcdtServiceError::build_internal_msg_error("Menu delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        menu_model_list: Vec<menu::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = menu_model_list.iter().map(|menu_model| {
            menu_model.id_menu.clone()
        }).collect::<Vec<String>>();
        let delete_result = menu::Entity::delete_many()
            .filter(Expr::col(menu::Column::IdMenu).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Menu batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Menu batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = menu::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Menu delete_all failed");
                TcdtServiceError::build_internal_msg_error("Menu delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct MenuQuery;

impl MenuQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<menu::Model, TcdtServiceError> {
        let menu_entity =
            menu::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("Menu find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Menu find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("Menu cant not find data"))?;
        Ok(menu_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<menu::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idMenu", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            menu::Entity::default(),
            aq_condition,
            "sys_menu",
            "Menu",
        )?;

        let menus = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Menu find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Menu find_by_ids failed", err)
            })?;

        Ok(menus)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<menu::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            menu::Entity::default(),
            aq_condition,
            "sys_menu",
            "Menu",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<menu::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Menu num_items failed");
                TcdtServiceError::build_internal_msg_error("Menu num_items failed", err)
            })?;
        let menus = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Menu fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Menu fetch_page failed", err)
            })?;
        Ok((menus, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<menu::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            menu::Entity::default(),
            aq_condition,
            "sys_menu",
            "Menu",
        )?;

        let menus = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Menu find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Menu find_collection_by_condition failed", err)
            })?;

        Ok(menus)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<menu::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            menu::Entity::default(),
            aq_condition,
            "sys_menu",
            "Menu",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Menu find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Menu find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            menu::Entity::default(),
            aq_condition,
            "sys_menu",
            "Menu",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Menu count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Menu count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            menu::Entity::default(),
            aq_condition,
            "sys_menu",
            "Menu",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Menu exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Menu exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
