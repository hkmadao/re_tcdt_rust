use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTreeVO {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub id_parent: Option<String>,
    #[serde(default)]
    pub action: i32,
    /// 项目编号
    #[serde(default)]
    pub code: Option<String>,
    /// 显示名称
    #[serde(default)]
    pub display_name: Option<String>,
    /// 系统路径
    #[serde(default)]
    pub path: Option<String>,
    /// 后台项目模板编号:
    #[serde(default)]
    pub template_code: Option<String>,
    /// 前端项目模板编号:
    #[serde(default)]
    pub web_template_code: Option<String>,
    /// 备注
    #[serde(default)]
    pub note: Option<String>,
    /// 文件名样式
    #[serde(default)]
    pub file_name_type: Option<String>,
    ///
    /// 树级别   
    /// project: 系统   
    /// subProject: 子系统    
    /// entityCollection: 实体集
    ///
    #[serde(default)]
    pub level: String,
    /// 子节点
    #[serde(default)]
    pub children: Vec<ProjectTreeVO>,
}
