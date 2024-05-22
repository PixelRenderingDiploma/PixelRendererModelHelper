use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub name: Option<String>,
    pub mesh: Option<u32>,
    
    pub rotation: Option<[f32; 4]>,
    pub scale: Option<[f32; 3]>,
    pub translation: Option<[f32; 3]>,

    #[serde(default)]
    pub children: Vec<u32>
}