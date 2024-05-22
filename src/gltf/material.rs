use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    alpha_mode: Option<String>,
    double_sided: Option<bool>,
    emissive_factor: Option<[f32; 3]>,
    name: Option<String>,
    pbr_metallic_roughness: PbrMetallicRoughness
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    base_color_factor: [f32; 4],
    base_color_texture: Option<BaseColorTexture>,
    metallic_factor: f64,
    roughness_factor: f64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseColorTexture {
    index: i32,
    tex_coord: i32,
}