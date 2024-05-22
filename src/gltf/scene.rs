use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<u32>
}