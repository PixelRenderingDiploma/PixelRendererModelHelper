pub mod accessor;
pub mod asset;
pub mod buffer_view;
pub mod image;
pub mod sampler;
pub mod texture;
pub mod node;
pub mod scene;
pub mod material;
pub mod mesh;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;
use byteorder::{NativeEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::binary_serializable::BinarySerializable;
use crate::read_to_string_exact::ReadToStringExact;
use accessor::{Accessor, AccessorType};
use asset::Asset;
use buffer_view::{BufferView, Buffer};
use image::Image;
use material::Material;
use mesh::Mesh;
use sampler::Sampler;
use node::Node;
use texture::Texture;
use scene::Scene;

static BINARY_HEADER_MAGIC: &str = "glTF";
static BINARY_HEADER_LENGTH: u64 = 12;
static BINARY_CHUNK_HEADER_LENGTH: u64 = 8;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GLTF {
    pub accessors: Vec<Accessor>,
    pub asset: Asset,
    pub buffer_views: Vec<BufferView>,
    pub buffers: Vec<Buffer>,
    #[serde(default)]
    pub images: Vec<Image>,
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub samplers: Vec<Sampler>,
    pub scene: u32,
    pub scenes: Vec<Scene>,
    #[serde(default)]
    pub textures: Vec<Texture>
}

impl std::fmt::Display for GLTF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

pub struct GLTFReader {
    pub gltf: GLTF,
    pub gltf_size: u32,
    pub reader: BufReader<File>,
    pub version: u32,
    pub length: u32
}

impl GLTFReader {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Unable to read scene file");
        let mut reader = BufReader::new(file);

        let magic = reader.read_to_string_lossy_exact(4).expect("Can't read file magic");
        let version = reader.read_u32::<NativeEndian>().expect("Can't read file version");
        let length = reader.read_u32::<NativeEndian>().expect("Can't read file length");

        assert!(magic == BINARY_HEADER_MAGIC, "File is not a glTF binary file");

        let gltf_size = reader.read_u32::<NativeEndian>().expect("Unable to read json size");
        let _ = reader.seek(SeekFrom::Start(BINARY_HEADER_LENGTH + BINARY_CHUNK_HEADER_LENGTH)); // let _ = reader.seek_relative(4); // Skip chunk type
        let gltf_str = reader.read_to_string_exact(gltf_size as usize).expect("Unable to convert buffer to string");
        println!("{}", gltf_str);

        // Output the parsed JSON to a file
        let val: Value = serde_json::from_str(&gltf_str).unwrap();
        let gltf_pretty = serde_json::to_string_pretty(&val).unwrap();
        let _ = std::fs::write("../output/info.json", gltf_pretty);

        let gltf: GLTF = serde_json::from_str(&gltf_str).expect("Can't parse JSON");        

        GLTFReader {gltf, gltf_size, reader, version, length }
    }
}

pub trait BufferReader {
    fn read_u8_buf(&mut self, idx: u32) -> Vec<u8>;
    fn read_u8_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<u8>>;
    fn read_u16_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<u16>>;
    fn read_u32_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<u32>>;
    fn read_f32_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<f32>>;
    fn read_serializable_buf<T: BinarySerializable>(&mut self, idx: u32) -> Vec<T>;
}

impl BufferReader for GLTFReader {
    fn read_u8_buf(&mut self, idx: u32) -> Vec<u8> {
        let offset = self.gltf.buffer_views[idx as usize].byte_offset;
        let length = self.gltf.buffer_views[idx as usize].byte_length;
        println!("{} {}", offset, length); 
        
        let _ = self.reader.seek(SeekFrom::Start(BINARY_HEADER_LENGTH + BINARY_CHUNK_HEADER_LENGTH + self.gltf_size as u64 + BINARY_CHUNK_HEADER_LENGTH + offset as u64));

        let mut buffer = vec![0; length as usize];
        let _ = self.reader.read_exact(&mut buffer);
        
        buffer
    }

    fn read_u8_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<u8>> {
        let buffer = self.read_u8_buf(idx);
        rearrange_data(buffer, t)
    }

    fn read_u16_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<u16>> {
        let u_buffer = self.read_u8_buf(idx);
        let buffer: Vec<u16> = u_buffer.chunks_exact(2)
            .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
            .collect();

        rearrange_data(buffer, t)
    }

    fn read_u32_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<u32>> {
        let u_buffer = self.read_u8_buf(idx);
        let buffer: Vec<u32> = u_buffer.chunks_exact(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
            .collect();

        rearrange_data(buffer, t)
    }

    fn read_f32_buf_arr(&mut self, idx: u32, t: AccessorType) -> Vec<Vec<f32>> {
        let u_buffer = self.read_u8_buf(idx);
        let buffer: Vec<f32> = u_buffer.chunks_exact(4)
            .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))
            .collect();

        rearrange_data(buffer, t)
    }

    fn read_serializable_buf<T: BinarySerializable>(&mut self, idx: u32) -> Vec<T> {
        let offset = self.gltf.buffer_views[idx as usize].byte_offset;
        let length = self.gltf.buffer_views[idx as usize].byte_length;
        println!("{} {}", offset, length); 

        let size = mem::size_of::<T>();
        let count = (length as usize) / size;
        let mut vec = Vec::with_capacity(count);
        
        for _ in 0..count {
            let item = T::read(&mut self.reader).unwrap();
            vec.push(item);
        }

        vec
    }
}

fn rearrange_data<T: Clone>(data: Vec<T>, arrange: AccessorType) -> Vec<Vec<T>> {
    match arrange {
        AccessorType::SCALAR => vec![data],
        AccessorType::VEC2 => data.chunks(2).map(|chunk| chunk.to_vec()).collect(),
        AccessorType::VEC3 => data.chunks(3).map(|chunk| chunk.to_vec()).collect(),
        AccessorType::VEC4 => data.chunks(4).map(|chunk| chunk.to_vec()).collect(),
        AccessorType::MAT2 => data.chunks(4).map(|chunk| chunk.to_vec()).collect(),
        AccessorType::MAT3 => data.chunks(9).map(|chunk| chunk.to_vec()).collect(),
        AccessorType::MAT4 => data.chunks(16).map(|chunk| chunk.to_vec()).collect(),
    }
}