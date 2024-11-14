use std::collections::HashMap;

use crate::{
    common::aq_const::DO_UNCHANGE,
    dto::vo::base::data_type_vo::DataTypeVO,
};
use ::entity::entity::data_type::{
    Column as DataTypeColumn, Entity as DataTypeEntity, Model as DataTypeModel,
};
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, DbConn, DynIden, EntityTrait, Order, QueryFilter, QueryOrder};
use tcdt_common::tcdt_service_error::TcdtServiceError;

pub struct DataTypeExtQuery;

impl DataTypeExtQuery {
    pub async fn find_and_make_map_by_project_id(
        db: &DbConn,
        id_project: String,
    ) -> Result<HashMap<String, DataTypeVO>, TcdtServiceError> {
        let data_type_entities = DataTypeEntity::find()
            .filter(DataTypeColumn::IdProject.eq(id_project))
            .order_by(
                Expr::col(ColumnRef::Column(DynIden::new(Alias::new("sn")))),
                Order::Asc,
            )
            .all(db)
            .await
            .map_err(|err| {
                log::error!("data_type find failed");
                TcdtServiceError::build_internal_msg_error("data_type find failed", err)
            })?;
        let data_type_vos: Vec<DataTypeVO> = data_type_entities
            .iter()
            .map(|data_type_entity| convert_to_vo(data_type_entity))
            .collect();
        let mut result: HashMap<String, DataTypeVO> = HashMap::new();
        for data_type_vo in data_type_vos {
            result.insert(data_type_vo.id_data_type.clone(), data_type_vo.clone());
        }
        Ok(result)
    }
    pub async fn find_and_make_map_from_preset(
        db: &DbConn,
    ) -> Result<HashMap<String, DataTypeVO>, TcdtServiceError> {
        let data_type_entities = DataTypeEntity::find()
            .filter(DataTypeColumn::IdProject.is_null())
            .all(db)
            .await
            .map_err(|err| {
                log::error!("data_type find failed");
                TcdtServiceError::build_internal_msg_error("data_type find failed", err)
            })?;
        let data_type_vos: Vec<DataTypeVO> = data_type_entities
            .iter()
            .map(|data_type_entity| convert_to_vo(data_type_entity))
            .collect();
        let mut result: HashMap<String, DataTypeVO> = HashMap::new();
        for data_type_vo in data_type_vos {
            result.insert(data_type_vo.id_data_type.clone(), data_type_vo.clone());
        }
        Ok(result)
    }
}

fn convert_to_vo(data_type_entity: &DataTypeModel) -> DataTypeVO {
    DataTypeVO {
        action: DO_UNCHANGE,
        id_data_type: data_type_entity.id_data_type.clone(),
        code: data_type_entity.code.clone(),
        display_name: data_type_entity.display_name.clone(),
        note: data_type_entity.note.clone(),
        sn: data_type_entity.sn.clone(),
        len: data_type_entity.len.clone(),
        pcs: data_type_entity.pcs.clone(),
        column_type: data_type_entity.column_type.clone(),
        object_type: data_type_entity.object_type.clone(),
        object_type_package: data_type_entity.object_type_package.clone(),
        ext1: data_type_entity.ext1.clone(),
        ext2: data_type_entity.ext2.clone(),
        ext3: data_type_entity.ext3.clone(),
        ext4: data_type_entity.ext4.clone(),
        ext5: data_type_entity.ext5.clone(),
        ext6: data_type_entity.ext6.clone(),
        default_value: data_type_entity.default_value.clone(),
        fg_mandatory: data_type_entity.fg_mandatory.clone(),
        type_script_type: data_type_entity.type_script_type.clone(),
        web_input_type: data_type_entity.web_input_type.clone(),
        id_project: data_type_entity.id_project.clone(),
        fg_preset: data_type_entity.fg_preset.clone(),
        project: None,
    }
}
