use crate::resource::chunk_mesh_resource::ChunkMeshResource;
use std::sync::Arc;

pub struct Scene {
    opaque_chunks: Vec<Arc<ChunkMeshResource>> 
}

impl Scene {

    pub fn cmd_prepare(&mut self) {
    
    }

    pub fn cmd_dispatch(&mut self) {

    }

    pub fn cmd_queue_opaque_chunk() {

    }
}
