use jni::{sys::jlong, objects::{JClass, JByteBuffer, JObject}, JNIEnv};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use crate::{resource::texture_resource::TextureResource, ui::JavaHandle, engine_kernel::EngineKernel};
use super::jni_texture::JavaTextureDesc;

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_ResourceManager_00024JNI_createTextureResourceFromBuffer<'local>(mut env: JNIEnv<'local>, _class: JClass, kernel_ptr: jlong, desc: JObject<'local>, buffer: JByteBuffer<'local>) -> jlong {
    let texture_desc = JavaTextureDesc::new(&mut env, desc);
    let wgpu_texture_desc = wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: texture_desc.width as u32,
            height: texture_desc.height as u32,
            depth_or_array_layers: texture_desc.layers as u32 
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: (&texture_desc.dim).into(), 
        format: (&texture_desc.format).into(),
        usage:  wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: None,
        view_formats: &[],
    };
    let buf_size = env.get_direct_buffer_capacity(&buffer)
            .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
    let buf: _ = env
        .get_direct_buffer_address(&buffer)
        .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
    let slice = unsafe {std::slice::from_raw_parts(buf, buf_size)};   
    let kernel = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
    // TODO: this is going to make all this single threaded 
    let surface = kernel.window_surface.lock().expect("failed to lock surface"); 
    let texture = surface.device.create_texture_with_data(
            &surface.queue,
            &wgpu_texture_desc
        , slice); 
    
    TextureResource::to_handle(Arc::new(TextureResource {
        texture
    }))
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_ResourceManager_00024JNI_createTextureResource<'local>(mut env: JNIEnv<'local>, _class: JClass, kernel_ptr: jlong, desc: JObject<'local>) -> jlong {
    let texture_desc = JavaTextureDesc::new(&mut env, desc);
    let wgpu_texture_desc = wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: texture_desc.width as u32,
            height: texture_desc.height as u32,
            depth_or_array_layers: texture_desc.layers as u32 
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: (&texture_desc.dim).into(), 
        format: (&texture_desc.format).into(),
        usage:  wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: None,
        view_formats: &[],
    };
    let kernel = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
    // TODO: this is going to make all this single threaded 
    let surface = kernel.window_surface.lock().expect("failed to lock surface"); 
    let texture = surface.device.create_texture(&wgpu_texture_desc); 
    
    TextureResource::to_handle(Arc::new(TextureResource {
        texture
    }))
}

