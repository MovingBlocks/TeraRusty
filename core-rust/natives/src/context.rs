use std::sync::Mutex;
use once_cell::sync::{OnceCell, Lazy};
#[derive(Debug)]
pub struct Renderer {
    pub wgpu_instance: wgpu::Instance,
    pub wgpu_surface: wgpu::Surface,
    pub wgpu_adapter: wgpu::Adapter,
    pub wgpu_device: wgpu::Device,
    pub wgpu_graphics_queue: wgpu::Queue, 
}

impl Renderer {
    pub fn update_surface_size(&self, w: u32, h: u32) {
        let swapchain_capabilities = self.wgpu_surface.get_capabilities(&self.wgpu_adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: w,
            height: h,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        self.wgpu_surface.configure(&self.wgpu_device, &config);
    }
}


pub struct Context {
    pub renderer: OnceCell<Renderer>,
    pub test_pipeline: OnceCell<wgpu::RenderPipeline>, 
    pub swapchain_size: Mutex<glam::UVec2>
}

pub static ROOT: Lazy<Context> = Lazy::new(|| {
    Context {
        renderer: OnceCell::new(),
        test_pipeline: OnceCell::new(),
        swapchain_size: Mutex::new(glam::UVec2::new(0,0))
    }
});

