use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "dto_entity_collection")]
pub struct Model {
    #[sea_orm(primary_key, comment = "DTO实体集")]
    pub id_dto_entity_collection: String,
    /// 代码包名:
    #[sea_orm(comment = "代码包名")]
    pub package_name: Option<String>,
    /// 显示名称:
    #[sea_orm(comment = "显示名称")]
    pub display_name: Option<String>,
    /// 主DTO实体集id:
    #[sea_orm(comment = "主DTO实体集id")]
    pub id_main_dto_entity: Option<String>,
    /// DTO模块id:
    #[sea_orm(comment = "DTO模块id")]
    pub id_dto_module: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
/// DtoModuleLinked
pub struct DtoModuleLinked;
impl Linked for DtoModuleLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_module::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_module::Entity)
            .from(Column::IdDtoModule)
            .to(super::dto_module::Column::IdDtoModule)
            .into()]
    }
}
/// DtoEnumAssociatesLinked
pub struct DtoEnumAssociatesLinked;
impl Linked for DtoEnumAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum_associate::Entity)
            .from(Column::IdDtoEntityCollection)
            .to(super::dto_enum_associate::Column::IdDtoEntityCollection)
            .into()]
    }
}
/// DeAssociatesLinked
pub struct DeAssociatesLinked;
impl Linked for DeAssociatesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity_associate::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity_associate::Entity)
            .from(Column::IdDtoEntityCollection)
            .to(super::dto_entity_associate::Column::IdDtoEntityCollection)
            .into()]
    }
}
/// DtoEnumsLinked
pub struct DtoEnumsLinked;
impl Linked for DtoEnumsLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_enum::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_enum::Entity)
            .from(Column::IdDtoEntityCollection)
            .to(super::dto_enum::Column::IdDtoEntityCollection)
            .into()]
    }
}
/// DtoNodeUisLinked
pub struct DtoNodeUisLinked;
impl Linked for DtoNodeUisLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_node_ui::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_node_ui::Entity)
            .from(Column::IdDtoEntityCollection)
            .to(super::dto_node_ui::Column::IdDtoEntityCollection)
            .into()]
    }
}
/// DtoEntitiesLinked
pub struct DtoEntitiesLinked;
impl Linked for DtoEntitiesLinked {
    type FromEntity = Entity;

    type ToEntity = super::dto_entity::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Entity::belongs_to(super::dto_entity::Entity)
            .from(Column::IdDtoEntityCollection)
            .to(super::dto_entity::Column::IdDtoEntityCollection)
            .into()]
    }
}

impl ActiveModelBehavior for ActiveModel {}