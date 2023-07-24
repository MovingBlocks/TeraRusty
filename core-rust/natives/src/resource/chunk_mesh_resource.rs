use bytemuck::{Pod, Zeroable};
use futures::lock::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;
use crate::ui::{JavaHandle, arc_from_handle, arc_to_handle, arc_dispose_handle};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkPosition([f32; 3]);
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkNormal([f32; 3]);
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkUV([f32; 2]);
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkColor(u32);

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkAttributes {
    sunlight: u8,
    block_light: u8,
    ambient_occlusion: u8,
    pad: u8,
    flags: u8,
    frames: u8
}

const POSTION_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout  {
    array_stride: std::mem::size_of::<ChunkPosition>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
};

const NORMAL_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout  {
    array_stride: std::mem::size_of::<ChunkNormal>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
};

const UV_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout  {
    array_stride: std::mem::size_of::<ChunkUV>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
};

const COLOR_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout  {
    array_stride: std::mem::size_of::<ChunkColor>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Unorm8x4],
};

const ATTRIBUTE_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout  {
    array_stride: std::mem::size_of::<ChunkAttributes>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Unorm8x4, 1 => Uint8x2],
};


pub enum MeshRenderType {
    Opaque,
    Translucent,
    Billboard,
    WaterAndIce
}

pub struct ChunkMeshEntry {
    render_type: MeshRenderType,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    num_elements: u32,
    position_start: u64,
    normal_start: u32,
    uv_start: u32,
    color_start: u32,
    attribute_start: u32 
}

impl ChunkMeshEntry {
    pub fn num_elements(&self) -> u32 { self.num_elements }
    pub fn vertex_buffer(&self) -> &wgpu::Buffer { &self.vertex_buffer }
    pub fn positions_slice(&self) -> wgpu::BufferSlice { self.vertex_buffer.slice(self.position_start..(self.position_start + (self.num_elements as u64 * std::mem::size_of::<ChunkPosition>() as u64)))}
   // pub fn normals(&self) -> &wgpu::Buffer { &self.vertex_buffer_pos }
   // pub fn uvs(&self) -> &wgpu::Buffer { &self.vertex_buffer_pos }
   // pub fn colors(&self) -> &wgpu::Buffer { &self.vertex_buffer_pos }
   // pub fn attributes(&self) -> &wgpu::Buffer { &self.vertex_buffer_pos }

}

pub struct ChunkMeshResource {
    data_lock: Mutex<()>,
    meshes: SmallVec<[ChunkMeshEntry; 5]>,
}

impl ChunkMeshResource {
    fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        position: &[ChunkPosition],
        normal: &[ChunkNormal],
        uv: &[ChunkUV],
        color: &[ChunkColor],
        attributes: &[ChunkAttributes]
    ) {
        assert!(position.len() == normal.len(), "mismatch in the number of vertices");
        assert!(position.len() == uv.len(), "mismatch in the number of vertices");
        assert!(position.len() == color.len(), "mismatch in the number of vertices");
        assert!(position.len() == attributes.len(), "mismatch in the number of vertices");
    }

    pub fn set_mesh_resource(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_type: MeshRenderType,
        index_buffer: &[u32],
        position: &[ChunkPosition],
        normal: &[ChunkNormal],
        uv: &[ChunkUV],
        color: &[ChunkColor],
        attributes: &[ChunkAttributes]
        ) {
        assert!(position.len() == normal.len(), "mismatch in the number of vertices");
        assert!(position.len() == uv.len(), "mismatch in the number of vertices");
        assert!(position.len() == color.len(), "mismatch in the number of vertices");
        assert!(position.len() == attributes.len(), "mismatch in the number of vertices");

        let num_elements: u64 = position.len() as u64;

        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                    label: Some("Unit Square Vertex Buffer"),
                    size: num_elements * 
                        (
                            std::mem::size_of::<ChunkPosition>() as u64 + 
                            std::mem::size_of::<ChunkNormal>() as u64 + 
                            std::mem::size_of::<ChunkUV>() as u64 + 
                            std::mem::size_of::<ChunkColor>() as u64 + 
                            std::mem::size_of::<ChunkAttributes >() as u64 
                        ),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false 
            }
        );
    }

    pub fn unset_mesh_resource(render_type: MeshRenderType) {

    }

    fn as_slice(&self) -> &[ChunkMeshEntry] {
        return self.meshes.as_slice();
    }
}

impl JavaHandle<Arc<ChunkMeshResource>> for ChunkMeshResource {
    fn from_handle(ptr: jni::sys::jlong) -> Option<Arc<ChunkMeshResource>> {
        arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<ChunkMeshResource>) -> jni::sys::jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jni::sys::jlong) {
        arc_dispose_handle::<ChunkMeshResource>(ptr);
    }
}

