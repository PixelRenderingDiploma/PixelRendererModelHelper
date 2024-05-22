use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    pub buffer_view: u32,
    pub byte_offset: Option<u32>,
    pub component_type: ComponentType,
    pub count: u32,
    pub max: Option<Vec<f32>>,
    pub min: Option<Vec<f32>>,
    pub name: Option<String>,

    #[serde(rename = "type")] 
    pub a_type: AccessorType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccessorType {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    MAT2,
    MAT3,
    MAT4
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ComponentType {
    I8 = 5120,
    U8,
    I16,
    U16,
    U32 = 5125,
    F32
}