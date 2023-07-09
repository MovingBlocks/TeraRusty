use std::sync::RwLock;

pub struct Context {
    pub wgpu_instance: Option<wgpu::Instance>,
    pub wgpu_surface: Option<wgpu::Surface>
}

impl Context {

}

lazy_static! {
    pub static ref CONTEXT: RwLock<Context> = RwLock::new(Context {
        wgpu_instance: None,
        wgpu_surface: None
    });
}
