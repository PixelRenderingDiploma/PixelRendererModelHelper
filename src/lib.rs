pub mod read_to_string_exact;
pub mod binary_serializable;
pub mod gltf;

#[cfg(test)]
mod tests {
    use super::*;
    use gltf::BufferReader;

    #[test]
    fn cube_read() {
        let mut gltf_reader = gltf::GLTFReader::new("resources/cube.glb");

        println!("{:?}", gltf_reader.gltf);
        println!("{}", gltf_reader.version);
        println!("{}", gltf_reader.length);


        println!("{:?}", gltf_reader.gltf.meshes);
        let mesh = gltf_reader.gltf.meshes.first().unwrap();
        let prim = mesh.primitives.first().unwrap();

        let pos_buf_idx = prim.attributes[&gltf::mesh::AttributeKey::Position] as usize;
        let pos_accessor = &gltf_reader.gltf.accessors[pos_buf_idx];
        assert_eq!(pos_accessor.count, 8);

        let idx_buf_idx = prim.indices as usize;
        let idx_accessor = &gltf_reader.gltf.accessors[idx_buf_idx];
        assert_eq!(idx_accessor.count, 36);

        let pos_data = gltf_reader.read_f32_buf_arr(pos_buf_idx as u32, gltf::accessor::AccessorType::VEC3);
        println!("{:?}", pos_data);
        assert_eq!(pos_data.len(), 8);

        let idx_data = gltf_reader.read_u32_buf_arr(idx_buf_idx as u32, gltf::accessor::AccessorType::SCALAR)[0].clone();
        println!("{:?}", idx_data);
        println!("{}", idx_data.len());

        assert_eq!(idx_data.len(), 36);
    }
}
