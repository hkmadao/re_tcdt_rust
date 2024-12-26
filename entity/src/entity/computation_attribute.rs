use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_computation_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "计算属性id")]
    pub id_computation_attribute: String,
    /// 属性名称:
    #[sea_orm(comment = "属性名称")]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 数据长度:
    #[sea_orm(comment = "数据长度")]
    pub len: Option<i32>,
    /// 是否必填:
    #[sea_orm(comment = "是否必填")]
    pub fg_mandatory: Option<bool>,
    /// 默认值:
    #[sea_orm(comment = "默认值")]
    pub default_value: Option<String>,
    /// 精度:
    #[sea_orm(comment = "精度")]
    pub pcs: Option<i32>,
    /// 序号:
    #[sea_orm(comment = "序号")]
    pub sn: Option<i32>,
    /// 所在组件实体id:
    #[sea_orm(comment = "所在组件实体id")]
    pub id_component_entity: Option<String>,
    /// 属性类型:
    #[sea_orm(comment = "属性类型")]
    pub id_attribute_type: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ComponentEntityLinked
pub struct ComponentEntityLinked;
impl Linked for ComponentEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_entity::Entity)
            .from(Column::IdComponentEntity)
            .to(super::component_entity::Column::IdComponentEntity)
            .into()]
    }
}
/// AttributeTypeLinked
pub struct AttributeTypeLinked;
impl Linked for AttributeTypeLinked {
    type FromEntity = Entity;

    type ToEntity = super::data_type::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::data_type::Entity)
            .from(Column::IdAttributeType)
            .to(super::data_type::Column::IdDataType)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub fn convert_model_to_active_model(computation_attribute_model: Model) -> ActiveModel {
    ActiveModel {
        id_computation_attribute: Set(computation_attribute_model.id_computation_attribute.clone()),
        attribute_name: Set(computation_attribute_model.attribute_name.clone()),
        display_name: Set(computation_attribute_model.display_name.clone()),
        len: Set(computation_attribute_model.len.clone()),
        fg_mandatory: Set(computation_attribute_model.fg_mandatory.clone()),
        default_value: Set(computation_attribute_model.default_value.clone()),
        pcs: Set(computation_attribute_model.pcs.clone()),
        sn: Set(computation_attribute_model.sn.clone()),
        id_component_entity: Set(computation_attribute_model.id_component_entity.clone()),
        id_attribute_type: Set(computation_attribute_model.id_attribute_type.clone()),
    }
}