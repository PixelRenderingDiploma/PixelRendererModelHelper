use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    pub wrap_s: u32,
    pub wrap_t: u32,
    pub mag_filter: Option<u32>,
    pub min_filter: Option<u32>
}