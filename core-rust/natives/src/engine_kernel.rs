use futures::{executor::block_on, TryFutureExt};
use jni::{sys::{JNIEnv, jlong, jint, jfloat}, objects::JClass};
use std::sync::Arc;
use crate::{java_util::{arc_from_handle, arc_to_handle, arc_dispose_handle, JavaHandle}, window_surface::{WindowSurface, WindowSurfaceDesc}, ui::{UserInterface, Rect}, resource::texture_resource::TextureResource} ;
use std::cell::{RefCell, Cell};
use std::sync::Mutex;

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
   pub frame_encoder: RefCell<Option<FrameContext>>
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
           frame_encoder: RefCell::new(None)
        }
    }

    pub fn dispatch_event(&self, event: &EngineEvent) {
        match event {
            EngineEvent::Resize(payload) => {
                let mut surface = self.window_surface.lock().expect("failed to resolve surface");
                //let mut ui = self.user_interface.get_mut(); 
                surface.resize_surface(payload.width, payload.height);
            }
        }
    }

    pub fn cmd_prepare(&self) {
        let surface = self.window_surface.lock().expect("failed to lock surface");
        let mut ui = self.user_interface.borrow_mut();
        ui.cmd_prepare();
        self.frame_encoder.replace(
            Some(FrameContext {
                encoder: surface.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
            })
        );
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
        
        let mut encoder = self.frame_encoder.replace(None).unwrap();
        
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
            &mut encoder.encoder 
        );
         
        window_surface.queue.submit(std::iter::once(encoder.encoder.finish()));
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

//impl EngineKernel {
//
//    pub fn cmd_prepare(&self) {
//        let window = self.surface.read().unwrap();
//        
//        if let Some(ui) = self.user_interface.borrow_mut().as_mut() {
//            ui.cmd_prepare();
//        }
//        self.encoder.replace( 
//            Some(window.as_ref().unwrap().device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })));
//    }
//
//    pub fn cmd_dispatch(&self) {
//        let surface = self.surface.read().unwrap();
//
//        let Some(ref mut window) = surface.as_ref() else { return };
//        let frame = window
//            .surface
//            .get_current_texture()
//            .expect("Failed to acquire next swap chain texture");
//
//        let view = frame
//            .texture
//            .create_view(&wgpu::TextureViewDescriptor::default());
//        
//        let mut encoder = self.encoder.replace(None).unwrap();
//        
//        let mut ui_ref = self.user_interface.borrow_mut();
//        let ui = ui_ref.as_mut().unwrap();
//        let frame_texture = &frame.texture;
//        let size = frame_texture.size();
//        
//        ui.cmd_dispatch(
//            &Rect {
//                min: [0.0,0.0],
//                max: [size.width as f32, size.height as f32],
//            },
//            &view,
//            &window.device,
//            &window.queue,
//            &mut encoder 
//        );
//         
//        window.queue.submit(std::iter::once(encoder.finish()));
//        frame.present();
//    }
//
//   // #[no_mangle]
//   // pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_drop(_jni: JNIEnv, _class: JClass, ptr: jlong) {
//   //     EngineKernel::drop_handle(ptr);
//   // }
//
//   // #[no_mangle]
//   // pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_create(_jni: JNIEnv, _class: JClass) -> jlong  {
//   //     EngineKernel::to_handle(Arc::new(EngineKernel {
//   //         instance: wgpu::Instance::default(),
//   //         surface: RwLock::new(None::<WindowSurface>),
//   //         user_interface: RefCell::new(None),
//   //         encoder: RefCell::new(None)
//   //     }))
//   // }
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_resizeSurface(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong, width: jint, height: jint) {
//        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//
//        let mut surface = kernel_arc.surface.write().expect("failed to resolve surface");
//        let mut ui = kernel_arc.user_interface.borrow_mut(); 
//        
//        let Some(surface) = surface.as_mut() else {panic!("surface not initialized");};
//        surface.resize_surface(width, height);
//        
//        if let Some(ui) = ui.as_mut() {
//            ui.resize_surface(&surface.device, &glam::IVec2::new(width,height));
//        }
//    }
//    
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdDispatch(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong) {
//        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        //let mut kernel = kernel_arc.borrow_mut();
//        kernel_arc.cmd_dispatch();
//    }
//
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdPrepare(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong) {
//        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        //let mut kernel = kernel_arc.borrow_mut();
//        kernel_arc.cmd_prepare();
//    }
//
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceX11(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong,
//        display_ptr: jlong,
//        window_ptr: jlong) {
//        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        {
//            let mut sur = kernel.surface.write().expect("failed to acquire surface");
//            (*sur) = Some(block_on(WindowSurface::create_window_x11(&kernel.instance, 
//               display_ptr as *mut c_void, window_ptr as *mut c_void )));
//        }
//        kernel.initialize_subsystems();
//    }
//    
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceWin32(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong,
//        _display_ptr: jlong,
//        window_ptr: jlong) {
//        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        {
//            let mut surface = kernel.surface.write().expect("failed to acquire surface");
//            (*surface) = Some(block_on(WindowSurface::create_window_win32(&kernel.instance, window_ptr as *mut c_void )));
//        }
//        kernel.initialize_subsystems();
//    }
//
//    fn initialize_subsystems(&self) {
//        let surface_lock = self.surface.read().expect("failed to acquire surface");
//        let surface = surface_lock.as_ref().unwrap();
//        self.user_interface.replace(Some(UserInterface::new(&surface.device, &surface.surface_configuration)));
//    }
//    
//    // User Interface
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUISetCrop(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong, min_x: jfloat, min_y: jfloat, max_x: jfloat, max_y: jfloat ) {
//        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        let mut ui_ref = kernel.user_interface.borrow_mut();
//        let ui = ui_ref.as_mut().unwrap();
//        ui.cmd_set_crop(Some(Rect {
//            min: [min_x, min_y],
//            max: [max_x, max_y]
//        }));
//    }
//
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUIClearCrop(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong) {
//        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        let mut ui_ref = kernel.user_interface.borrow_mut();
//        let ui = ui_ref.as_mut().unwrap();
//        ui.cmd_set_crop(None);
//    }
//
//    #[no_mangle]
//    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUIDrawTexture(_jni: JNIEnv, _class: JClass,
//        kernel_ptr: jlong,
//        tex_ptr: jlong,
//        uv_min_x: jfloat, uv_min_y: jfloat, uv_max_x: jfloat, uv_max_y: jfloat,
//        pos_min_x: jfloat, pos_min_y: jfloat, pos_max_x: jfloat, pos_max_y: jfloat,
//        tint_color: jint) {
//        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
//        let Some(text_resource_arc) = TextureResource::from_handle(tex_ptr) else {panic!("invalid tex resource")};
//       
//        let mut ui_ref = kernel.user_interface.borrow_mut();
//        let ui = ui_ref.as_mut().unwrap();
//        //let ui = kernel.user_interface.borrow_mut().as_mut().unwrap();
//        let window_read = kernel.surface.read().unwrap();
//        let window = window_read.as_ref().unwrap();
//
//
//        ui.cmd_draw_texture(
//            &window.queue,
//            &window.device,
//            &text_resource_arc,
//            &Rect {
//                min: [uv_min_x, uv_min_y],
//                max: [uv_max_x, uv_max_y]
//            },
//            &Rect {
//                min: [pos_min_x, pos_min_y],
//                max: [pos_max_x, pos_max_y]
//            },
//            tint_color as u32
//        );
//
//    }
//}


