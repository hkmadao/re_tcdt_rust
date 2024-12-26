use crate::{
    common::{
        aq::*,
    },
    dto::po::base::dd_enum_po::DdEnumPO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::dd_enum;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DdEnumMutation;

impl DdEnumMutation {
    pub async fn create(
        db: &DbConn,
        dd_enum_model: dd_enum::Model,
    ) -> Result<dd_enum::Model, TcdtServiceError> {
        let mut dd_enum_active_model = dd_enum::convert_model_to_active_model(dd_enum_model);
        let id = generate_id();
        dd_enum_active_model.id_enum = Set(id.clone());
        let _ = dd_enum::Entity::insert(dd_enum_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DdEnum insert failed",
                err,
            )
        })?;

        let dd_enum_save = dd_enum::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DdEnum insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DdEnum insert after cannot find entity"))?;
        Ok(dd_enum_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        dd_enum_model: dd_enum::Model,
    ) -> Result<dd_enum::Model, TcdtServiceError> {
        let id = dd_enum_model.id_enum.clone();

        let dd_enum_persist_model: dd_enum::ActiveModel = dd_enum::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DdEnum update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DdEnum update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut dd_enum_active_model = dd_enum::convert_model_to_active_model(dd_enum_model);

        let dd_enum_save = dd_enum_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DdEnum update failed",
                err,
            )
        })?;

        Ok(dd_enum_save)
    }

    pub async fn delete(
        db: &DbConn,
        dd_enum_model: dd_enum::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = dd_enum::Entity::delete(dd_enum_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum delete failed");
                TcdtServiceError::build_internal_msg_error("DdEnum delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        dd_enum_model_list: Vec<dd_enum::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = dd_enum_model_list.iter().map(|dd_enum_model| {
            dd_enum_model.id_enum.clone()
        }).collect::<Vec<String>>();
        let delete_result = dd_enum::Entity::delete_many()
            .filter(Expr::col(dd_enum::Column::IdEnum).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DdEnum batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = dd_enum::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum delete_all failed");
                TcdtServiceError::build_internal_msg_error("DdEnum delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DdEnumQuery;

impl DdEnumQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<dd_enum::Model, TcdtServiceError> {
        let dd_enum_entity =
            dd_enum::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DdEnum find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DdEnum cant not find data"))?;
        Ok(dd_enum_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<dd_enum::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idEnum", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_condition,
            "dd_enum",
            "DdEnum",
        )?;

        let dd_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_by_ids failed", err)
            })?;

        Ok(dd_enums)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<dd_enum::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_condition,
            "dd_enum",
            "DdEnum",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<dd_enum::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DdEnum num_items failed");
                TcdtServiceError::build_internal_msg_error("DdEnum num_items failed", err)
            })?;
        let dd_enums = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DdEnum fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DdEnum fetch_page failed", err)
            })?;
        Ok((dd_enums, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<dd_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_condition,
            "dd_enum",
            "DdEnum",
        )?;

        let dd_enums = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_collection_by_condition failed", err)
            })?;

        Ok(dd_enums)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<dd_enum::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_condition,
            "dd_enum",
            "DdEnum",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_condition,
            "dd_enum",
            "DdEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            dd_enum::Entity::default(),
            aq_condition,
            "dd_enum",
            "DdEnum",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DdEnum exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DdEnum exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
