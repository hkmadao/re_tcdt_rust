use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_entity_associate")]
pub struct Model {
    #[sea_orm(primary_key, comment = "关系连线")]
    pub id_entity_associate: String,
    /// 两个实体多条连线时，连线的序号:
    #[sea_orm(comment = "两个实体多条连线时，连线的序号")]
    pub group_order: Option<i32>,
    /// 上级关系:
    #[sea_orm(comment = "上级关系")]
    pub up_associate_type: Option<String>,
    /// 下级关系:
    #[sea_orm(comment = "下级关系")]
    pub down_associate_type: Option<String>,
    /// 下级实体属性名称:
    #[sea_orm(comment = "下级实体属性名称")]
    pub down_attribute_name: Option<String>,
    /// 下级实体属性显示名称:
    #[sea_orm(comment = "下级实体属性显示名称")]
    pub down_attribute_display_name: Option<String>,
    /// 引用实体属性:
    #[sea_orm(comment = "引用实体属性")]
    pub ref_attribute_name: Option<String>,
    /// 引用实体属性显示名称:
    #[sea_orm(comment = "引用实体属性显示名称")]
    pub ref_attribute_display_name: Option<String>,
    /// 外键字段名称:
    #[sea_orm(comment = "外键字段名称")]
    pub fk_column_name: Option<String>,
    /// 外键属性:
    #[sea_orm(comment = "外键属性")]
    pub fk_attribute_name: Option<String>,
    /// 外键属性显示名称:
    #[sea_orm(comment = "外键属性显示名称")]
    pub fk_attribute_display_name: Option<String>,
    /// 是否建立物理外键:
    #[sea_orm(comment = "是否建立物理外键")]
    pub fg_foreign_key: Option<bool>,
    /// 下级实体排序:
    #[sea_orm(comment = "下级实体排序")]
    pub down_order_str: Option<String>,
    /// 批量获取下级实体数量:
    #[sea_orm(comment = "批量获取下级实体数量")]
    pub down_batch_size: Option<i32>,
    /// 实体集id:
    #[sea_orm(comment = "实体集id")]
    pub id_entity_collection: Option<String>,
    /// 上级实体id:
    #[sea_orm(comment = "上级实体id")]
    pub id_up: Option<String>,
    /// 下级实体id:
    #[sea_orm(comment = "下级实体id")]
    pub id_down: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// EntityCollectionLinked
pub struct EntityCollectionLinked;
impl Linked for EntityCollectionLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_collection::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_collection::Entity)
            .from(Column::IdEntityCollection)
            .to(super::entity_collection::Column::IdEntityCollection)
            .into()]
    }
}
/// UpEntityLinked
pub struct UpEntityLinked;
impl Linked for UpEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdUp)
            .to(super::dd_entity::Column::IdEntity)
            .into()]
    }
}
/// DownEntityLinked
pub struct DownEntityLinked;
impl Linked for DownEntityLinked {
    type FromEntity = Entity;

    type ToEntity = super::dd_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dd_entity::Entity)
            .from(Column::IdDown)
            .to(super::dd_entity::Column::IdEntity)
            .into()]
    }
}
/// ComponentEntityAssociatesLinked
pub struct ComponentEntityAssociatesLinked;
impl Linked for ComponentEntityAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::component_entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component_entity_associate::Entity)
            .from(Column::IdEntityAssociate)
            .to(super::component_entity_associate::Column::IdEntityAssociate)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}