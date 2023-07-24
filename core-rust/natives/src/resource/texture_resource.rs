use std::sync::Arc;
use glam::u32;


use crate::java_util::{arc_dispose_handle, arc_from_handle, arc_to_handle, JavaHandle, set_joml_vector2f};

pub struct TextureResource {
    pub texture: wgpu::Texture,
}

pub trait TextureFormatExt {
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



