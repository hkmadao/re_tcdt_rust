use crate::{
    common::{
        aq::*,
    },
    dto::po::base::sub_project_po::SubProjectPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::sub_project;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct SubProjectMutation;

impl SubProjectMutation {
    pub async fn create(
        db: &DbConn,
        sub_project_model: sub_project::Model,
    ) -> Result<sub_project::Model, TcdtServiceError> {
        let mut sub_project_active_model = sub_project::convert_model_to_active_model(sub_project_model);
        let id = generate_id();
        sub_project_active_model.id_sub_project = Set(id.clone());
        let _ = sub_project::Entity::insert(sub_project_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "SubProject insert failed",
                err,
            )
        })?;

        let sub_project_save = sub_project::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "SubProject insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("SubProject insert after cannot find entity"))?;
        Ok(sub_project_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        sub_project_model: sub_project::Model,
    ) -> Result<sub_project::Model, TcdtServiceError> {
        let id = sub_project_model.id_sub_project.clone();

        let sub_project_persist_model: sub_project::ActiveModel = sub_project::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "SubProject update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("SubProject update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut sub_project_active_model = sub_project::convert_model_to_active_model(sub_project_model);

        let sub_project_save = sub_project_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " SubProject update failed",
                err,
            )
        })?;

        Ok(sub_project_save)
    }

    pub async fn delete(
        db: &DbConn,
        sub_project_model: sub_project::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = sub_project::Entity::delete(sub_project_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("SubProject delete failed");
                TcdtServiceError::build_internal_msg_error("SubProject delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        sub_project_model_list: Vec<sub_project::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = sub_project_model_list.iter().map(|sub_project_model| {
            sub_project_model.id_sub_project.clone()
        }).collect::<Vec<String>>();
        let delete_result = sub_project::Entity::delete_many()
            .filter(Expr::col(sub_project::Column::IdSubProject).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("SubProject batch_delete failed");
                TcdtServiceError::build_internal_msg_error("SubProject batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = sub_project::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("SubProject delete_all failed");
                TcdtServiceError::build_internal_msg_error("SubProject delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct SubProjectQuery;

impl SubProjectQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<sub_project::Model, TcdtServiceError> {
        let sub_project_entity =
            sub_project::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("SubProject find_by_id failed");
                TcdtServiceError::build_internal_msg_error("SubProject find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("SubProject cant not find data"))?;
        Ok(sub_project_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<sub_project::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idSubProject", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            sub_project::Entity::default(),
            aq_condition,
            "dd_sub_project",
            "SubProject",
        )?;

        let sub_projects = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("SubProject find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("SubProject find_by_ids failed", err)
            })?;

        Ok(sub_projects)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<sub_project::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            sub_project::Entity::default(),
            aq_condition,
            "dd_sub_project",
            "SubProject",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<sub_project::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("SubProject num_items failed");
                TcdtServiceError::build_internal_msg_error("SubProject num_items failed", err)
            })?;
        let sub_projects = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("SubProject fetch_page failed");
                TcdtServiceError::build_internal_msg_error("SubProject fetch_page failed", err)
            })?;
        Ok((sub_projects, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<sub_project::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            sub_project::Entity::default(),
            aq_condition,
            "dd_sub_project",
            "SubProject",
        )?;

        let sub_projects = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("SubProject find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("SubProject find_collection_by_condition failed", err)
            })?;

        Ok(sub_projects)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<sub_project::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            sub_project::Entity::default(),
            aq_condition,
            "dd_sub_project",
            "SubProject",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("SubProject find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("SubProject find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            sub_project::Entity::default(),
            aq_condition,
            "dd_sub_project",
            "SubProject",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("SubProject count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("SubProject count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            sub_project::Entity::default(),
            aq_condition,
            "dd_sub_project",
            "SubProject",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("SubProject exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("SubProject exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
