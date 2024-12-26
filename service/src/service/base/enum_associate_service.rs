use crate::{
    common::{
        aq::*,
    },
    dto::po::base::enum_associate_po::EnumAssociatePO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::enum_associate;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct EnumAssociateMutation;

impl EnumAssociateMutation {
    pub async fn create(
        db: &DbConn,
        enum_associate_model: enum_associate::Model,
    ) -> Result<enum_associate::Model, TcdtServiceError> {
        let mut enum_associate_active_model = enum_associate::convert_model_to_active_model(enum_associate_model);
        let id = generate_id();
        enum_associate_active_model.id_enum_associate = Set(id.clone());
        let _ = enum_associate::Entity::insert(enum_associate_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "EnumAssociate insert failed",
                err,
            )
        })?;

        let enum_associate_save = enum_associate::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "EnumAssociate insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("EnumAssociate insert after cannot find entity"))?;
        Ok(enum_associate_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        enum_associate_model: enum_associate::Model,
    ) -> Result<enum_associate::Model, TcdtServiceError> {
        let id = enum_associate_model.id_enum_associate.clone();

        let enum_associate_persist_model: enum_associate::ActiveModel = enum_associate::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "EnumAssociate update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("EnumAssociate update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut enum_associate_active_model = enum_associate::convert_model_to_active_model(enum_associate_model);

        let enum_associate_save = enum_associate_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " EnumAssociate update failed",
                err,
            )
        })?;

        Ok(enum_associate_save)
    }

    pub async fn delete(
        db: &DbConn,
        enum_associate_model: enum_associate::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = enum_associate::Entity::delete(enum_associate_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate delete failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        enum_associate_model_list: Vec<enum_associate::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = enum_associate_model_list.iter().map(|enum_associate_model| {
            enum_associate_model.id_enum_associate.clone()
        }).collect::<Vec<String>>();
        let delete_result = enum_associate::Entity::delete_many()
            .filter(Expr::col(enum_associate::Column::IdEnumAssociate).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate batch_delete failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = enum_associate::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate delete_all failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct EnumAssociateQuery;

impl EnumAssociateQuery {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<enum_associate::Model, TcdtServiceError> {
        let enum_associate_entity =
            enum_associate::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("EnumAssociate find_by_id failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("EnumAssociate cant not find data"))?;
        Ok(enum_associate_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<enum_associate::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("idEnumAssociate", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let enum_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_by_ids failed", err)
            })?;

        Ok(enum_associates)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<enum_associate::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<enum_associate::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("EnumAssociate num_items failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate num_items failed", err)
            })?;
        let enum_associates = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate fetch_page failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate fetch_page failed", err)
            })?;
        Ok((enum_associates, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<enum_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let enum_associates = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_collection_by_condition failed", err)
            })?;

        Ok(enum_associates)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<enum_associate::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            enum_associate::Entity::default(),
            aq_condition,
            "dd_enum_associate",
            "EnumAssociate",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("EnumAssociate exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("EnumAssociate exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
