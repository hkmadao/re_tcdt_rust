use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::user_po::UserPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;
use ::entity::entity::user;
use sea_orm::*;

pub struct UserMutation;

impl UserMutation {
    pub async fn create(
        db: &DbConn,
        user_po: UserPO,
    ) -> Result<user::Model, TcdtServiceError> {
        let user_save = UserPO::insert(user_po, db, None)
            .await
            .map_err(|err| {
                log::error!("User insert failed");
                TcdtServiceError::build_internal_msg_error("User insert failed", err)
            })?;
        Ok(user_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        user_po: UserPO,
    ) -> Result<user::Model, TcdtServiceError> {
        let user_save = UserPO::update(user_po, db, None)
            .await
            .map_err(|err| {
                log::error!("User update failed");
                TcdtServiceError::build_internal_msg_error("User update failed", err)
            })?;
        Ok(user_save)
    }

    pub async fn delete(
        db: &DbConn,
        user_po: UserPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = UserPO::delete(user_po, db, None)
            .await
            .map_err(|err| {
                log::error!("User delete failed");
                TcdtServiceError::build_internal_msg_error("User delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        user_po_list: Vec<UserPO>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = user_po_list
            .iter()
            .map(|po| po.id_user.clone())
            .collect::<Vec<_>>();
        let delete_result = user::Entity::delete_many()
            .filter(user::Column::IdUser.is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("User batch_delete failed");
                TcdtServiceError::build_internal_msg_error("User batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = user::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("User delete_all failed");
                TcdtServiceError::build_internal_msg_error("User delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct UserQuery;

impl UserQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<user::Model, TcdtServiceError> {
        let user_entity =
            user::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                    log::error!("User find_by_id failed");
                    TcdtServiceError::build_internal_msg_error("User find_by_id failed", err)
                })?
                .ok_or(TcdtServiceError::build_internal_msg("User cant not find data"))?;
        Ok(user_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<user::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idUser".to_string(),
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
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let users = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("User find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("User find_by_ids failed", err)
            })?;

        Ok(users)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<user::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<user::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("User num_items failed");
                TcdtServiceError::build_internal_msg_error("User num_items failed", err)
            })?;
        let users = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("User fetch_page failed");
                TcdtServiceError::build_internal_msg_error("User fetch_page failed", err)
            })?;
        Ok((users, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<user::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let users = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("User find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User find_collection_by_condition failed", err)
            })?;

        Ok(users)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<user::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("User find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("User count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            user::Entity::default(),
            aq_condition,
            "sys_user",
            "User",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("User exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("User exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
