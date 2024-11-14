use serde::{Deserialize, Serialize};
/// 页面设计代码生成入参
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UIFactoryParamPO {
    /// 模板参数json化，用于调试输出
    #[serde(default)]
    pub param_json: Option<String>,
    /// UI配置名称
    #[serde(default)]
    pub name: Option<String>,
    /// UI配置显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// UI配置参数
    #[serde(default)]
    pub ui_json: Option<serde_json::Value>,
    /// UI配置字符串
    #[serde(default)]
    pub ui_content: Option<String>,
    /// 表单配置参数
    #[serde(default)]
    pub b_json: Option<serde_json::Value>,
    /// 表单配置参数字符串
    #[serde(default)]
    pub b_content: Option<String>,
    /// 表单配置引用的元数据参数
    #[serde(default)]
    pub b_m_d_json: Option<serde_json::Value>,
    /// 列表配置参数
    #[serde(default)]
    pub b_table_json: Option<serde_json::Value>,
    /// 列表配置参数字符串
    #[serde(default)]
    pub b_table_content: Option<String>,
    /// 查询配置参数
    #[serde(default)]
    pub q_json: Option<serde_json::Value>,
    /// 查询配置参数字符串
    #[serde(default)]
    pub q_content: Option<String>,
    /// 树配置参数
    #[serde(default)]
    pub t_json: Option<serde_json::Value>,
    /// 树配置参数字符串
    #[serde(default)]
    pub t_content: Option<String>,
    /// 列表按钮配置参数
    #[serde(default)]
    pub v_button_json: Option<serde_json::Value>,
    /// 列表按钮配置参数
    #[serde(default)]
    pub v_button_content: Option<String>,
    /// 表单按钮配置参数
    #[serde(default)]
    pub button_json: Option<serde_json::Value>,
    /// 表单按钮配置参数
    #[serde(default)]
    pub button_content: Option<String>,
}
