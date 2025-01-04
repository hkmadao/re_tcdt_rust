use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TemplateFileStat {
    pub id_project: String,
    /// 上级目录路径，用'/'分割
    pub parent_path_name: Option<String>,
    /// 文件路径，用'/'分割
    pub file_path_name: Option<String>,
    /// 文件路径，用'/'分割
    pub old_file_path_name: Option<String>,
    pub file_name: String,
    #[serde(default)]
    pub fg_file: bool,
    pub content: Option<String>,
    #[serde(default)]
    pub children: Vec<TemplateFileStat>,
}