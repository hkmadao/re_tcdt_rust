use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_entity_attribute")]
pub struct Model {
    #[sea_orm(primary_key, comment = "属性")]
    pub id_attribute: String,
    /// 属性名称:
    #[sea_orm(comment = "属性名称")]
    pub attribute_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 字段名称:
    #[sea_orm(comment = "字段名称")]
    pub column_name: Option<String>,
    /// 是否主键:
    #[sea_orm(comment = "是否主键")]
    pub fg_primary_key: Option<bool>,
    /// 是否必填:
    #[sea_orm(comment = "是否必填")]
    pub fg_mandatory: Option<bool>,
    /// 默认值:
    #[sea_orm(comment = "默认值")]
    pub default_value: Option<String>,
    /// 数据长度:
    #[sea_orm(comment = "数据长度")]
    pub len: Option<i32>,
    /// 精度:
    #[sea_orm(comment = "精度")]
    pub pcs: Option<i32>,
    /// 序号:
    #[sea_orm(comment = "序号")]
    pub sn: Option<i32>,
    /// 备注:
    #[sea_orm(comment = "备注")]
    pub note: Option<String>,
    /// 分类:
    #[sea_orm(comment = "分类")]
    pub category: Option<String>,
    /// 数据类型id:
    #[sea_orm(comment = "数据类型id")]
    pub id_attribute_type: Option<String>,
    /// 实体id:
    #[sea_orm(comment = "实体id")]
    pub id_entity: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
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
/// DdEntityLinked
pub struct DdEntityLinked;
impl Linked for DdEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdEntity)
            .to(super::dd_entity::Column::IdEntity)
            .into()]
    }
}
/// EnumAssociateLinked
pub struct EnumAssociateLinked;
impl Linked for EnumAssociateLinked {
    type FromEntity = Entity;

    type ToEntity = super::enum_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::enum_associate::Entity)
            .from(Column::IdAttribute)
            .to(super::enum_associate::Column::IdAttribute)
            .into()]
    }
}
/// DeAttributesLinked
pub struct DeAttributesLinked;
impl Linked for DeAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_attribute::Entity)
            .from(Column::IdAttribute)
            .to(super::dto_entity_attribute::Column::IdRefAttribute)
            .into()]
    }
}
/// ExtAttributesLinked
pub struct ExtAttributesLinked;
impl Linked for ExtAttributesLinked {
    type FromEntity = Entity;

    type ToEntity = super::ext_attribute::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::ext_attribute::Entity)
            .from(Column::IdAttribute)
            .to(super::ext_attribute::Column::IdAttribute)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}