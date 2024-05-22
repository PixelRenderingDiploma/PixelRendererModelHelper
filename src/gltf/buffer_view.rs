use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    pub buffer: u32,
    pub byte_length: u32,
    pub byte_offset: u32,
    pub byte_stride: Option<u32>,
    pub name: Option<String>,
    pub target: Option<u32>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    pub byte_length: u32,
    pub uri: Option<String>,
    pub name: Option<String>
}