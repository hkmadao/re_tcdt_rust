use ::entity::entity::{
    common_attribute, data_type, dd_entity, dd_enum, dto_computation_attribute, dto_entity,
    dto_entity_associate, dto_entity_attribute, dto_entity_collection, dto_enum,
    dto_enum_associate, dto_enum_attribute, dto_module, dto_node_ui, entity_attribute, project,
    sub_project,
};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, ColumnRef};
use sea_orm::ModelTrait;
use sea_orm::{DbConn, DynIden, Order, QueryOrder};
use serde::{Deserialize, Serialize};
use tcdt_common::tcdt_trait::TcdtViewObjectTrait;
use tcdt_macro::ViewObectConvert;
use tcdt_common::tcdt_service_error::TcdtServiceError;
/**DTO实体集 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityCollectionVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity_collection: String,
    /// 名称
    #[serde(default)]
    pub package_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 主DTO实体集id
    #[serde(default)]
    pub id_main_dto_entity: Option<String>,
    /// DTO模块id
    #[serde(default)]
    pub id_dto_module: Option<String>,
    ///
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub dto_module: Option<DtoModuleVO>,
    /// 公共属性
    #[serde(default)]
    #[tcdt_vo(vo_array, ignore)]
    pub common_attributes: Vec<CommonAttributeVO>,
    /// DTO实体信息
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub dto_entities: Vec<DtoEntityVO>,
    /// 系统数据类型
    #[serde(default)]
    #[tcdt_vo(vo_array, ignore)]
    pub sys_data_types: Vec<DataTypeVO>,
    /// DTO关系连线
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub de_associates: Vec<DtoEntityAssociateVO>,
    /// DTO实体集ui信息
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub dto_node_uis: Vec<DtoNodeUiVO>,
    /// DTO实体枚举关系
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub dto_enum_associates: Vec<DtoEnumAssociateVO>,
    /// DTO枚举实体
    #[serde(default)]
    #[tcdt_vo(vo_array)]
    pub dto_enums: Vec<DtoEnumVO>,
}
/**DTO关系连线 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity_associate: String,
    /// 两个实体多条连线时，连线的序号
    #[serde(default)]
    pub group_order: Option<i32>,
    /// 上级关系
    #[serde(default)]
    pub up_associate_type: Option<String>,
    /// 下级关系
    #[serde(default)]
    pub down_associate_type: Option<String>,
    /// 下级实体属性名称
    #[serde(default)]
    pub down_attribute_name: Option<String>,
    /// 下级实体属性显示名称
    #[serde(default)]
    pub down_attribute_display_name: Option<String>,
    /// 引用实体属性
    #[serde(default)]
    pub ref_attribute_name: Option<String>,
    /// 引用实体属性显示名称
    #[serde(default)]
    pub ref_attribute_display_name: Option<String>,
    /// 外键字段名称
    #[serde(default)]
    pub fk_column_name: Option<String>,
    /// 外键属性
    #[serde(default)]
    pub fk_attribute_name: Option<String>,
    /// 外键属性显示名称
    #[serde(default)]
    pub fk_attribute_display_name: Option<String>,
    /// 上级实体id
    #[serde(default)]
    pub id_up: Option<String>,
    /// 下级实体id
    #[serde(default)]
    pub id_down: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
}
/**枚举实体 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DdEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_enum: String,
    /// 名称
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型
    #[serde(default)]
    pub enum_value_type: Option<String>,
}
/**DTO枚举属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_enum_attribute: String,
    /// 枚举属性显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举属性编码
    #[serde(default)]
    pub code: Option<String>,
    /// 枚举值
    #[serde(default)]
    pub enum_value: Option<String>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 引用id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// DTO枚举id
    #[serde(default)]
    pub id_dto_enum: Option<String>,
}
/**DTO实体集ui信息 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoNodeUiVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_node_ui: String,
    /// x坐标
    #[serde(default)]
    pub x: Option<i32>,
    /// y坐标
    #[serde(default)]
    pub y: Option<i32>,
    /// 宽度
    #[serde(default)]
    pub width: Option<i32>,
    /// 高度
    #[serde(default)]
    pub height: Option<i32>,
    /// 元素id
    #[serde(default)]
    pub id_element: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
}
/**实体信息 */
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
/**子项目 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct SubProjectVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_sub_project: String,
    /// 名称
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 子系统路径
    #[serde(default)]
    pub path: Option<String>,
    /// 项目id
    #[serde(default)]
    pub id_project: Option<String>,
    /// 项目
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub project: Option<ProjectVO>,
}
/**属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct EntityAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_attribute: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 字段名称
    #[serde(default)]
    pub column_name: Option<String>,
    /// 是否主键
    #[serde(default)]
    pub fg_primary_key: Option<bool>,
    /// 是否必填
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 数据长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 分类
    #[serde(default)]
    pub category: Option<String>,
}
/**DTO枚举实体 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_enum: String,
    /// 名称
    #[serde(default)]
    pub class_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 枚举值的类型
    #[serde(default)]
    pub enum_value_type: Option<String>,
    /// 引用id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// 引用枚举
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_enum: Option<DdEnumVO>,
    /// DTO枚举属性
    #[serde(default)]
    #[tcdt_vo(vo_array, order_by = "sn asc")]
    pub dto_enum_attributes: Vec<DtoEnumAttributeVO>,
}
/**数据类型 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DataTypeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_data_type: String,
    /// 数据类型编码
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 序列号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 字段类型
    #[serde(default)]
    pub column_type: Option<String>,
    /// 对象类型名称
    #[serde(default)]
    pub object_type: Option<String>,
    /// 对象类型包名
    #[serde(default)]
    pub object_type_package: Option<String>,
    /// 扩展属性1
    #[serde(default)]
    pub ext1: Option<String>,
    /// 扩展属性2
    #[serde(default)]
    pub ext2: Option<String>,
    /// 扩展属性3
    #[serde(default)]
    pub ext3: Option<String>,
    /// 扩展属性4
    #[serde(default)]
    pub ext4: Option<String>,
    /// 扩展属性5
    #[serde(default)]
    pub ext5: Option<String>,
    /// 扩展属性6
    #[serde(default)]
    pub ext6: Option<String>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 必填标志
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// TypeScript类型
    #[serde(default)]
    pub type_script_type: Option<String>,
    /// HTML5输入框类型
    #[serde(default)]
    pub web_input_type: Option<String>,
}
/**DTO实体属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity_attribute: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 字段名称
    #[serde(default)]
    pub column_name: Option<String>,
    /// 是否主键
    #[serde(default)]
    pub fg_primary_key: Option<bool>,
    /// 是否必填
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 数据长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 类型
    #[serde(default)]
    pub category: Option<String>,
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    /// 引用属性id
    #[serde(default)]
    pub id_ref_attribute: Option<String>,
    /// 引用属性
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_attribute: Option<EntityAttributeVO>,
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/**DTO实体信息 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEntityVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_entity: String,
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
    /// 引用实体id
    #[serde(default)]
    pub id_ref: Option<String>,
    /// 引用实体
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub ref_entity: Option<DdEntityVO>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
    /// DTO实体属性
    #[serde(default)]
    #[tcdt_vo(vo_array, ignore)]
    pub de_attributes: Vec<DtoEntityAttributeVO>,
    /// DTO计算属性
    #[serde(default)]
    #[tcdt_vo(vo_array, ignore)]
    pub dc_attributes: Vec<DtoComputationAttributeVO>,
}
/**DTO实体枚举关系 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoEnumAssociateVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_enum_associate: String,
    /// 两个相同实体和枚举多条连线时，连线的序号
    #[serde(default)]
    pub group_order: Option<i32>,
    /// DTO实体id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    /// DTO枚举id
    #[serde(default)]
    pub id_dto_enum: Option<String>,
    /// DTO实体属性id
    #[serde(default)]
    pub id_dto_entity_attribute: Option<String>,
    /// DTO实体集id
    #[serde(default)]
    pub id_dto_entity_collection: Option<String>,
}
/**公共属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct CommonAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_common_attribute: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 字段名称
    #[serde(default)]
    pub column_name: Option<String>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 是否必填
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 数据长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<i32>,
    /// 序号
    #[serde(default)]
    pub sn: Option<i32>,
    /// 引用属性名称
    #[serde(default)]
    pub ref_attribute_name: Option<String>,
    /// 引用属性显示名称
    #[serde(default)]
    pub ref_display_name: Option<String>,
    /// 属性类别
    #[serde(default)]
    pub category: Option<String>,
}
/**DTO计算属性 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoComputationAttributeVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_computation_attribute: String,
    /// 属性名称
    #[serde(default)]
    pub attribute_name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 数据长度
    #[serde(default)]
    pub len: Option<i32>,
    /// 是否必填
    #[serde(default)]
    pub fg_mandatory: Option<bool>,
    /// 默认值
    #[serde(default)]
    pub default_value: Option<String>,
    /// 精度
    #[serde(default)]
    pub pcs: Option<String>,
    /// 序号
    #[serde(default)]
    pub sn: Option<String>,
    ///  DTO实体信息id
    #[serde(default)]
    pub id_dto_entity: Option<String>,
    #[serde(default)]
    pub id_attribute_type: Option<String>,
}
/**DTO模块 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct DtoModuleVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_dto_module: String,
    /// DTO模块名称
    #[serde(default)]
    pub name: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// DTO模块空间路径
    #[serde(default)]
    pub path: Option<String>,
    /// 子项目id
    #[serde(default)]
    pub id_sub_project: Option<String>,
    /// 子项目
    #[serde(default)]
    #[tcdt_vo(vo_ref)]
    pub sub_project: Option<SubProjectVO>,
}
/**项目 */
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, ViewObectConvert)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVO {
    #[tcdt_vo(ignore)]
    #[serde(default)]
    pub action: i32,
    #[tcdt_vo(vo_primary_key)]
    #[serde(default)]
    pub id_project: String,
    /// 项目编号
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 系统路径
    #[serde(default)]
    pub path: Option<String>,
    /// 项目模板编号
    #[serde(default)]
    pub template_code: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 文件名样式
    #[serde(default)]
    pub file_name_type: Option<String>,
}
