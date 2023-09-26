use bytemuck::{Pod, Zeroable};
use smallvec::SmallVec;
use std::sync::{Arc, Mutex};
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


#[derive(PartialEq, Clone, Copy)]
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

    num_elements: u64,
    position_start: u64,
    normal_start: u64,
    uv_start: u64,
    color_start: u64,
    attribute_start: u64 
}

impl ChunkMeshEntry {
    pub fn num_elements(&self) -> u64 { self.num_elements }
    pub fn buf_vertex_buffer(&self) -> &wgpu::Buffer { &self.vertex_buffer }
    pub fn buf_index_buffer(&self) -> &wgpu::Buffer { &self.index_buffer }
    
    pub fn buf_positions_slice(&self) -> wgpu::BufferSlice { self.vertex_buffer.slice(self.position_start..(self.position_start + (self.num_elements as u64 * std::mem::size_of::<ChunkPosition>() as u64)))}
    pub fn buf_normals_slice(&self) -> wgpu::BufferSlice { self.vertex_buffer.slice(self.normal_start..(self.normal_start + (self.num_elements as u64 * std::mem::size_of::<ChunkNormal>() as u64))) }
    pub fn buf_uvs_slice(&self) -> wgpu::BufferSlice{ self.vertex_buffer.slice(self.uv_start..(self.uv_start + (self.num_elements as u64 * std::mem::size_of::<ChunkUV>() as u64))) }
    pub fn buf_colors_slice(&self) -> wgpu::BufferSlice { self.vertex_buffer.slice(self.color_start..(self.color_start + (self.num_elements as u64 * std::mem::size_of::<ChunkColor>() as u64))) }
    pub fn buf_attributes_slice(&self) -> wgpu::BufferSlice { self.vertex_buffer.slice(self.attribute_start..(self.attribute_start + (self.num_elements as u64 * std::mem::size_of::<ChunkAttributes>() as u64)))}
}

pub struct ChunkMeshResource {
    meshes: SmallVec<[ChunkMeshEntry; 5]>,
}

impl ChunkMeshResource {
    pub fn new() -> Self {
        Self {
            meshes: SmallVec::new()
        }
    }

    pub fn set_mesh_resource(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_type: MeshRenderType,
        indexes: &[u32],
        position: &[ChunkPosition],
        normal: &[ChunkNormal],
        uv: &[ChunkUV],
        color: &[ChunkColor],
        attributes: &[ChunkAttributes]
        ) {

        let num_elements: u64 = position.len() as u64;
        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                    label: Some("Chunk Vertex Buffer"),
                    size: num_elements * 
                        (
                            std::mem::size_of::<ChunkPosition>() as u64 + 
                            std::mem::size_of::<ChunkNormal>() as u64 + 
                            std::mem::size_of::<ChunkUV>() as u64 + 
                            std::mem::size_of::<ChunkColor>() as u64 + 
                            std::mem::size_of::<ChunkAttributes>() as u64 
                        ),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false 
            }
        );

        let index_buffer = device.create_buffer(
            &wgpu::BufferDescriptor  {
                label: Some("Chunk Index Buffer"),
                size: (indexes.len() * std::mem::size_of::<u32>()) as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false
            }
        );
        
        let position_byte: &[u8] = bytemuck::cast_slice(position);
        let normal_byte: &[u8] = bytemuck::cast_slice(normal);
        let uv_byte: &[u8] = bytemuck::cast_slice(uv);
        let color_byte: &[u8] = bytemuck::cast_slice(color);
        let attribute_byte: &[u8] = bytemuck::cast_slice(attributes);
        
        let mut cursor: u64 = 0;
        let position_start = cursor;
        if position_byte.len() > 0 {
            queue.write_buffer(&vertex_buffer, position_start, position_byte);
        }
        cursor += position_byte.len() as u64;
        
        let normal_start = cursor;
        if normal_byte.len() > 0 {
            queue.write_buffer(&vertex_buffer, normal_start , normal_byte);
        }
        cursor += normal_byte.len() as u64;
        
        let uv_start = cursor;
        if uv_byte.len() > 0 {
            queue.write_buffer(&vertex_buffer, uv_start, uv_byte);
        }
        cursor += uv_byte.len() as u64;
    
        let color_start = cursor;
        if color_byte.len() > 0 {
            queue.write_buffer(&vertex_buffer, color_start, color_byte);
        }
        cursor += normal_byte.len() as u64;
    
        let attribute_start = cursor;
        if attribute_byte.len() > 0 {  
            queue.write_buffer(&vertex_buffer, attribute_start, attribute_byte);
        }
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(indexes));
   
        let chunk_mesh = ChunkMeshEntry {
            render_type,
            vertex_buffer,
            index_buffer,
            num_elements,
            position_start,
            normal_start,
            uv_start,
            color_start,
            attribute_start
        };
        
        match  self.meshes.iter_mut().find(|ref el| {
            el.render_type == render_type
        }) {
            Some(entry) => {
                (*entry) = chunk_mesh; 
            },
            None => {
                self.meshes.push(chunk_mesh);
            }
        };
    }

    pub fn unset_mesh_resource(&mut self, render_type: MeshRenderType) {
        match self.meshes.iter().position(|ref el| el.render_type == render_type) {
            Some(index) => {
                self.meshes.remove(index);
            },
            _ => {}
        }
    }

    pub fn as_slice(&self) -> &[ChunkMeshEntry] {
        return self.meshes.as_slice();
    }
}

impl JavaHandle<Arc<Mutex<ChunkMeshResource>>> for ChunkMeshResource {
    fn from_handle(ptr: jni::sys::jlong) -> Option<Arc<Mutex<ChunkMeshResource>>> {
        arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<Mutex<ChunkMeshResource>>) -> jni::sys::jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jni::sys::jlong) {
        arc_dispose_handle::<Mutex<ChunkMeshResource>>(ptr);
    }
}

