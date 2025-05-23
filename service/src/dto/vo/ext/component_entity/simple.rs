use ::entity::entity::{component_entity, dd_entity};
use sea_orm::DbConn;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_service_error::TcdtServiceError;
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_component_entity: String,
    /// 虚拟实体标志:组合实体组件下的组件实体都是虚拟虚拟实体
    #[serde(default)]
    pub fg_virtual: Option<bool>,
    #[serde(default)]
    pub id_entity: Option<String>,
    #[serde(default)]
    pub id_component: Option<String>,
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dd_entity: Option<DdEntityVO>,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_entity: String,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 类名
    #[serde(default)]
    pub class_name: Option<String>,
    /// 表名
    #[serde(default)]
    pub table_name: Option<String>,
    /// 主属性code
    #[serde(default)]
    pub pk_attribute_code: Option<String>,
    /// 主属性名称
    #[serde(default)]
    pub pk_attribute_name: Option<String>,
    /// 主属性类型名称
    #[serde(default)]
    pub pk_attribute_type_name: Option<String>,
}
/**/
