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


//fn wgpu_texture_desc<'local, 'ret>(mut env: JNIEnv, obj: &JObject) -> wgpu::TextureDescriptor<'ret> {
//    let width = env.get_field(obj, "width", "I").unwrap().i().unwrap();
//    let height = env.get_field(obj, "height", "I").unwrap().i().unwrap();
//    let layer = env.get_field(obj, "layers", "I").unwrap().i().unwrap();
//    let texture_dim = env.get_field(obj, "dim", "I").unwrap().i().unwrap();
//    let format = env.get_field(obj, "format", "I").unwrap().i().unwrap();
//
//    let texture_format : wgpu::TextureFormat = unsafe { std::mem::transmute::<jint, JavaImageFormat>(format) }.into();
//    let texture_dim : wgpu::TextureDimension = unsafe { std::mem::transmute::<jint, JavaTextureDim> (texture_dim) }.into();
//    
//    wgpu::TextureDescriptor {
//        size: wgpu::Extent3d {
//            width: width as u32,
//            height: height as u32,
//            depth_or_array_layers: layer as u32 
//        },
//        mip_level_count: 1,
//        sample_count: 1,
//        dimension: texture_dim,
//        format: texture_format,
//        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
//        label: None,
//        view_formats: &[],
//    }
//}
//
//
//#[no_mangle]
//pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_drop(
//    _jni: JNIEnv,
//    _class: JClass,
//    ptr: jlong,
//) {
//    TextureResource::drop_handle(ptr);
//}
//
//
//#[no_mangle]
//pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_writeTextureBuffer<
//    'local,
//>(env: JNIEnv<'local>,
//    _class: JClass,
//    kernel_ptr: jlong,
//    texture_ptr: jlong,
//    buffer: JByteBuffer<'local>)  {
//    let kernel_arc = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
//    let texture_arc = TextureResource::from_handle(texture_ptr).expect("texture invalid"); 
//    let buf_size = env
//        .get_direct_buffer_capacity(&buffer)
//        .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
//    let buf: _ = env
//        .get_direct_buffer_address(&buffer)
//        .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
//    let slice = unsafe {std::slice::from_raw_parts(buf, buf_size)};
//    let window_read = kernel_arc.surface.read().unwrap();
//    let window = window_read.as_ref().unwrap();
//    
//    let format = texture_arc.texture.format().bit_size_block() / 8;
//    window.queue.write_texture(
//        texture_arc.texture.as_image_copy(),
//        &slice,
//        wgpu::ImageDataLayout {
//            offset: 0,
//            bytes_per_row: Some(format),
//            rows_per_image: None,
//        },
//        wgpu::Extent3d::default(),
//    );
//}
//
//#[no_mangle]
//pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_createTextureResource<
//    'local,
//>(
//    env: JNIEnv<'local>,
//    _class: JClass,
//    kernel_ptr: jlong,
//    desc: JObject<'local>,
//)  -> jlong {
//    let kernel = EngineKernel::from_handle(kernel_ptr).expect("kernel invalid");
//    let window_read = kernel.surface.read().unwrap();
//    let window = window_read.as_ref().unwrap();
//    let texture_desc = wgpu_texture_desc(env, &desc); 
//    
//    let texture = window.device.create_texture(
//    &wgpu::TextureDescriptor {
//        ..texture_desc
//    });
//    TextureResource::to_handle(Arc::new(TextureResource {
//        texture
//    }))
//}
//#[no_mangle]
//pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_createTextureResourceFromBuffer<
//    'local,
//>(
//    env: JNIEnv<'local>,
//    _class: JClass,
//    kernel_ptr: jlong,
//    desc: JObject<'local>,
//    buffer: JByteBuffer<'local>
//) -> jlong {
//    let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//    //let kernel = kernel_arc.borrow();
//    let buf_size = env
//        .get_direct_buffer_capacity(&buffer)
//        .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
//    let buf: _ = env
//        .get_direct_buffer_address(&buffer)
//        .expect("Unable to get address to direct buffer. Buffer must be allocated direct.");
//    let slice = unsafe {std::slice::from_raw_parts(buf, buf_size)};
//    let window_read = kernel_arc.surface.read().unwrap();
//    let window = window_read.as_ref().unwrap();
//    let texture_desc = wgpu_texture_desc(env, &desc); 
//
//    let texture = window.device.create_texture_with_data(
//        &window.queue,
//        &wgpu::TextureDescriptor {
//            ..texture_desc 
//        }
//    , slice);
//    TextureResource::to_handle(Arc::new(TextureResource {
//        texture
//    }))
//}
//
//#[no_mangle]
//pub extern "system" fn Java_org_terasology_engine_rust_TeraTexture_00024JNI_getSize<
//    'local,
//>(
//    mut env: JNIEnv<'local>,
//    _class: JClass,
//    texture_ptr: jlong,
//    mut vec2_obj: JObject<'local>,
//) {
//    let texture_arc = TextureResource::from_handle(texture_ptr).expect("texture invalid"); 
//    let size = texture_arc.texture.size();
//   // joml_vec2::<f32>(vec2_obj)
//   //     .set(&mut env, size.width as f32, size.height as f32);
//    set_joml_vector2f(env, &mut vec2_obj, size.width as f32, size.height as f32);
//}
