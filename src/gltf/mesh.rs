use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    pub name: String,
    pub primitives: Vec<MeshPrimitives>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshPrimitives {
    pub attributes: HashMap<AttributeKey, u8>,
    pub indices: u8,
    pub material: u8,
    pub mode: u8
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AttributeKey {
    Position,
    Normal,
    Tangent,
    Texcoord(u8),
    Color(u8),
    Joints(u8),
    Weights(u8)
}

impl Serialize for AttributeKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            AttributeKey::Position => "POSITION".to_string(),
            AttributeKey::Normal => "NORMAL".to_string(),
            AttributeKey::Tangent => "TANGENT".to_string(),
            AttributeKey::Texcoord(index) => format!("TEXCOORD_{}", index),
            AttributeKey::Color(index) => format!("COLOR_{}", index),
            AttributeKey::Joints(index) => format!("JOINTS_{}", index),
            AttributeKey::Weights(index) => format!("WEIGHTS_{}", index),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for AttributeKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttributeKeyVisitor;

        impl<'de> serde::de::Visitor<'de> for AttributeKeyVisitor {
            type Value = AttributeKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string key for AttributeKey")
            }

            fn visit_str<E>(self, value: &str) -> Result<AttributeKey, E>
            where
                E: serde::de::Error,
            {
                if let Some((prefix, index)) = value.rsplit_once('_') {
                    if let Ok(index) = index.parse::<u8>() {
                        return match prefix {
                            "TEXCOORD" => Ok(AttributeKey::Texcoord(index)),
                            "COLOR" => Ok(AttributeKey::Color(index)),
                            "JOINTS" => Ok(AttributeKey::Joints(index)),
                            "WEIGHTS" => Ok(AttributeKey::Weights(index)),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        };
                    }
                }

                // Handle non-indexed attributes
                match value {
                    "POSITION" => Ok(AttributeKey::Position),
                    "NORMAL" => Ok(AttributeKey::Normal),
                    "TANGENT" => Ok(AttributeKey::Tangent),
                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                }
            }
        }

        deserializer.deserialize_str(AttributeKeyVisitor)
    }
}

const FIELDS: &[&str] = &[
    "POSITION", "NORMAL", "TANGENT", "TEXCOORD", "COLOR", "JOINTS", "WEIGHTS"
];