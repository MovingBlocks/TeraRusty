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

impl Rect {
    pub fn size(&self) -> [f32; 2] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1]
        ]
    }
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GuiTexturePerFrameUniform {
    pub view_transform: [[f32; 4]; 4]
}

const VERTEX_BUFFER_INITIAL_SIZE: u64= 4096;
const INDEX_BUFFER_INITIAL_SIZE: u64 = 2048;

impl Rect {
    pub fn zero() -> Self{
        Rect {
            min:[0.0,0.0],
            max: [0.0,0.0]
        }
    }
}

#[derive(Clone)]
struct TextureDrawGroup {
    texture_index: u32,
   
    cursor_index: u32,

    vertex_offset: u64,
    vertex_offset_end: u64,
    vertex_buffer: std::rc::Rc<wgpu::Buffer>,
    
    index_offset: u64,
    index_offset_end: u64,
    index_buffer: std::rc::Rc<wgpu::Buffer>,
    
    crop: Option<Rect>
}


pub enum UIDrawGroup {
    Texture(TextureDrawGroup),
}


pub struct UserInterface {
    crop: Option<Rect>, 
   // uniform_buffer: wgpu::Buffer,

    frame_uniform: wgpu::Buffer,
    immediate_vertex_buffer: Option<Rc<wgpu::Buffer>>,
    immediate_index_buffer: Option<Rc<wgpu::Buffer>>,
    vertex_buffer_offset: u64,
    index_buffer_offset: u64,

    gui_texture_pipeline: wgpu::RenderPipeline,

    tile_sampler: wgpu::Sampler,
    default_sampler: wgpu::Sampler,
    
    draw_groups: Vec<UIDrawGroup>,
}

impl Drop for UserInterface {
    fn drop(&mut self) {
        self.draw_groups.clear();
    }
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
        self.draw_groups.clear();
        self.cmd_set_crop(None); 
    }

    pub fn cmd_dispatch(&self, quad: &Rect,view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder) {
        {

            // Create and update the transform matrix for the current frame.
            // This is required to adapt to vulkan coordinates.
            let size = quad.size();
            let offset_x = quad.min[0] / size[0];
            let offset_y = quad.min[1] / size[1];
            let perFrame = GuiTexturePerFrameUniform  {
                view_transform: [
                    [2.0 / size[0], 0.0, 0.0, 0.0],
                    [0.0, 2.0 / -size[1], 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [-1.0 - offset_x * 2.0, 1.0 + offset_y * 2.0, 0.0, 1.0],
                ]
            }; 
            queue.write_buffer(&self.frame_uniform, 0, bytemuck::cast(perFrame));
        } 

        encoder.push_debug_group("ui pass");
        for group in self.draw_groups.iter() {
            match group {
                UIDrawGroup::Texture(ref tex) => {
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true
                            },
                        })],
                        depth_stencil_attachment: None 
                    });
                    pass.set_pipeline(&self.gui_texture_pipeline);
                    pass.set_index_buffer(tex.index_buffer.slice(tex.index_offset..tex.index_offset_end), wgpu::IndexFormat::Uint32);
                    pass.set_vertex_buffer(0, tex.vertex_buffer.slice(tex.vertex_offset..tex.vertex_offset_end));
                    pass.draw_indexed(0..tex.cursor_index, 0, 0..1);
                }
            }

        }
        encoder.pop_debug_group();
    }
    
    pub fn cmd_set_crop(&mut self, rect: Option<Rect>) {
        self.crop = rect;
    }

    fn evaluate_draw_group(&mut self, new_group: UIDrawGroup) -> bool {
        let group_valid = match self.draw_groups.last() {
            Some(current_draw_group) => {
                match (current_draw_group, &new_group) {
                    (UIDrawGroup::Texture(current), UIDrawGroup::Texture(new_group)) => {
                       current.texture_index == new_group.texture_index
                        && current.crop == new_group.crop
                        && Rc::ptr_eq(&current.vertex_buffer, &new_group.vertex_buffer)
                        && Rc::ptr_eq(&current.index_buffer, &new_group.index_buffer)
                    }
                }
            },
            None => false
        };

        if !group_valid {
            self.draw_groups.push(new_group);
            return true
        }
        return false
    }


    fn request_buffer_immediate(&mut self, device: &wgpu::Device, request_vertex_buffer_size: u64, request_index_buffer_size: u64) -> (u64, u64){
        let mut vb_last_offset = self.vertex_buffer_offset;
        let mut ib_last_offset = self.index_buffer_offset; 

        if self.immediate_vertex_buffer.is_none() ||  (request_vertex_buffer_size + self.vertex_buffer_offset) > self.immediate_vertex_buffer.as_ref().unwrap().size() {
            let size = self.immediate_vertex_buffer.as_ref().map_or_else(|| VERTEX_BUFFER_INITIAL_SIZE, |d| d.size() * 2) as wgpu::BufferAddress;
            self.immediate_vertex_buffer = Some(Rc::new(device.create_buffer(
                &wgpu::BufferDescriptor  {
                    label: Some("Unit Square Vertex Buffer"),
                    size ,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false
                })));
            vb_last_offset = 0; 
            self.vertex_buffer_offset = 0;
        }
        if self.immediate_index_buffer.is_none() ||  (request_index_buffer_size + self.index_buffer_offset) > self.immediate_index_buffer.as_ref().unwrap().size() {
            let size = self.immediate_index_buffer.as_ref().map_or_else(|| INDEX_BUFFER_INITIAL_SIZE, |d| d.size() * 2) as wgpu::BufferAddress;
            self.immediate_index_buffer = Some(Rc::new(device.create_buffer(
                &wgpu::BufferDescriptor  {
                    label: Some("Unit Square Vertex Buffer"),
                    size,
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false
                })));
            ib_last_offset = 0;
            self.index_buffer_offset = 0;
        }
        self.vertex_buffer_offset += request_vertex_buffer_size;
        self.index_buffer_offset += request_index_buffer_size;
        (vb_last_offset, ib_last_offset)
    }

    pub fn cmd_draw_texture(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, uv: &Rect, pos: &Rect) {
        const NUM_VERTS: usize = 4;
        const NUM_INDCIES: usize = 6;
        let request_vertex_buffer_size = (std::mem::size_of::<PositionTexCoord>() * NUM_VERTS) as u64;
        let request_index_buffer_size = (std::mem::size_of::<u32>() * NUM_INDCIES) as u64;
        let (vb_buffer_start_offset, ib_buffer_start_offset) = self.request_buffer_immediate(device, request_vertex_buffer_size, request_index_buffer_size);
        let _is_new_group = self.evaluate_draw_group(UIDrawGroup::Texture(TextureDrawGroup {
            vertex_buffer: self.immediate_vertex_buffer.as_ref().unwrap().clone(),
            index_buffer: self.immediate_index_buffer.as_ref().unwrap().clone(),
            vertex_offset: vb_buffer_start_offset,
            vertex_offset_end: vb_buffer_start_offset,
            index_offset: ib_buffer_start_offset,
            index_offset_end: ib_buffer_start_offset,
            crop: self.crop,
            texture_index: 0,
            cursor_index: 0
        }));
        let UIDrawGroup::Texture(ref mut current_group) = self.draw_groups.last_mut().unwrap();

        let vertex_data: &[PositionTexCoord; NUM_VERTS] = &[
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
       
        let cursor = current_group.cursor_index;
        let index_data: &[u32; NUM_INDCIES] = &[
            cursor + 0, cursor + 1, cursor + 2,
            cursor + 0, cursor + 2, cursor + 3
        ];
        current_group.cursor_index += 4;
        current_group.vertex_offset_end += request_vertex_buffer_size;
        current_group.index_offset_end += request_index_buffer_size;
        queue.write_buffer(&self.immediate_vertex_buffer.as_ref().unwrap(), vb_buffer_start_offset, bytemuck::cast_slice(vertex_data));
        queue.write_buffer(&self.immediate_index_buffer.as_ref().unwrap(), ib_buffer_start_offset, bytemuck::cast_slice(index_data));
    }

    pub fn new(
        device: &wgpu::Device,
        surface: &wgpu::SurfaceConfiguration
    ) -> UserInterface {
        let gui_per_frame_size = mem::size_of::<GuiTexturePerFrameUniform>() as wgpu::BufferAddress;

        let gui_texture_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("gui_texture.wgsl"))),
        });

        let gui_per_frame_uniform =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 2, // global
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(gui_per_frame_size),
                    },
                    count: None,
                }],
            });
        
        let gui_texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::Cube,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        
        let gui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&gui_per_frame_uniform, &gui_texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let gui_texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gui texture pipeline"),
            layout: Some(&gui_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &gui_texture_shader ,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                    },
                ]
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
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            depth_stencil: None,
        });

        let frame_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: gui_per_frame_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
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
            frame_uniform, 
            immediate_vertex_buffer: None,
            immediate_index_buffer: None,
            vertex_buffer_offset: 0,
            index_buffer_offset: 0,
            gui_texture_pipeline,
            tile_sampler,
            default_sampler,

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


