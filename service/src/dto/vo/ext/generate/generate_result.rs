use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateResult {
    pub zip_file_name: String,
    pub zip_file_full_name: String,
    pub generate_target_dir: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSingleFileResult {
    pub file_name: String,
    pub file_full_name: String,
    pub generate_target_dir: String,
}