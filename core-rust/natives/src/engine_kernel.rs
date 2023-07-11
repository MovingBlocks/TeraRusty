use jni::{sys::{JNIEnv, jlong}, objects::JClass};
use std::sync::Arc;
use crate::{java_util::{arc_from_handle, arc_to_handle, arc_dispose_handle, JavaHandle}, window_surface::WindowSurface} ;
use std::sync::RwLock;

pub struct EngineKernel {
    pub instance: wgpu::Instance,
    pub surface: Option<WindowSurface>,
}

impl EngineKernel {
    pub fn renderer(&self) -> &wgpu::Instance {
       &self.instance 
    }
}

impl JavaHandle<Arc<RwLock<EngineKernel>>> for EngineKernel {
    fn from_handle(ptr: jlong) -> Option<Arc<RwLock<EngineKernel>>> {
        arc_from_handle(ptr)  
    }

    fn to_handle(from: Arc<RwLock<EngineKernel>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<RwLock<EngineKernel>>(ptr); 
    }
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_create(_jni: JNIEnv, _class: JClass) -> jlong  {
    EngineKernel::to_handle(Arc::new(RwLock::new(EngineKernel {
        instance: wgpu::Instance::default(),
        surface: None::<WindowSurface>,
    })))
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_drop(_jni: JNIEnv, _class: JClass, ptr: jlong) {
    EngineKernel::drop_handle(ptr);
}


