use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::role_menu_po::RoleMenuPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::role_menu;
use sea_orm::*;

pub struct RoleMenuMutation;

impl RoleMenuMutation {
    pub async fn create(
        db: &DbConn,
        role_menu_po: RoleMenuPO,
    ) -> Result<role_menu::Model, TcdtServiceError> {
        let role_menu_save = RoleMenuPO::insert(role_menu_po, db, None)
            .await
            .map_err(|err| {
                log::error!("RoleMenu insert failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu insert failed", err)
            })?;
        Ok(role_menu_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        role_menu_po: RoleMenuPO,
    ) -> Result<role_menu::Model, TcdtServiceError> {
        let role_menu_save = RoleMenuPO::update(role_menu_po, db, None)
            .await
            .map_err(|err| {
                log::error!("RoleMenu update failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu update failed", err)
            })?;
        Ok(role_menu_save)
    }

    pub async fn delete(
        db: &DbConn,
        role_menu_po: RoleMenuPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = RoleMenuPO::delete(role_menu_po, db, None)
            .await
            .map_err(|err| {
                log::error!("RoleMenu delete failed");
                TcdtServiceError::build_internal_msg_error("RoleMenu delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        role_menu_po_list: Vec<RoleMenuPO>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = role_menu_po_list
            .iter()
            .map(|po| po.id_role_menu.clone())
            .collect::<Vec<_>>();
        let delete_result = role_menu::Entity::delete_many()
            .filter(role_menu::Column::IdRoleMenu.is_in(id_list))
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
        let aq_conditoin = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idRoleMenu".to_string(),
                    operator_code: OPERATOR_CODE_IN.to_owned(),
                    filter_params: ids
                        .iter()
                        .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
                        .collect(),
                }],
            })),
            orders: vec![],
        };
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_conditoin,
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
        let aq_conditoin = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<Vec<role_menu::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<Option<role_menu::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_conditoin,
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
        aq_conditoin: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            role_menu::Entity::default(),
            aq_conditoin,
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
