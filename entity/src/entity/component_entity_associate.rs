use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dd_component_entity_associate")]
pub struct Model {
    #[sea_orm(primary_key, comment = "组件关系")]
    pub id_component_entity_associate: String,
    /// 下级实体包名:
    #[sea_orm(comment = "下级实体包名")]
    pub down_package_name: Option<String>,
    /// 上级实体包名:
    #[sea_orm(comment = "上级实体包名")]
    pub up_package_name: Option<String>,
    /// 是否agg关系连线:
    #[sea_orm(comment = "是否agg关系连线")]
    pub fg_agg_asso: Option<bool>,
    /// id:
    #[sea_orm(comment = "id")]
    pub id_entity_associate: Option<String>,
    /// 组件id:
    #[sea_orm(comment = "组件id")]
    pub id_component: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// ComponentLinked
pub struct ComponentLinked;
impl Linked for ComponentLinked {
    type FromEntity = Entity;

    type ToEntity = super::component::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::component::Entity)
            .from(Column::IdComponent)
            .to(super::component::Column::IdComponent)
            .into()]
    }
}
/// EntityAssociateLinked
pub struct EntityAssociateLinked;
impl Linked for EntityAssociateLinked {
    type FromEntity = Entity;

    type ToEntity = super::entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::entity_associate::Entity)
            .from(Column::IdEntityAssociate)
            .to(super::entity_associate::Column::IdEntityAssociate)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}