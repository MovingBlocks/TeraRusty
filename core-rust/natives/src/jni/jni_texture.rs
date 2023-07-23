use jni::{JNIEnv, objects::JObject, sys::jint};

pub struct JavaTextureDesc {
   pub width: u32,
   pub height: u32,
   pub layers: u32,
   pub dim: JavaTextureDim,
   pub format: JavaImageFormat
}

impl JavaTextureDesc {
    pub fn new<'local>(env: &mut JNIEnv<'local>, obj: JObject<'local>) -> Self {
        let width = env.get_field(&obj, "width", "I").unwrap().i().unwrap();
        let height = env.get_field(&obj, "height", "I").unwrap().i().unwrap();
        let layer = env.get_field(&obj, "layers", "I").unwrap().i().unwrap();
        let dim = env.get_field(&obj, "dim", "I").unwrap().i().unwrap();
        let format = env.get_field(&obj, "format", "I").unwrap().i().unwrap();
    
        let texture_format  = unsafe { std::mem::transmute::<jint, JavaImageFormat>(format) };
        let texture_dim  = unsafe { std::mem::transmute::<jint, JavaTextureDim> (dim) };
        Self {
            width: width as u32,
            height: height as u32,
            layers: layer as u32,
            dim: texture_dim,
            format: texture_format
        }
    }
}


#[repr(u32)]
pub enum JavaTextureDim {
    DIM_1D,
    DIM_2D,
    DIM_3D
}

impl From<&JavaTextureDim> for wgpu::TextureDimension {
    fn from(item: &JavaTextureDim) -> Self {
       match item {
           JavaTextureDim::DIM_1D => wgpu::TextureDimension::D1, 
           JavaTextureDim::DIM_2D => wgpu::TextureDimension::D2, 
           JavaTextureDim::DIM_3D => wgpu::TextureDimension::D3, 
        }
    }
}

#[repr(u32)]
pub enum JavaImageFormat {
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


impl From<&JavaImageFormat> for wgpu::TextureFormat {
    fn from(item: &JavaImageFormat) -> Self {
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

