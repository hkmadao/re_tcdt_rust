use crate::{
    common::{
        aq::*,
    },
    dto::po::base::project_po::ProjectPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::project;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct ProjectMutation;

impl ProjectMutation {
    pub async fn create(
        db: &DbConn,
        project_model: project::Model,
    ) -> Result<project::Model, TcdtServiceError> {
        let mut project_active_model = project::convert_model_to_active_model(project_model);
        let id = generate_id();
        project_active_model.id_project = Set(id.clone());
        let _ = project::Entity::insert(project_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Project insert failed",
                err,
            )
        })?;

        let project_save = project::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "Project insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("Project insert after cannot find entity"))?;
        Ok(project_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        project_model: project::Model,
    ) -> Result<project::Model, TcdtServiceError> {
        let id = project_model.id_project.clone();

        let project_persist_model: project::ActiveModel = project::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "Project update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("Project update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut project_active_model = project::convert_model_to_active_model(project_model);

        let project_save = project_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " Project update failed",
                err,
            )
        })?;

        Ok(project_save)
    }

    pub async fn delete(
        db: &DbConn,
        project_model: project::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = project::Entity::delete(project_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Project delete failed");
                TcdtServiceError::build_internal_msg_error("Project delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        project_model_list: Vec<project::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = project_model_list.iter().map(|project_model| {
            project_model.id_project.clone()
        }).collect::<Vec<String>>();
        let delete_result = project::Entity::delete_many()
            .filter(Expr::col(project::Column::IdProject).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("Project batch_delete failed");
                TcdtServiceError::build_internal_msg_error("Project batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = project::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("Project delete_all failed");
                TcdtServiceError::build_internal_msg_error("Project delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct ProjectQuery;

impl ProjectQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<project::Model, TcdtServiceError> {
        let project_entity =
            project::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("Project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Project find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("Project cant not find data"))?;
        Ok(project_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<project::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idProject", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        let projects = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Project find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("Project find_by_ids failed", err)
            })?;

        Ok(projects)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<project::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<project::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("Project num_items failed");
                TcdtServiceError::build_internal_msg_error("Project num_items failed", err)
            })?;
        let projects = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("Project fetch_page failed");
                TcdtServiceError::build_internal_msg_error("Project fetch_page failed", err)
            })?;
        Ok((projects, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<project::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        let projects = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("Project find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Project find_collection_by_condition failed", err)
            })?;

        Ok(projects)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<project::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("Project find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Project find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Project count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Project count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("Project exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("Project exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
