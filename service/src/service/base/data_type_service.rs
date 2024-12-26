use crate::{
    common::{
        aq::*,
    },
    dto::po::base::data_type_po::DataTypePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::data_type;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct DataTypeMutation;

impl DataTypeMutation {
    pub async fn create(
        db: &DbConn,
        data_type_model: data_type::Model,
    ) -> Result<data_type::Model, TcdtServiceError> {
        let mut data_type_active_model = data_type::convert_model_to_active_model(data_type_model);
        let id = generate_id();
        data_type_active_model.id_data_type = Set(id.clone());
        let _ = data_type::Entity::insert(data_type_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DataType insert failed",
                err,
            )
        })?;

        let data_type_save = data_type::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "DataType insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("DataType insert after cannot find entity"))?;
        Ok(data_type_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        data_type_model: data_type::Model,
    ) -> Result<data_type::Model, TcdtServiceError> {
        let id = data_type_model.id_data_type.clone();

        let data_type_persist_model: data_type::ActiveModel = data_type::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "DataType update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("DataType update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut data_type_active_model = data_type::convert_model_to_active_model(data_type_model);

        let data_type_save = data_type_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " DataType update failed",
                err,
            )
        })?;

        Ok(data_type_save)
    }

    pub async fn delete(
        db: &DbConn,
        data_type_model: data_type::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = data_type::Entity::delete(data_type_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DataType delete failed");
                TcdtServiceError::build_internal_msg_error("DataType delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        data_type_model_list: Vec<data_type::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = data_type_model_list.iter().map(|data_type_model| {
            data_type_model.id_data_type.clone()
        }).collect::<Vec<String>>();
        let delete_result = data_type::Entity::delete_many()
            .filter(Expr::col(data_type::Column::IdDataType).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("DataType batch_delete failed");
                TcdtServiceError::build_internal_msg_error("DataType batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = data_type::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("DataType delete_all failed");
                TcdtServiceError::build_internal_msg_error("DataType delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct DataTypeQuery;

impl DataTypeQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<data_type::Model, TcdtServiceError> {
        let data_type_entity =
            data_type::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("DataType find_by_id failed");
                TcdtServiceError::build_internal_msg_error("DataType find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("DataType cant not find data"))?;
        Ok(data_type_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<data_type::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idDataType", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            data_type::Entity::default(),
            aq_condition,
            "dd_data_type",
            "DataType",
        )?;

        let data_types = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DataType find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("DataType find_by_ids failed", err)
            })?;

        Ok(data_types)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<data_type::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            data_type::Entity::default(),
            aq_condition,
            "dd_data_type",
            "DataType",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<data_type::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("DataType num_items failed");
                TcdtServiceError::build_internal_msg_error("DataType num_items failed", err)
            })?;
        let data_types = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("DataType fetch_page failed");
                TcdtServiceError::build_internal_msg_error("DataType fetch_page failed", err)
            })?;
        Ok((data_types, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<data_type::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            data_type::Entity::default(),
            aq_condition,
            "dd_data_type",
            "DataType",
        )?;

        let data_types = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("DataType find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DataType find_collection_by_condition failed", err)
            })?;

        Ok(data_types)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<data_type::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            data_type::Entity::default(),
            aq_condition,
            "dd_data_type",
            "DataType",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("DataType find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DataType find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            data_type::Entity::default(),
            aq_condition,
            "dd_data_type",
            "DataType",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DataType count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DataType count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            data_type::Entity::default(),
            aq_condition,
            "dd_data_type",
            "DataType",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("DataType exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("DataType exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
