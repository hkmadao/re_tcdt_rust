use std::collections::HashMap;

/**
 * 主键
 */
pub const DATA_TYPE_INTERNAL_PK: &str = "InternalPK";
/**
 * 外键
 */
pub const DATA_TYPE_INTERNAL_FK: &str = "InternalFK";
/**
 * 1对多关系 子实体的引用属性
 */
pub const DATA_TYPE_REF: &str = "InternalRef";
/**
 * 1对1关系 子实体的引用属性
 */
pub const DATA_TYPE_SINGLE_REF: &str = "InternalSingleRef";
/**
 * 1对1关系 主实体的子属性
 */
pub const DATA_TYPE_SINGLE: &str = "InternalSingle";
/**
 * 1对多关系 主实体的子属性
 */
pub const DATA_TYPE_ARRAY: &str = "InternalArray";
/**
 * agg 外键
 */
pub const DATA_TYPE_AGG_FK: &str = "InternalAggFK";
/**
 * 1对多关系 子实体的引用属性
 */
pub const DATA_TYPE_AGG_REF: &str = "InternalAggRef";
/**
 * agg 1对1关系 子实体的引用属性
 */
pub const DATA_TYPE_AGG_SINGLE_REF: &str = "InternalAggSingleRef";
/**
 * agg 1对1关系 主实体的子属性
 */
pub const DATA_TYPE_AGG_SINGLE: &str = "InternalAggSingle";
/**
 * agg 1对多关系 主实体的子属性
 */
pub const DATA_TYPE_AGG_ARRAY: &str = "InternalAggArray";

#[derive(Clone, Debug, Default)]
pub struct EntityInfo {
    /**
     * 实体名称
     */
    pub name: String,
    /**
     * 实体显示名称
     */
    pub display_name: String,
    /**
     * 类名
     */
    pub class_name: String,
    /**
     * 表名
     */
    pub table_name: String,
    /**
     * 服务路径
     */
    pub base_path: String,
}

#[derive(Clone, Debug, Default)]
pub struct AttributeInfo {
    /**
     * 属性名称
     */
    pub name: String,
    /**
     * 属性显示名称
     */
    pub display_name: String,
    /**
     * 数据库字段名
     */
    pub column_name: String,
    /**
     * 数据类型
     */
    pub data_type: String,
    /**
     * 关联的内部属性名称（外键属性，外键引用属性）
     */
    pub inner_attribute_name: String,

    /**
     * 外部实体名
     */
    pub out_entity_name: String,

    /**
     * 外部实体主属性名
     */
    pub out_entity_pk_attribute_name: String,

    /**
     * 外部实体引用本实体的属性名称
     */
    pub out_entity_reversal_attribute_name: String,

    /**
     * 外部实体引用本实体的外键属性名称
     */
    pub out_entity_id_reversal_attribute_name: String,
}

#[derive(Clone, Debug)]
pub struct EntityDesc {
    /**
     * 实体信息
     */
    pub entity_info: EntityInfo,

    /**
     * 属性信息
     */
    pub attribute_info_map: HashMap<String, AttributeInfo>,

    /**
     * 获取主键属性描述
     */
    pub pk_attribute_info: AttributeInfo,

    /**
     * 获取不在同一个聚合根下的外键Id属性描述
     */
    pub normal_fk_id_attribute_infos: Vec<AttributeInfo>,

    /**
     * 获取不在同一个聚合根下的外键属性描述
     */
    pub normal_fk_attribute_infos: Vec<AttributeInfo>,

    /**
     * 获取不在同一个聚合根下子属性描述
     */
    pub normal_children: Vec<AttributeInfo>,

    /**
     * 1对1情况的子属性
     */
    pub normal_one_2_one_children: Vec<AttributeInfo>,
}
