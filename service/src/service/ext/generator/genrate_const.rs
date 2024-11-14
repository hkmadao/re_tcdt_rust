pub const INTERNAL_PK: &str = "InternalPK";
pub const INTERNAL_FK: &str = "InternalFK";
pub const INTERNAL_REF: &str = "InternalRef";
pub const INTERNAL_SINGLE_REF: &str = "InternalSingleRef";
pub const INTERNAL_SINGLE: &str = "InternalSingle";
pub const INTERNAL_ARRAY: &str = "InternalArray";
/// agg 外键
pub const INTERNAL_AGG_FK: &str = "InternalAggFK";
/// 1对多关系 子实体的引用属性
pub const INTERNAL_AGG_REF: &str = "InternalAggRef";
/// agg 1对1关系 子实体的引用属性
pub const INTERNAL_AGG_SINGLE_REF: &str = "InternalAggSingleRef";
/// agg 1对1关系 主实体的子属性
pub const INTERNAL_AGG_SINGLE: &str = "InternalAggSingle";
/// agg 1对多关系 主实体的子属性
pub const INTERNAL_AGG_ARRAY: &str = "InternalAggArray";

/**0...1 */
pub const DOWN_TYPE_ZERO_TO_ONE: &str = "zeroToOne";
/**0...N */
pub const DOWN_TYPE_ZERO_TO_MANY: &str = "zeroToMany";
/**1...1 */
pub const DOWN_TYPE_ONE_TO_ONE: &str = "oneToOne";
/**1...N */
pub const DOWN_TYPE_ONE_TO_MANY: &str = "oneToMany";
