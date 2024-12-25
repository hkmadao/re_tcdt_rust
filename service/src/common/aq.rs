use serde::{Deserialize, Serialize};
use crate::common::aq_const::{LOGIC_OPERATOR_CODE_AND, OPERATOR_CODE_EQUAL, OPERATOR_CODE_IN};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AqPageInfoInput {
    /// 当前页码
    pub page_index: u64,
    /// 分页大小
    pub page_size: u64,
    /// 总记录数
    #[serde(default)]
    pub total_count: u64,
    /// 查询条件
    pub logic_node: Option<Box<AqLogicNode>>,
    /// 排序设置
    #[serde(default)]
    pub orders: Vec<AqOrder>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AqCondition {
    /// 查询条件
    #[serde(default)]
    pub logic_node: Option<Box<AqLogicNode>>,
    /// 分页信息
    #[serde(default)]
    pub orders: Vec<AqOrder>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AqLogicNode {
    /// 逻辑操作编码
    pub logic_operator_code: String,
    /// 子节点
    pub logic_node: Option<Box<AqLogicNode>>,
    /// 查询条件集合
    pub filter_nodes: Vec<AqFilterNode>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AqOrder {
    /// 排序方向
    pub direction: String,
    /// 排序属性
    pub property: String,
    /// 是否忽略
    pub ignore_case: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EFilterParam {
    Bool(Option<bool>),
    U32(Option<u32>),
    U64(Option<u64>),
    I32(Option<i32>),
    I64(Option<i64>),
    F32(Option<f32>),
    F64(Option<f64>),
    String(Option<Box<String>>),
}

impl From<sea_orm::Value> for EFilterParam {
    fn from(value: sea_orm::Value) -> Self {
        match value {
            sea_orm::Value::Bool(v) => EFilterParam::Bool(v),
            sea_orm::Value::Int(v) => EFilterParam::I32(v),
            sea_orm::Value::BigInt(v) => EFilterParam::I64(v),
            sea_orm::Value::Unsigned(v) => EFilterParam::U32(v),
            sea_orm::Value::BigUnsigned(v) => EFilterParam::U64(v),
            sea_orm::Value::Float(v) => EFilterParam::F32(v),
            sea_orm::Value::Double(v) => EFilterParam::F64(v),
            sea_orm::Value::String(v) => EFilterParam::String(v),
            sea_orm::Value::TinyUnsigned(_) => panic!("unsupported TinyUnsigned"),
            sea_orm::Value::SmallUnsigned(_) => panic!("unsupported SmallUnsigned"),
            sea_orm::Value::TinyInt(_) => panic!("unsupported TinyInt"),
            sea_orm::Value::SmallInt(_) => panic!("unsupported SmallInt"),
            sea_orm::Value::Char(_) => panic!("unsupported Char"),
            sea_orm::Value::Bytes(_) => panic!("unsupported Bytes"),
            sea_orm::Value::Json(_) => panic!("unsupported Json"),
            sea_orm::Value::ChronoDate(_) => panic!("unsupported ChronoDate"),
            sea_orm::Value::ChronoTime(_) => panic!("unsupported ChronoTime"),
            sea_orm::Value::ChronoDateTime(_) => panic!("unsupported ChronoDateTime"),
            sea_orm::Value::ChronoDateTimeUtc(_) => panic!("unsupported ChronoDateTimeUtc"),
            sea_orm::Value::ChronoDateTimeLocal(_) => panic!("unsupported ChronoDateTimeLocal"),
            sea_orm::Value::ChronoDateTimeWithTimeZone(_) => {
                panic!("unsupported ChronoDateTimeWithTimeZone")
            }
            sea_orm::Value::TimeDate(_) => panic!("unsupported TimeDate"),
            sea_orm::Value::TimeTime(_) => panic!("unsupported TimeTime"),
            sea_orm::Value::TimeDateTime(_) => panic!("unsupported TimeDateTime"),
            sea_orm::Value::TimeDateTimeWithTimeZone(_) => {
                panic!("unsupported TimeDateTimeWithTimeZone")
            }
            sea_orm::Value::Uuid(_) => panic!("unsupported Uuid"),
            sea_orm::Value::Decimal(_) => panic!("unsupported Decimal"),
            sea_orm::Value::BigDecimal(_) => panic!("unsupported BigDecimal"),
            _ => panic!("unsupported ..."),
        }
    }
}

impl Into<sea_orm::Value> for EFilterParam {
    fn into(self) -> sea_orm::Value {
        match self {
            EFilterParam::Bool(v) => sea_orm::Value::Bool(v),
            EFilterParam::U32(v) => sea_orm::Value::Unsigned(v),
            EFilterParam::U64(v) => sea_orm::Value::BigUnsigned(v),
            EFilterParam::I32(v) => sea_orm::Value::Int(v),
            EFilterParam::I64(v) => sea_orm::Value::BigInt(v),
            EFilterParam::F32(v) => sea_orm::Value::Float(v),
            EFilterParam::F64(v) => sea_orm::Value::Double(v),
            EFilterParam::String(v) => sea_orm::Value::String(v),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AqFilterNode {
    /// 查询条件名称
    pub name: String,
    /// 比较操作符编码
    pub operator_code: String,
    /// 查询参数
    pub filter_params: Vec<EFilterParam>,
}

impl AqCondition {
    pub fn build_equal_condition(field_name: &str, field_value: EFilterParam) -> Self {
        AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_string(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: field_name.to_string(),
                    operator_code: OPERATOR_CODE_EQUAL.to_string(),
                    filter_params: vec![field_value],
                }],
            })),
            orders: vec![],
        }
    }

    pub fn build_in_condition(field_name: &str, field_value_list: Vec<EFilterParam>) -> Self {
        AqCondition {
            logic_node: Some(Box::new(AqLogicNode {
                logic_operator_code: LOGIC_OPERATOR_CODE_AND.to_string(),
                logic_node: None,
                filter_nodes: vec![AqFilterNode {
                    name: field_name.to_string(),
                    operator_code: OPERATOR_CODE_IN.to_string(),
                    filter_params: field_value_list,
                }],
            })),
            orders: vec![],
        }
    }
}
