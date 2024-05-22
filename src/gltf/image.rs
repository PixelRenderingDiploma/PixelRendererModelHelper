use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub buffer_view: u32,
    pub mime_type: String,
    pub name: String
}