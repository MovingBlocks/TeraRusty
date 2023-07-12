use crate::{
    engine_kernel::EngineKernel,
    java_util::{arc_dispose_handle, arc_from_handle, arc_to_handle, JavaHandle},
};
use jni::{
    objects::JClass,
    sys::{jlong, JNIEnv},
};
use std::cell::RefCell;
use std::sync::Arc;
use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::util::{align_to, DeviceExt};

pub struct Rect {
    pub min: [f32; 2], 
    pub max: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PositionTexCoord {
    pos: [f32; 2],
    uv: [f32; 2]
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TextureUniform {
    pub transform: [[f32;2]; 2],
    pub uv_transform: [[f32;2]; 2],
    pub texture_index: u32,
    pub sampler_index: u32,
}

const MAX_INDRECT_DRAW_CALLS: usize = 2048;
const UNIT_TEXTURE_MESH: &'static [PositionTexCoord] = &[
    PositionTexCoord {
        pos: [0.0, 0.0],
        uv: [0.0, 0.0]   
    },
    PositionTexCoord {
        pos: [1.0, 0.0],
        uv: [1.0, 0.0]   
    },
    PositionTexCoord {
        pos: [1.0, 1.0],
        uv: [1.0, 1.0]   
    },
    PositionTexCoord {
        pos: [0.0, 1.0],
        uv: [0.0, 1.0]   
    }
];

const UNIT_TEXTURE_INDEX: &'static [u32] = &[
    0, 1, 2,
    0, 2, 3
];

impl Rect {
    pub fn zero() -> Self{
        Rect {
            min:[0.0,0.0],
            max: [0.0,0.0]
        }
    }
}

pub struct GuiSubstem {
   crop: Option<Rect>, 
   indirect_buffer: wgpu::Buffer,
   unit_texture_vertex_buffer: wgpu::Buffer,
   unit_texture_index_buffer: wgpu::Buffer
}

impl JavaHandle<Arc<RefCell<GuiSubstem>>> for GuiSubstem {
    fn from_handle(ptr: jlong) -> Option<Arc<RefCell<GuiSubstem>>> {
        arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<RefCell<GuiSubstem>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<RefCell<GuiSubstem>>(ptr);
    }
}

impl GuiSubstem {

}


struct GuiDrawCmd {

    uv_transofmr: glam::Mat2
}

impl GuiSubstem {
    fn set_crop(&mut self, rect: Rect) {
        self.crop = Some(rect);
    }

    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_GuiSubsystem_00024JNI_drop(
        _jni: JNIEnv,
        _class: JClass,
        ptr: jlong,
    ) {
        GuiSubstem::drop_handle(ptr);
    }

    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_GuiSubsystem_00024JNI_create(
        _jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
    ) -> jlong {
        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let read_kernel = kernel.read().unwrap();
        let Some(surface) = read_kernel.surface.as_ref() else {panic!("surface not initialized");};

        let indirect_buffer = surface.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("query buffer"),
            size: (mem::size_of::<wgpu::util::DrawIndirect>() * MAX_INDRECT_DRAW_CALLS ) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: false,
        });

        let unit_texture_vertex_buffer = surface.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Unit Square Vertex Buffer"),
                contents: bytemuck::cast_slice(&UNIT_TEXTURE_MESH),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );

        let unit_texture_index_buffer = surface.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Unit Square Index Buffer"),
                contents: bytemuck::cast_slice(&UNIT_TEXTURE_INDEX),
                usage: wgpu::BufferUsages::INDEX,
            },
        );
        GuiSubstem::to_handle(Arc::new(RefCell::new(GuiSubstem {
            crop: None,
            indirect_buffer,
            unit_texture_vertex_buffer,
            unit_texture_index_buffer
        })))
    }
}


