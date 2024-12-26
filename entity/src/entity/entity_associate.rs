use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dd_entity_associate")]
pub struct Model {
    #[sea_orm(primary_key, comment = "实体连线id")]
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

pub fn convert_model_to_active_model(entity_associate_model: Model) -> ActiveModel {
    ActiveModel {
        id_entity_associate: Set(entity_associate_model.id_entity_associate.clone()),
        group_order: Set(entity_associate_model.group_order.clone()),
        up_associate_type: Set(entity_associate_model.up_associate_type.clone()),
        down_associate_type: Set(entity_associate_model.down_associate_type.clone()),
        down_attribute_name: Set(entity_associate_model.down_attribute_name.clone()),
        down_attribute_display_name: Set(entity_associate_model.down_attribute_display_name.clone()),
        ref_attribute_name: Set(entity_associate_model.ref_attribute_name.clone()),
        ref_attribute_display_name: Set(entity_associate_model.ref_attribute_display_name.clone()),
        fk_column_name: Set(entity_associate_model.fk_column_name.clone()),
        fk_attribute_name: Set(entity_associate_model.fk_attribute_name.clone()),
        fk_attribute_display_name: Set(entity_associate_model.fk_attribute_display_name.clone()),
        fg_foreign_key: Set(entity_associate_model.fg_foreign_key.clone()),
        down_order_str: Set(entity_associate_model.down_order_str.clone()),
        down_batch_size: Set(entity_associate_model.down_batch_size.clone()),
        id_entity_collection: Set(entity_associate_model.id_entity_collection.clone()),
        id_up: Set(entity_associate_model.id_up.clone()),
        id_down: Set(entity_associate_model.id_down.clone()),
    }
}