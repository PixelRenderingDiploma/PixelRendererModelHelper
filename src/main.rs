use std::env;

use pixel_renderer_model_helper::gltf::mesh::AttributeKey;
use pixel_renderer_model_helper::gltf::accessor::AccessorType;
use pixel_renderer_model_helper::gltf::{GLTFReader, BufferReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let scene_path = &args[1];

    let mut gltf_reader = GLTFReader::new(scene_path);
    println!("{:?}", gltf_reader.gltf);
    println!("{}", gltf_reader.version);
    println!("{}", gltf_reader.length);

    let img_buff_idx = gltf_reader.gltf.images[0].buffer_view;
    let img_data = gltf_reader.read_u8_buf_arr(img_buff_idx, AccessorType::SCALAR);
    let _ = std::fs::write("../output/output.png", &img_data[0]);

    let pos_buff_idx = gltf_reader.gltf.meshes[0].primitives[0].attributes[&AttributeKey::Position];
    let pos_data = gltf_reader.read_f32_buf_arr(pos_buff_idx as u32, AccessorType::VEC3);
    // println!("{:?}", pos_data);
    println!("{}", pos_data.len());

    let idx_buff_idx = gltf_reader.gltf.meshes[0].primitives[0].indices;
    let idx_data = gltf_reader.read_u16_buf_arr(idx_buff_idx as u32, AccessorType::SCALAR);
    
    println!("{:?}", idx_data.len());
}
