use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    pub sampler: u32,
    pub source: u32,
    pub name: Option<String>
}