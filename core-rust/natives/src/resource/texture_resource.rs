use std::convert::From;
use std::sync::Arc;
use glam::u32;
use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jint;


use crate::java_util::{arc_dispose_handle, arc_from_handle, arc_to_handle, JavaHandle, set_joml_vector2f};

pub struct TextureResource {
    pub texture: wgpu::Texture,
}

trait TextureFormatExt {
    fn bit_size_block(&self) -> u32;
}

impl TextureFormatExt for wgpu::TextureFormat {

    fn bit_size_block(&self) -> u32 {
       //TODO: incomplete
       match self {
            wgpu::TextureFormat::R8Unorm |
                wgpu::TextureFormat::R8Sint |
                wgpu::TextureFormat::R8Snorm => 8,
            wgpu::TextureFormat::Rgba8Unorm |
                wgpu::TextureFormat::Rgba8Sint |
                wgpu::TextureFormat::Rgba8Uint |
                wgpu::TextureFormat::Rgba8UnormSrgb => 32, 
            _ => 0
        } 
    }
}

//impl TextureResource {
//
//    pub fn write_buffer(&mut self, queue: &wgpu::Queue, buf: &[u8]) {
//
//        let format = self.texture.format().bit_size_block() / 8;
//        queue.write_texture(
//            self.texture.as_image_copy(),
//            &buf,
//            wgpu::ImageDataLayout {
//                offset: 0,
//                bytes_per_row: Some(format),
//                rows_per_image: None,
//            },
//            wgpu::Extent3d::default(),
//        );
//    }
//}

impl JavaHandle<Arc<TextureResource>> for TextureResource {
    fn from_handle(ptr: jni::sys::jlong) -> Option<Arc<TextureResource>> {
        arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<TextureResource>) -> jni::sys::jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jni::sys::jlong) {
        arc_dispose_handle::<TextureResource>(ptr);
    }
}



