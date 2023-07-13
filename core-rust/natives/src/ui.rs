use crate::engine_kernel::{EngineKernel, WeakEngineRef};
use crate::java_util::{arc_dispose_handle, arc_from_handle, arc_to_handle, JavaHandle};
use jni::sys::{jlong};
use std::{borrow::Cow, f32::consts, mem};
use std::cell::RefCell;
use std::sync::Arc;
use bytemuck::{Pod, Zeroable};
use wgpu::util::{align_to, DeviceExt, StagingBelt};
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
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
pub struct GuiTextureUniform {
    pub transform: [[f32;2]; 2],
    pub uv_transform: [[f32;2]; 2],
    pub texture_index: u32,
    pub sampler_index: u32,
}

const VERTEX_BUFFER_INITIAL_SIZE: u64= 4096;
const INDEX_BUFFER_INITIAL_SIZE: u64 = 2048;
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

struct TextureDrawGroup {
    texture_index: u32,
    vertex_offset: u32,
    index_offset: u32,
    texture_size: u32,
    sampler_size: u32,
    vertex_buffer: Option<std::rc::Weak<wgpu::Buffer>>,
    index_buffer: Option<std::rc::Weak<wgpu::Buffer>>,
    crop: Option<Rect>
}

pub enum UIDrawGroup {
    Texture(TextureDrawGroup),
}


pub struct UserInterface {
    crop: Option<Rect>, 
    uniform_buffer: wgpu::Buffer,
   
    vertex_buffer: Option<Rc<wgpu::Buffer>>,
    index_buffer: Option<Rc<wgpu::Buffer>>,
    cursor_vertex: u64, 
    cursor_index: u64,

    gui_texture_pipeline: wgpu::RenderPipeline,

    tile_sampler: wgpu::Sampler,
    default_sampler: wgpu::Sampler,
    
    draw_groups: Vec<UIDrawGroup>,
}

impl JavaHandle<Arc<RefCell<UserInterface>>> for UserInterface {
    fn from_handle(ptr: jlong) -> Option<Arc<RefCell<UserInterface>>> {
        arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<RefCell<UserInterface>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<RefCell<UserInterface>>(ptr);
    }
}


impl UserInterface {
    pub fn cmd_prepare(&mut self) {
        self.cmd_set_crop(None); 
    }

    pub fn cmd_dispatch(&self) {

    }
    
    pub fn cmd_set_crop(&mut self, rect: Option<Rect>) {
        self.crop = rect;
}

    fn evaluate_draw_group(&mut self, group: UIDrawGroup) -> (bool, &UIDrawGroup){
        let group_valid = match self.draw_groups.last() {
            Some(current_draw_group) => {
                match (current_draw_group, &group) {
                    (UIDrawGroup::Texture(current), UIDrawGroup::Texture(newGroup)) => {
                       current.texture_index == newGroup.texture_index
                        && current.crop == newGroup.crop
                        && match (&current.vertex_buffer, &newGroup.vertex_buffer) {
                            (Some(b1), Some(b2)) => std::rc::Weak::ptr_eq(&b1, &b2), 
                            _ => false
                        } && match (&current.index_buffer, &newGroup.index_buffer) {
                            (Some(b1), Some(b2)) => std::rc::Weak::ptr_eq(&b1, &b2), 
                            _ => false
                        }
                    }
                    _ => false 
                }
            },
            None => false
        };

        if !group_valid {
            self.draw_groups.push(group);
            return (true, &self.draw_groups.last().unwrap())
        }
        return (false, &self.draw_groups.last().unwrap())
    }

    fn write_vertex_data(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, data: &[u8]) {
        if self.vertex_buffer.is_none() ||  (data.len() as u64 + self.cursor_vertex) > self.vertex_buffer.unwrap().size() {
            let size = self.vertex_buffer.map_or_else(|| VERTEX_BUFFER_INITIAL_SIZE, |d| d.size() * 2) as wgpu::BufferAddress;
            self.vertex_buffer = Some(Rc::new(device.create_buffer(
                &wgpu::BufferDescriptor  {
                    label: Some("Unit Square Vertex Buffer"),
                    size ,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false
                })));
            self.cursor_vertex = 0;
        }
        queue.write_buffer(&self.vertex_buffer.unwrap(), self.cursor_vertex, data);
        self.cursor_vertex += data.len() as u64;
    }


    pub fn cmd_draw_texture(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, uv: &Rect, pos: &Rect) {
        let vertex_data: &[PositionTexCoord] = &[
            PositionTexCoord {
                pos: pos.min,
                uv: uv.min  
            },
            PositionTexCoord {
                pos: [pos.max[0], pos.min[1]],
                uv: [uv.max[0], uv.min[1]]   
            },
            PositionTexCoord {
                pos: pos.max,
                uv: uv.max,
            },
            PositionTexCoord {
                pos: [pos.min[0], pos.max[1]],
                uv: [uv.min[0], uv.max[1]]   
            }
        ];
        const index_data: &[u32] = &[
            0, 1, 2,
            0, 2, 3
        ];
        
        let vertex_raw = bytemuck::cast_slice(vertex_data) as &[u8];
        let index_raw = bytemuck::cast_slice(index_data) as &[u8];
        if self.vertex_buffer.is_none() ||  (vertex_raw.len() as u64 + self.cursor_vertex) > self.vertex_buffer.unwrap().size() {
            let size = self.vertex_buffer.map_or_else(|| VERTEX_BUFFER_INITIAL_SIZE, |d| d.size() * 2) as wgpu::BufferAddress;
            self.vertex_buffer = Some(Rc::new(device.create_buffer(
                &wgpu::BufferDescriptor  {
                    label: Some("Unit Square Vertex Buffer"),
                    size ,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false
                })))
        }

        if self.index_buffer.is_none() ||  (index_raw.len() as u64 + self.cursor_vertex) > self.index_buffer.unwrap().size() {
            let size = self.index_buffer.map_or_else(|| INDEX_BUFFER_INITIAL_SIZE, |d| d.size() * 2) as wgpu::BufferAddress;
            self.index_buffer = Some(Rc::new(device.create_buffer(
                &wgpu::BufferDescriptor  {
                    label: Some("Unit Square Vertex Buffer"),
                    size,
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false
                })))
        }




        //self.vertex_buffer.size()
       // queue.write_buffer(
       //     &self.vertex_buffer,
       //     0 as wgpu::BufferAddress,
       //     vertex_raw 
       // );
    }

    pub fn new(
        device: &wgpu::Device,
        surface: &wgpu::SurfaceConfiguration
    ) -> UserInterface {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("query buffer"),
            size: (mem::size_of::<wgpu::util::DrawIndirect>() * MAX_UNIFORM_DRAW_CALLS) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: false,
        });



        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Unit Square Index Buffer"),
                contents: bytemuck::cast_slice(&UNIT_TEXTURE_INDEX),
                usage: wgpu::BufferUsages::INDEX,
            },
        );
        
        let gui_texture_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("gui_texture.wgsl"))),
        });

        let gui_texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::Cube,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        
        let gui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&gui_texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let gui_texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sky"),
            layout: Some(&gui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &gui_texture_shader ,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &gui_texture_shader ,
                entry_point: "fs_main",
                targets: &[Some(surface.view_formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
           // depth_stencil: Some(wgpu::DepthStencilState {
           //     format: Self::DEPTH_FORMAT,
           //     depth_write_enabled: false,
           //     depth_compare: wgpu::CompareFunction::LessEqual,
           //     stencil: wgpu::StencilState::default(),
           //     bias: wgpu::DepthBiasState::default(),
           // }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            depth_stencil: None,
        });

        let tile_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("wrap sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("default sampler"),
            ..Default::default()
        });



        UserInterface {
            crop: None,
            uniform_buffer,
            vertex_buffer: None,
            index_buffer: None,
            gui_texture_pipeline,
            tile_sampler,
            default_sampler,

            cursor_vertex: 0,
            cursor_index: 0,
            draw_groups: Vec::new() 
        }
    }


   // #[no_mangle]
   // pub extern "system" fn Java_org_terasology_engine_rust_GuiSubsystem_00024JNI_drop(
   //     _jni: JNIEnv,
   //     _class: JClass,
   //     ptr: jlong,
   // ) {
   //     UserInterface::drop_handle(ptr);
   // }

   // #[no_mangle]
   // pub extern "system" fn Java_org_terasology_engine_rust_GuiSubsystem_00024JNI_create(
   //     _jni: JNIEnv,
   //     _class: JClass,
   //     kernel_ptr: jlong,
   // ) -> jlong {
   //     let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
   //     let read_kernel = kernel.read().unwrap();
   //     let Some(surface) = read_kernel.surface.as_ref() else {panic!("surface not initialized");};

   //     let indirect_buffer = surface.device.create_buffer(&wgpu::BufferDescriptor {
   //         label: Some("query buffer"),
   //         size: (mem::size_of::<wgpu::util::DrawIndirect>() * MAX_INDRECT_DRAW_CALLS ) as wgpu::BufferAddress,
   //         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::MAP_WRITE,
   //         mapped_at_creation: false,
   //     });

   //     let unit_texture_vertex_buffer = surface.device.create_buffer_init(
   //         &wgpu::util::BufferInitDescriptor {
   //             label: Some("Unit Square Vertex Buffer"),
   //             contents: bytemuck::cast_slice(&UNIT_TEXTURE_MESH),
   //             usage: wgpu::BufferUsages::VERTEX,
   //         },
   //     );

   //     let unit_texture_index_buffer = surface.device.create_buffer_init(
   //         &wgpu::util::BufferInitDescriptor {
   //             label: Some("Unit Square Index Buffer"),
   //             contents: bytemuck::cast_slice(&UNIT_TEXTURE_INDEX),
   //             usage: wgpu::BufferUsages::INDEX,
   //         },
   //     );
   //     UserInterface::to_handle(Arc::new(RefCell::new(UserInterface {
   //         crop: None,
   //         indirect_buffer,
   //         unit_texture_vertex_buffer,
   //         unit_texture_index_buffer
   //     })))
   // }
}


