use crate::{
    common::{
        aq::*,
    },
    dto::po::base::{{ rootInfo.snakeCaseName }}_po::{{ rootInfo.pascalCaseName }}PO,
    util::dyn_query::make_select_by_condition,
};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use ::entity::entity::{{ rootInfo.snakeCaseName }};
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use crate::util::id_util::generate_id;

pub struct {{ rootInfo.pascalCaseName }}Mutation;

impl {{ rootInfo.pascalCaseName }}Mutation {
    pub async fn create(
        db: &DbConn,
        {{ rootInfo.snakeCaseName }}_model: {{ rootInfo.snakeCaseName }}::Model,
    ) -> Result<{{ rootInfo.snakeCaseName }}::Model, TcdtServiceError> {
        let mut {{ rootInfo.snakeCaseName }}_active_model = {{ rootInfo.snakeCaseName }}::convert_model_to_active_model({{ rootInfo.snakeCaseName }}_model);
        let id = generate_id();
        {{ rootInfo.snakeCaseName }}_active_model.{{ rootInfo.pkAttributeInfo.snakeCaseName }} = Set(id.clone());
        let _ = {{ rootInfo.snakeCaseName }}::Entity::insert({{ rootInfo.snakeCaseName }}_active_model).exec(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "{{ rootInfo.pascalCaseName }} insert failed",
                err,
            )
        })?;

        let {{ rootInfo.snakeCaseName }}_save = {{ rootInfo.snakeCaseName }}::Entity::find_by_id(id).one(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                "{{ rootInfo.pascalCaseName }} insert after find failed",
                err,
            )
        })?
            .ok_or(TcdtServiceError::build_internal_msg("{{ rootInfo.pascalCaseName }} insert after cannot find entity"))?;
        Ok({{ rootInfo.snakeCaseName }}_save)
    }

    pub async fn update_by_id(
        db: &DbConn,
        {{ rootInfo.snakeCaseName }}_model: {{ rootInfo.snakeCaseName }}::Model,
    ) -> Result<{{ rootInfo.snakeCaseName }}::Model, TcdtServiceError> {
        let id = {{ rootInfo.snakeCaseName }}_model.{{ rootInfo.pkAttributeInfo.snakeCaseName }}.clone();

        let {{ rootInfo.snakeCaseName }}_persist_model: {{ rootInfo.snakeCaseName }}::ActiveModel = {{ rootInfo.snakeCaseName }}::Entity::find_by_id(&id)
            .one(db)
            .await
            .map_err(|err| {
                TcdtServiceError::build_internal_msg_error(
                    "{{ rootInfo.pascalCaseName }} update before find_by_id failed",
                    err,
                )
            })?
            .ok_or(TcdtServiceError::build_internal_msg(&format!("{{ rootInfo.pascalCaseName }} update before cannot find entity [{}].", stringify!(#entity_name_ident))))?
            .into_active_model();

        let mut {{ rootInfo.snakeCaseName }}_active_model = {{ rootInfo.snakeCaseName }}::convert_model_to_active_model({{ rootInfo.snakeCaseName }}_model);

        let {{ rootInfo.snakeCaseName }}_save = {{ rootInfo.snakeCaseName }}_active_model
            .update(db)
            .await.map_err(|err| {
            TcdtServiceError::build_internal_msg_error(
                " {{ rootInfo.pascalCaseName }} update failed",
                err,
            )
        })?;

        Ok({{ rootInfo.snakeCaseName }}_save)
    }

    pub async fn delete(
        db: &DbConn,
        {{ rootInfo.snakeCaseName }}_model: {{ rootInfo.snakeCaseName }}::Model,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let delete_result = {{ rootInfo.snakeCaseName }}::Entity::delete({{ rootInfo.snakeCaseName }}_model.into_active_model())
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} delete failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} delete_all failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn batch_delete(
        db: &DbConn,
        {{ rootInfo.snakeCaseName }}_model_list: Vec<{{ rootInfo.snakeCaseName }}::Model>,
    ) -> Result<DeleteResult, TcdtServiceError> {
        let id_list = {{ rootInfo.snakeCaseName }}_model_list.iter().map(|{{ rootInfo.snakeCaseName }}_model| {
            {{ rootInfo.snakeCaseName }}_model.{{ rootInfo.pkAttributeInfo.snakeCaseName }}.clone()
        }).collect::<Vec<String>>();
        let delete_result = {{ rootInfo.snakeCaseName }}::Entity::delete_many()
            .filter(Expr::col({{ rootInfo.snakeCaseName }}::Column::{{ rootInfo.pkAttributeInfo.pascalCaseName }}).is_in(id_list))
            .exec(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} batch_delete failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} batch_delete failed", err)
            })?;
        Ok(delete_result)
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, TcdtServiceError> {
        let delete_all_result = {{ rootInfo.snakeCaseName }}::Entity::delete_many().exec(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} delete_all failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} delete_all failed", err)
            })?;
        Ok(delete_all_result)
    }
}

pub struct {{ rootInfo.pascalCaseName }}Query;

impl {{ rootInfo.pascalCaseName }}Query {
    pub async fn find_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<{{ rootInfo.snakeCaseName }}::Model, TcdtServiceError> {
        let {{ rootInfo.snakeCaseName }}_entity =
            {{ rootInfo.snakeCaseName }}::Entity::find_by_id(id)
                .one(db)
                .await.map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} find_by_id failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_by_id failed", err)
            })?
                .ok_or(TcdtServiceError::build_internal_msg("{{ rootInfo.pascalCaseName }} cant not find data"))?;
        Ok({{ rootInfo.snakeCaseName }}_entity)
    }

    pub async fn find_by_ids(
        db: &DbConn,
        ids: Vec<String>,
    ) -> Result<Vec<{{ rootInfo.snakeCaseName }}::Model>, TcdtServiceError> {
        let aq_condition = AqCondition::build_in_condition("{{ rootInfo.pkAttributeInfo.camelCaseName }}", ids
            .iter()
            .map(|id| EFilterParam::String(Some(Box::new(id.to_string()))))
            .collect());

        let sql_build = make_select_by_condition(
            {{ rootInfo.snakeCaseName }}::Entity::default(),
            aq_condition,
            "{{ rootInfo.tableName }}",
            "{{ rootInfo.pascalCaseName }}",
        )?;

        let {{ rootInfo.snakeCaseName }}s = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} find_by_ids failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_by_ids failed", err)
            })?;

        Ok({{ rootInfo.snakeCaseName }}s)
    }

    pub async fn find_page_by_page_condition(
        db: &DbConn,
        aq_page: AqPageInfoInput,
    ) -> Result<(Vec<{{ rootInfo.snakeCaseName }}::Model>, u64), TcdtServiceError> {
        let page_size = aq_page.page_size;
        let page_index = aq_page.page_index;
        let aq_condition = AqCondition {
            logic_node: aq_page.logic_node,
            orders: aq_page.orders,
        };
        let sql_build = make_select_by_condition(
            {{ rootInfo.snakeCaseName }}::Entity::default(),
            aq_condition,
            "{{ rootInfo.tableName }}",
            "{{ rootInfo.pascalCaseName }}",
        )?;

        // Setup paginator
        let paginator: Paginator<DatabaseConnection, SelectModel<{{ rootInfo.snakeCaseName }}::Model>> =
            sql_build.paginate(db, page_size);
        let num_items = paginator.num_items()
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} num_items failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} num_items failed", err)
            })?;
        let {{ rootInfo.snakeCaseName }}s = paginator.fetch_page(page_index - 1)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} fetch_page failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} fetch_page failed", err)
            })?;
        Ok(({{ rootInfo.snakeCaseName }}s, num_items))
    }

    pub async fn find_collection_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Vec<{{ rootInfo.snakeCaseName }}::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            {{ rootInfo.snakeCaseName }}::Entity::default(),
            aq_condition,
            "{{ rootInfo.tableName }}",
            "{{ rootInfo.pascalCaseName }}",
        )?;

        let {{ rootInfo.snakeCaseName }}s = sql_build.all(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} find_collection_by_condition failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_collection_by_condition failed", err)
            })?;

        Ok({{ rootInfo.snakeCaseName }}s)
    }

    pub async fn find_one_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<Option<{{ rootInfo.snakeCaseName }}::Model>, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            {{ rootInfo.snakeCaseName }}::Entity::default(),
            aq_condition,
            "{{ rootInfo.tableName }}",
            "{{ rootInfo.pascalCaseName }}",
        )?;

        let data_type = sql_build.one(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} find_one_by_condition failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} find_one_by_condition failed", err)
            })?;
        Ok(data_type)
    }

    pub async fn count_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<u64, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            {{ rootInfo.snakeCaseName }}::Entity::default(),
            aq_condition,
            "{{ rootInfo.tableName }}",
            "{{ rootInfo.pascalCaseName }}",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} count_by_condition failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} count_by_condition failed", err)
            })?;

        Ok(count)
    }

    pub async fn exists_by_condition(
        db: &DbConn,
        aq_condition: AqCondition,
    ) -> Result<bool, TcdtServiceError> {
        let sql_build = make_select_by_condition(
            {{ rootInfo.snakeCaseName }}::Entity::default(),
            aq_condition,
            "{{ rootInfo.tableName }}",
            "{{ rootInfo.pascalCaseName }}",
        )?;

        let count = sql_build.count(db)
            .await
            .map_err(|err| {
                log::error!("{{ rootInfo.pascalCaseName }} exists_by_condition failed");
                TcdtServiceError::build_internal_msg_error("{{ rootInfo.pascalCaseName }} exists_by_condition failed", err)
            })?;

        Ok(count > 0)
    }
}
