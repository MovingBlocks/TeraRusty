use futures::executor::block_on;
use jni::{sys::{JNIEnv, jlong, jint, jfloat}, objects::JClass};
use std::sync::Arc;
use crate::{java_util::{arc_from_handle, arc_to_handle, arc_dispose_handle, JavaHandle}, window_surface::WindowSurface, resources::ResourceManager,  ui::UserInterface} ;
use std::sync::RwLock;
use core::ffi::c_void;
use std::cell::RefCell;

pub struct EngineKernel {
     pub instance: wgpu::Instance,
     pub surface: Option<WindowSurface>,
     pub user_interface: Option<UserInterface>,
     pub resource: ResourceManager,
}

pub type WeakEngineRef = std::sync::Weak<RefCell<EngineKernel>>; 
impl EngineKernel {

    pub fn cmd_prepare(&self) {
       // if let Some(ui) = self.user_interface {
       //     ui.cmd_prepare();
       // }
    }

    pub fn cmd_dispatch(&self) {


    }

    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_drop(_jni: JNIEnv, _class: JClass, ptr: jlong) {
        EngineKernel::drop_handle(ptr);
    }

    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_create(_jni: JNIEnv, _class: JClass) -> jlong  {
        EngineKernel::to_handle(Arc::new(RefCell::new(EngineKernel {
            instance: wgpu::Instance::default(),
            surface: None::<WindowSurface>,
            resource: ResourceManager::new(),
            user_interface: None 
        })))
    }
    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_resizeSurface(_jni: JNIEnv, _class: JClass,
        kernel_ptr: jlong, width: jint, height: jint) {
        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let mut kernel = kernel_arc.borrow_mut();
        let Some(surface) = kernel.surface.as_mut() else {panic!("surface not initialized");};
        surface.resize_surface(width, height);
    }
    
    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_dispatch(_jni: JNIEnv, _class: JClass,
        kernel_ptr: jlong) {
        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let kernel = kernel_arc.borrow_mut();
        let Some(surface) = kernel.surface.as_ref() else {panic!("surface invalid");};
    }

    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceX11(_jni: JNIEnv, _class: JClass,
        kernel_ptr: jlong,
        display_ptr: jlong,
        window_ptr: jlong) {
        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let mut kernel = kernel_arc.borrow_mut();
        kernel.surface = Some(block_on(WindowSurface::create_window_x11(&kernel.instance, 
           display_ptr as *mut c_void, window_ptr as *mut c_void )));
        kernel.initialize_subsystems();
    }
    
    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceWin32(_jni: JNIEnv, _class: JClass,
        kernel_ptr: jlong,
        _display_ptr: jlong,
        window_ptr: jlong) {
        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let mut kernel = kernel_arc.borrow_mut();
        kernel.surface = Some(block_on(WindowSurface::create_window_win32(&kernel.instance, window_ptr as *mut c_void )));
        kernel.initialize_subsystems();
    }

    fn initialize_subsystems(&mut self) {
        //let Some(surface) = self.surface;
        //self.user_interface = Some(UserInterface::new(&surface.device));

    }
    
    // User Interface
    #[no_mangle]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUISetCrop(_jni: JNIEnv, _class: JClass,
        kernel_ptr: jlong, min_x: jfloat, min_y: jfloat, max_x: jfloat, max_y: jfloat ) {
        let Some(kernel_arc) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let mut kernel = kernel_arc.borrow_mut();
        let Some(ui) = kernel.user_interface.as_mut() else {panic!("surface invalid");};
        
        ui.cmd_set_crop(None);    
    }
}

impl JavaHandle<Arc<RefCell<EngineKernel>>> for EngineKernel {
    fn from_handle(ptr: jlong) -> Option<Arc<RefCell<EngineKernel>>> {
        arc_from_handle(ptr)  
    }

    fn to_handle(from: Arc<RefCell<EngineKernel>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<RefCell<EngineKernel>>(ptr); 
    }
}


