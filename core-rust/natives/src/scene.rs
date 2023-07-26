use glam::u32;
use jni::sys::jlong;
use crate::id_pool::IDPool;
use crate::resource::chunk_mesh_resource::ChunkMeshResource;
use crate::ui::{JavaHandle, arc_from_handle, arc_to_handle, arc_dispose_handle};
use std::sync::{Weak,Arc, Mutex};

pub struct SceneChunk {
    pub transform: glam::Mat4,
    pub mesh: Option<std::sync::Weak<Mutex<ChunkMeshResource>>>
}

impl SceneChunk {
}

impl JavaHandle<Arc<SceneChunk>> for SceneChunk {
    fn from_handle(ptr: jlong) -> Option<Arc<SceneChunk>> {
        arc_from_handle(ptr)  
    }

    fn to_handle(from: Arc<SceneChunk>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<SceneChunk>(ptr); 
    }
}

pub struct Scene {
    chunk_uniform_buffer: wgpu::Buffer,
    
    chunk_id_pool: IDPool,
    chunk_pool: smallvec::SmallVec<[SceneChunk; 1024]> 
}


pub struct ChunkMutator<'a>{
    chunk: &'a SceneChunk,
    scene: &'a Scene,
    index: u32
}

impl<'a> ChunkMutator<'a> {

}

pub type ChunkHandle = u32;
impl Scene {
//    pub fn register_chunk<'a>(&mut self) -> ChunkHandle {
//        let chunk_id = self.chunk_id_pool.fetch_id();
//        let new_chunk = SceneChunk {
//            transform: glam::Mat4::IDENTITY,
//            mesh: None
//        };
//        
//        match self.chunk_pool.get_mut(chunk_id as usize) {
//            Some(view) => {
//                (*view) = new_chunk;
//            },
//            None => {
//                self.chunk_pool.push(new_chunk);
//            }
//        }
//        return chunk_id ;
//    }
//
//    pub fn return_chunk(&mut self, id: ChunkHandle) {
//        self.chunk_id_pool.return_id(id);
//    }
//    pub fn fetch_chunk<'a>(&'a mut self, id: ChunkHandle) -> ChunkMutator<'a> {
//        ChunkMutator {
//            chunk: &self.chunk_pool[id as usize],
//            scene: self,
//            index: id
//        }
//    }

}



