use crate::{
    common::{
        aq::*,
        aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_IN},
    },
    dto::po::base::project_po::ProjectPO,
    util::dyn_query::make_select_by_condition,
};
use ::entity::entity::{common_attribute, data_type, project};
use nanoid::nanoid;
use sea_orm::*;
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtCudParamObjectTrait;

pub struct ProjectMutation;

impl ProjectMutation {
    pub async fn create(
        db: &DbConn,
        project_po: ProjectPO,
    ) -> Result<project::Model, TcdtServiceError> {
        let txn = db.begin().await.map_err(|err| {
            log::error!("Project insert tx begin failed");
            TcdtServiceError::build_internal_msg_error("Project insert tx begin failed", err)
        })?;

        let project_save = Self::save_project(db, project_po, &txn).await?;
        txn.commit().await.map_err(|err| {
            log::error!("Project insert tx commit failed");
            TcdtServiceError::build_internal_msg_error("Project insert tx commit failed", err)
        })?;
        Ok(project_save)
    }

    async fn save_project(
        db: &DbConn,
        project_po: ProjectPO,
        txn: &DatabaseTransaction,
    ) -> Result<project::Model, TcdtServiceError> {
        let project_save = ProjectPO::insert(project_po, txn, None)
            .await
            .map_err(|err| {
                log::error!("Project insert failed");
                TcdtServiceError::build_internal_msg_error("Project insert failed", err)
            })?;
        let id_project = project_save.id_project.clone();
        let preset_data_type_list = data_type::Entity::find()
            .filter(data_type::Column::FgPreset.eq(true))
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find preset data_type failed");
                TcdtServiceError::build_internal_msg_error("find preset data_type failed", err)
            })?;
        let preset_common_attribute_list = common_attribute::Entity::find()
            .filter(common_attribute::Column::FgPreset.eq(true))
            .all(db)
            .await
            .map_err(|err| {
                log::error!("find preset common_attribute failed");
                TcdtServiceError::build_internal_msg_error(
                    "find preset common_attribute failed",
                    err,
                )
            })?;
        let data_type_id_map_list = preset_data_type_list
            .iter()
            .map(|data_entity| {
                let old_id = data_entity.id_data_type.clone();
                let new_id = nanoid!();
                (old_id, new_id)
            })
            .collect::<Vec<_>>();
        let data_type_list = preset_data_type_list
            .iter()
            .map(|data_entity| {
                let mut new_data_entity = data_entity.clone();
                let new_id = data_type_id_map_list
                    .iter()
                    .find(|data_type_id_map| data_type_id_map.0 == data_entity.id_data_type)
                    .unwrap()
                    .1
                    .clone();
                new_data_entity.id_data_type = new_id.clone();
                new_data_entity.id_project = Some(id_project.clone());
                new_data_entity.fg_preset = false;
                new_data_entity.into_active_model()
            })
            .collect::<Vec<_>>();
        let common_attribute_list = preset_common_attribute_list
            .iter()
            .map(|common_attribute_entity| {
                let mut new_data_entity = common_attribute_entity.clone();
                let new_id = data_type_id_map_list
                    .iter()
                    .find(|data_type_id_map| {
                        Some(data_type_id_map.0.to_string()) == common_attribute_entity.id_data_type
                    })
                    .unwrap()
                    .1
                    .clone();
                new_data_entity.id_data_type = Some(new_id.clone());
                new_data_entity.id_project = Some(id_project.clone());
                new_data_entity.id_common_attribute = nanoid!();
                new_data_entity.fg_preset = false;
                new_data_entity.into_active_model()
            })
            .collect::<Vec<_>>();
        if data_type_list.len() > 0 {
            data_type::Entity::insert_many(data_type_list)
                .exec(txn)
                .await
                .map_err(|err| {
                    log::error!("insert_many data_type failed");
                    TcdtServiceError::build_internal_msg_error("insert_many data_type failed", err)
                })?;
        }
        if common_attribute_list.len() > 0 {
            common_attribute::Entity::insert_many(common_attribute_list)
                .exec(txn)
                .await
                .map_err(|err| {
                    log::error!("insert_many common_attribute failed");
                    TcdtServiceError::build_internal_msg_error(
                        "insert_many common_attribute failed",
                        err,
                    )
                })?;
        }
        Ok(project_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        project_po: ProjectPO,
    ) -> Result<project::Model, TcdtServiceError> {
        let project_save = ProjectPO::update(project_po, db, None)
            .await
            .map_err(|err| {
                log::error!("Project update failed");
                TcdtServiceError::build_internal_msg_error("Project update failed", err)
            })?;
        Ok(project_save)
    }

    pub async fn delete(
        db: &DbConn,
        project_po: ProjectPO,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let txn = db.begin().await.map_err(|err| {
            log::error!("delete project tx begin failed");
            TcdtServiceError::build_internal_msg_error("delete project tx begin failed", err)
        })?;
        let id_project = project_po.id_project.clone();
        common_attribute::Entity::delete_many()
            .filter(common_attribute::Column::IdProject.eq(id_project.clone()))
            .exec(&txn)
            .await
            .map_err(|err| {
                log::error!("delete data_type failed");
                TcdtServiceError::build_internal_msg_error("delete data_type failed", err)
            })?;
        data_type::Entity::delete_many()
            .filter(data_type::Column::IdProject.eq(id_project.clone()))
            .exec(&txn)
            .await
            .map_err(|err| {
                log::error!("delete data_type failed");
                TcdtServiceError::build_internal_msg_error("delete data_type failed", err)
            })?;
        let delete_result = ProjectPO::delete(project_po, &txn, None)
            .await
            .map_err(|err| {
                log::error!("Project delete failed");
                TcdtServiceError::build_internal_msg_error("Project delete failed", err)
            })?;
        txn.commit().await.map_err(|err| {
            log::error!("delete project tx commit failed");
            TcdtServiceError::build_internal_msg_error("delete project tx commit failed", err)
        })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = project::Entity::delete_many()
            .exec(db)
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
    pub async fn find_by_id(db: &DbConn, id: String) -> Result<project::Model, TcdtServiceError> {
        let project_entity = project::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| {
                log::error!("Project find_by_id failed");
                TcdtServiceError::build_internal_msg_error("Project find_by_id failed", err)
            })?
            .ok_or(TcdtServiceError::build_internal_msg(
                "Project cant not find data",
            ))?;
        Ok(project_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<project::Model>, TcdtServiceError> {
        let aq_condition = AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_owned(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: "idProject".to_string(),
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
            project::Entity::default(),
            aq_condition,
            "dd_project",
            "Project",
        )?;

        let projects = sql_build.all(db).await.map_err(|err| {
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
        let num_items = paginator.num_items().await.map_err(|err| {
            log::error!("Project num_items failed");
            TcdtServiceError::build_internal_msg_error("Project num_items failed", err)
        })?;
        let projects = paginator.fetch_page(page_index - 1).await.map_err(|err| {
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

        let projects = sql_build.all(db).await.map_err(|err| {
            log::error!("Project find_collection_by_condition failed");
            TcdtServiceError::build_internal_msg_error(
                "Project find_collection_by_condition failed",
                err,
            )
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

        let data_type = sql_build.one(db).await.map_err(|err| {
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

        let count = sql_build.count(db).await.map_err(|err| {
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

        let count = sql_build.count(db).await.map_err(|err| {
            log::error!("Project exists_by_condition failed");
            TcdtServiceError::build_internal_msg_error("Project exists_by_condition failed", err)
        })?;

        Ok(count > 0)
    }
}
