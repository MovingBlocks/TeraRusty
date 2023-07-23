use futures::executor::block_on;
use jni::sys::jlong;
use std::sync::Arc;
use crate::{java_util::{arc_from_handle, arc_to_handle, arc_dispose_handle, JavaHandle}, window_surface::{WindowSurface, WindowSurfaceDesc}, ui::{UserInterface}, math::rect::Rect} ;
use std::cell::RefCell;
use std::sync::Mutex;
use std::cell::Cell;

pub struct ResizePayload {
    pub width: u32,
    pub height: u32
}

pub enum EngineEvent {
   Resize(ResizePayload)
}

pub struct FrameContext {
    pub encoder: wgpu::CommandEncoder
}

pub struct EngineKernel {
   pub instance: wgpu::Instance,
   pub window_surface: Mutex<WindowSurface>,
    
   pub user_interface: RefCell<UserInterface>,
   pub frame_encoder: Mutex<Cell<Option<FrameContext>>>

}

pub struct EngineKernelDesc {
   pub surface: WindowSurfaceDesc,
}

impl EngineKernel {
    pub fn new(instance: wgpu::Instance, desc: &EngineKernelDesc) -> Self {
        let surface = block_on(WindowSurface::create(&instance, &desc.surface));

        let ui = UserInterface::new(&surface.device, &surface.surface_info());
        Self {
           instance,
           window_surface:  Mutex::new(surface),
           user_interface: RefCell::new(ui),
           frame_encoder: Mutex::new(Cell::new(None))
        }
    }

    pub fn dispatch_event(&self, event: &EngineEvent) {
        match event {
            EngineEvent::Resize(payload) => {
                let mut surface = self.window_surface.lock().expect("failed to resolve surface");
                surface.resize_surface(payload.width, payload.height);
            }
        }
    }

    pub fn cmd_prepare(&self) {
        let surface = self.window_surface.lock().expect("failed to lock surface");
        let mut ui = self.user_interface.borrow_mut();
        ui.cmd_prepare();
        let frame_encoder = self.frame_encoder.lock().expect("Could not lock frame_encoder");
        frame_encoder.set(Some(FrameContext {
                encoder: surface.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
            }));
    }


    pub fn cmd_dispatch(&self) {
        let window_surface = self.window_surface.lock().expect("failed to lock surface");
        let frame = 
            window_surface.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let frame_context_cell: Cell<Option<FrameContext>> = Cell::new(None);
        self.frame_encoder.lock().expect("Could not lock frame_encoder").swap(&frame_context_cell);
        let mut frame_context = frame_context_cell.into_inner().expect("cmd_prepare");

        let mut ui = self.user_interface.borrow_mut();
        let frame_texture = &frame.texture;
        let size = frame_texture.size();
        
        ui.cmd_dispatch(
            &Rect {
                min: [0.0,0.0],
                max: [size.width as f32, size.height as f32],
            },
            &view,
            &window_surface.device,
            &window_surface.queue,
            &mut frame_context.encoder 
        );
         
        window_surface.queue.submit(std::iter::once(frame_context.encoder.finish()));
        frame.present();
    }
}

impl JavaHandle<Arc<EngineKernel>> for EngineKernel {
    fn from_handle(ptr: jlong) -> Option<Arc<EngineKernel>> {
        arc_from_handle(ptr)  
    }

    fn to_handle(from: Arc<EngineKernel>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<EngineKernel>(ptr); 
    }
}
