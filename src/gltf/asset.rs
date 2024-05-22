use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub version: String,
    pub generator: Option<String>,
    pub copyright: Option<String>
}