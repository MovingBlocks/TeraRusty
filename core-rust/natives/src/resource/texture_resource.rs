use std::convert::From;
use std::sync::Arc;

use glam::u32;
use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass, JObject};
use jni::sys::{jint, jlong};
use wgpu::util::DeviceExt;

use crate::engine_kernel::EngineKernel;
use crate::java_util::{
    arc_dispose_handle, arc_from_handle, arc_to_handle, JavaHandle, set_joml_vector2f,
};

pub struct TextureResource {
    pub texture: wgpu::Texture,
}

fn wgpu_texture_desc<'local, 'ret>(
    mut env: JNIEnv,
    obj: &JObject,
) -> wgpu::TextureDescriptor<'ret> {
    let width = env.get_field(obj, "width", "I").unwrap().i().unwrap();
    let height = env.get_field(obj, "height", "I").unwrap().i().unwrap();
    let layer = env.get_field(obj, "layers", "I").unwrap().i().unwrap();
    let texture_dim = env.get_field(obj, "dim", "I").unwrap().i().unwrap();
    let format = env.get_field(obj, "format", "I").unwrap().i().unwrap();

    let texture_format: wgpu::TextureFormat =
        unsafe { std::mem::transmute::<jint, JavaImageFormat>(format) }.into();
    let texture_dim: wgpu::TextureDimension =
        unsafe { std::mem::transmute::<jint, JavaTextureDim>(texture_dim) }.into();

    wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: layer as u32,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: texture_dim,
        format: texture_format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: None,
        view_formats: &[],
    }
}

impl TextureResource {
    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_drop(
        _jni: JNIEnv,
        _class: JClass,
        ptr: jlong,
    ) {
        TextureResource::drop_handle(ptr);
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_writeTextureBuffer<
        'local,
    >(
        env: JNIEnv<'local>,
        _class: JClass,
        kernel_ptr: jlong,
        texture_ptr: jlong,
        buffer: JByteBuffer<'local>,
    ) {
        let kernel_arc = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
        let texture_arc = TextureResource::from_handle(texture_ptr).expect("texture invalid");
        let mut kernel = kernel_arc.borrow_mut();
        let buf_size = env
            .get_direct_buffer_capacity(&buffer)
            .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
        let buf: _ = env
            .get_direct_buffer_address(&buffer)
            .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
        let slice = unsafe { std::slice::from_raw_parts(buf, buf_size) };
        let window = kernel.surface.as_mut().unwrap();
        let format = texture_arc.texture.format().bit_size_block() / 8;
        window.queue.write_texture(
            texture_arc.texture.as_image_copy(),
            &slice,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(format),
                rows_per_image: None,
            },
            wgpu::Extent3d::default(),
        );
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_createTextureResource<
        'local,
    >(
        env: JNIEnv<'local>,
        _class: JClass,
        kernel_ptr: jlong,
        desc: JObject<'local>,
    ) -> jlong {
        let kernel_arc = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
        let mut kernel = kernel_arc.borrow_mut();
        let window = kernel.surface.as_mut().unwrap();
        let texture_desc = wgpu_texture_desc(env, &desc);

        let texture = window
            .device
            .create_texture(&wgpu::TextureDescriptor { ..texture_desc });
        TextureResource::to_handle(Arc::new(TextureResource { texture }))
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_createTextureResourceFromBuffer<
        'local,
    >(
        env: JNIEnv<'local>,
        _class: JClass,
        kernel_ptr: jlong,
        desc: JObject<'local>,
        buffer: JByteBuffer<'local>,
    ) -> jlong {
        let arc = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
        let kernel = arc.borrow();
        let buf_size = env
            .get_direct_buffer_capacity(&buffer)
            .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
        let buf: _ = env
            .get_direct_buffer_address(&buffer)
            .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
        let slice = unsafe { std::slice::from_raw_parts(buf, buf_size) };
        let window = kernel.surface.as_ref().unwrap();
        let texture_desc = wgpu_texture_desc(env, &desc);

        let texture = window.device.create_texture_with_data(
            &window.queue,
            &wgpu::TextureDescriptor { ..texture_desc },
            slice,
        );
        TextureResource::to_handle(Arc::new(TextureResource { texture }))
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_getSize<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass,
        texture_ptr: jlong,
        mut vec2_obj: JObject<'local>,
    ) {
        let texture_arc = TextureResource::from_handle(texture_ptr).expect("texture invalid");
        let size = texture_arc.texture.size();
        // joml_vec2::<f32>(vec2_obj)
        //     .set(&mut env, size.width as f32, size.height as f32);
        set_joml_vector2f(env, &mut vec2_obj, size.width as f32, size.height as f32);
    }
}

impl JavaHandle<Arc<TextureResource>> for TextureResource {
    fn from_handle(ptr: jlong) -> Option<Arc<TextureResource>> {
        arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<TextureResource>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<TextureResource>(ptr);
    }
}

#[repr(u32)]
enum JavaTextureDim {
    DIM_1D,
    DIM_2D,
    DIM_3D,
}

impl From<JavaTextureDim> for wgpu::TextureDimension {
    fn from(item: JavaTextureDim) -> Self {
        match item {
            JavaTextureDim::DIM_1D => wgpu::TextureDimension::D1,
            JavaTextureDim::DIM_2D => wgpu::TextureDimension::D2,
            JavaTextureDim::DIM_3D => wgpu::TextureDimension::D3,
        }
    }
}

#[repr(u32)]
enum JavaImageFormat {
    UNKNOWN,
    R8_UNORM,
    R8_SNORM,
    R8_UINT,
    R8_SINT,
    R8G8_UNORM,
    R8G8_SNORM,
    R8G8_UINT,
    R8G8_SINT,
    R16_UNORM,
    R16_SNORM,
    R16_UINT,
    R16_SINT,
    R8G8B8A8_UNORM,
    R8G8B8A8_SNORM,
    R8G8B8A8_UINT,
    R8G8B8A8_SINT,
    R8G8B8A8_SRGB,
}

trait TextureFormatExt {
    fn bit_size_block(&self) -> u32;
}

impl TextureFormatExt for wgpu::TextureFormat {
    fn bit_size_block(&self) -> u32 {
        //TODO: incomplete
        match self {
            wgpu::TextureFormat::R8Unorm
            | wgpu::TextureFormat::R8Sint
            | wgpu::TextureFormat::R8Snorm => 8,
            wgpu::TextureFormat::Rgba8Unorm
            | wgpu::TextureFormat::Rgba8Sint
            | wgpu::TextureFormat::Rgba8Uint
            | wgpu::TextureFormat::Rgba8UnormSrgb => 32,
            _ => 0,
        }
    }
}

impl From<JavaImageFormat> for wgpu::TextureFormat {
    fn from(item: JavaImageFormat) -> Self {
        match item {
            JavaImageFormat::R8_UNORM => wgpu::TextureFormat::R8Unorm,
            JavaImageFormat::R8_SNORM => wgpu::TextureFormat::R8Snorm,
            JavaImageFormat::R8_UINT => wgpu::TextureFormat::R8Uint,
            JavaImageFormat::R8_SINT => wgpu::TextureFormat::R8Sint,
            JavaImageFormat::R8G8_UNORM => wgpu::TextureFormat::Rg8Unorm,
            JavaImageFormat::R8G8_SNORM => wgpu::TextureFormat::Rg8Snorm,
            JavaImageFormat::R8G8_UINT => wgpu::TextureFormat::Rg8Uint,
            JavaImageFormat::R8G8_SINT => wgpu::TextureFormat::Rg8Sint,

            JavaImageFormat::R16_UNORM => wgpu::TextureFormat::R16Unorm,
            JavaImageFormat::R16_SNORM => wgpu::TextureFormat::R16Snorm,
            JavaImageFormat::R16_UINT => wgpu::TextureFormat::R16Uint,
            JavaImageFormat::R16_SINT => wgpu::TextureFormat::R16Sint,

            JavaImageFormat::R8G8B8A8_UNORM => wgpu::TextureFormat::Rgba8Unorm,
            JavaImageFormat::R8G8B8A8_SNORM => wgpu::TextureFormat::Rgba8Snorm,
            JavaImageFormat::R8G8B8A8_UINT => wgpu::TextureFormat::Rgba8Uint,
            JavaImageFormat::R8G8B8A8_SINT => wgpu::TextureFormat::Rgba8Sint,
            JavaImageFormat::R8G8B8A8_SRGB => wgpu::TextureFormat::Rgba8UnormSrgb,
            _ => panic!("invalid image format"),
        }
    }
}
