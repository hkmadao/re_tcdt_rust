use serde::{Deserialize, Serialize};

/// join entity from other collection
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinEntityPO {
    #[serde(default)]
    pub entity_collection: EntityCollectionPO,
    #[serde(default)]
    pub entity_ids: Vec<String>,
    #[serde(default)]
    pub enum_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntityCollectionPO {
    #[serde(default)]
    pub id_entity_collection: String,
}