use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dto_node_ui_po::DtoNodeUiPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dto_node_ui;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DtoNodeUiMutation;

impl DtoNodeUiMutation {
    pub async fn create(
        db: &DbConn,
        dto_node_ui_model: dto_node_ui::Model,
    ) -> Result<dto_node_ui::Model, TcdtServiceError> {
        let mut dto_node_ui_active_model = dto_node_ui::convert_model_to_active_model(dto_node_ui_model);
        let id = generate_id();
        dto_node_ui_active_model.id_dto_node_ui = Set(id.clone());
        let _ = dto_node_ui::Entity::insert(dto_node_ui_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoNodeUi insert failed",
                err,
            )
        })?;

        let dto_node_ui_save = dto_node_ui::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DtoNodeUi insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DtoNodeUi insert after cannot find entity"))?;
        Ok(dto_node_ui_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dto_node_ui_model: dto_node_ui::Model,
    ) -> Result<dto_node_ui::Model, TcdtServiceError> {
        let id = dto_node_ui_model.id_dto_node_ui.clone();

        let dto_node_ui_persist_model: dto_node_ui::ActiveModel = dto_node_ui::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DtoNodeUi update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DtoNodeUi update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dto_node_ui_active_model = dto_node_ui::convert_model_to_active_model(dto_node_ui_model);

        let dto_node_ui_save = dto_node_ui_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DtoNodeUi update failed",
                err,
            )
        })?;

        Ok(dto_node_ui_save)
    }

    pub async fn delete(
        db: &DbConn,
        dto_node_ui_model: dto_node_ui::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dto_node_ui::Entity::delete(dto_node_ui_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi delete failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dto_node_ui_model_list: Vec<dto_node_ui::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dto_node_ui_model_list.iter().map(|dto_node_ui_model| {
            dto_node_ui_model.id_dto_node_ui.clone()
        }).collect::<Vec<String>>();
        let delete_result = dto_node_ui::Entity::delete_many()
            .filter(Expr::col(dto_node_ui::Column::IdDtoNodeUi).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dto_node_ui::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi delete_all failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DtoNodeUiQuery;

impl DtoNodeUiQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dto_node_ui::Model, TcdtServiceError> {
        let dto_node_ui_entity =
            dto_node_ui::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DtoNodeUi find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DtoNodeUi cant not find data"))?;
        Ok(dto_node_ui_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dto_node_ui::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idDtoNodeUi", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dto_node_ui::Entity::default(),
            aq_condition,
            "dto_node_ui",
            "DtoNodeUi",
        )?;

        let dto_node_uis = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi find_by_ids failed", err)
            })?;

        Ok(dto_node_uis)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dto_node_ui::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dto_node_ui::Entity::default(),
            aq_condition,
            "dto_node_ui",
            "DtoNodeUi",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dto_node_ui::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi num_items failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi num_items failed", err)
            })?;
        let dto_node_uis = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi fetch_page failed", err)
            })?;
        Ok((dto_node_uis, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dto_node_ui::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_node_ui::Entity::default(),
            aq_condition,
            "dto_node_ui",
            "DtoNodeUi",
        )?;

        let dto_node_uis = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi find_collection_by_condition failed", err)
            })?;

        Ok(dto_node_uis)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dto_node_ui::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_node_ui::Entity::default(),
            aq_condition,
            "dto_node_ui",
            "DtoNodeUi",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_node_ui::Entity::default(),
            aq_condition,
            "dto_node_ui",
            "DtoNodeUi",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dto_node_ui::Entity::default(),
            aq_condition,
            "dto_node_ui",
            "DtoNodeUi",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DtoNodeUi exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DtoNodeUi exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
